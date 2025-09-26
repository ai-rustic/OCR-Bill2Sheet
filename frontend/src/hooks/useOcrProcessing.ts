import { useState, useCallback, useRef, useEffect } from 'react';
import { API_ENDPOINTS } from '@/config/api';
type OcrEventData = Record<string, unknown>;

const isRecord = (value: unknown): value is OcrEventData =>
  typeof value === 'object' && value !== null;


export interface OcrEvent {
  type: string;
  data: OcrEventData;
}

export interface UseOcrProcessingReturn {
  isProcessing: boolean;
  events: OcrEvent[];
  processImages: (files: File[]) => Promise<void>;
  clearEvents: () => void;
}

/**
 * Helper function to extract event type from data structure
 */
function getEventTypeFromData(data: OcrEventData): string | null {
  const hasImageIndex = typeof data['image_index'] === 'number';

  if (typeof data['processed'] === 'number') return 'finished';
  if (hasImageIndex && Array.isArray(data['items'])) return 'image_completed';
  if (hasImageIndex && typeof data['message'] === 'string') return 'image_failed';
  if (hasImageIndex && typeof data['filename'] === 'string') return 'image_processing';

  // Existing Axum backend events (legacy support)
  if ('session_id' in data && 'total_files' in data) return 'upload_started';
  if ('file_index' in data && 'file_name' in data && 'size_bytes' in data) return 'image_received';
  if ('file_index' in data && 'file_info' in data) return 'image_validation_success';
  if ('file_index' in data && 'error_code' in data) return 'image_validation_error';
  if ('total_processed' in data) return 'all_images_validated';
  if ('extracted_data' in data) return 'gemini_processing_success';
  if ('bill_id' in data) return 'bill_data_saved';
  if ('successful_files' in data && 'duration_ms' in data) return 'processing_complete';
  if ('error_type' in data) return 'processing_error';
  return null;
}

function normalizeEventData(data: OcrEventData, totalFiles: number): OcrEventData {
  const normalized: OcrEventData = { ...data };

  const imageIndex = normalized['image_index'];
  if (typeof imageIndex === 'number' && typeof normalized['file_index'] !== 'number') {
    normalized['file_index'] = imageIndex;
  }

  const filename = normalized['filename'];
  if (typeof filename === 'string' && typeof normalized['file_name'] !== 'string') {
    normalized['file_name'] = filename;
  }

  const message = normalized['message'];
  if (typeof message === 'string' && typeof normalized['error_message'] !== 'string') {
    normalized['error_message'] = message;
  }

  const processed = normalized['processed'];
  if (typeof processed === 'number' && typeof normalized['total_processed'] !== 'number') {
    normalized['total_processed'] = processed;
  }

  if (typeof normalized['total_files'] !== 'number' && totalFiles > 0) {
    normalized['total_files'] = totalFiles;
  }

  if (typeof normalized['timestamp'] !== 'string') {
    normalized['timestamp'] = new Date().toISOString();
  }

  return normalized;
}

/**
 * Helper function to check if event type indicates completion
 */
function isCompletionEvent(eventType: string): boolean {
  return ['processing_complete', 'processing_error', 'finished'].includes(eventType);
}

/**
 * Custom hook for processing images via OCR API with SSE
 */
export function useOcrProcessing(): UseOcrProcessingReturn {
  const [isProcessing, setIsProcessing] = useState(false);
  const [events, setEvents] = useState<OcrEvent[]>([]);
  const eventSourceRef = useRef<EventSource | null>(null);
  const totalFilesRef = useRef(0);

  const clearEvents = useCallback(() => {
    setEvents([]);
  }, []);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (eventSourceRef.current) {
        eventSourceRef.current.close();
        eventSourceRef.current = null;
      }
    };
  }, []);

  const processImages = useCallback(async (files: File[]) => {
    if (files.length === 0) return;

    setIsProcessing(true);
    clearEvents();

    totalFilesRef.current = files.length;
    const startTimestamp = new Date().toISOString();
    setEvents(() => [
      {
        type: 'upload_started',
        data: {
          total_files: totalFilesRef.current,
          timestamp: startTimestamp,
        },
      },
    ]);

    try {
      // First, upload the files to get the session started
      const formData = new FormData();
      files.forEach(file => {
        formData.append('files', file);
      });

      // Close any existing EventSource
      if (eventSourceRef.current) {
        eventSourceRef.current.close();
        eventSourceRef.current = null;
      }

      // Start the upload which will return SSE stream
      const response = await fetch(API_ENDPOINTS.OCR, {
        method: 'POST',
        body: formData,
        headers: {
          'Accept': 'text/event-stream',
          'Cache-Control': 'no-cache',
        },
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      // Check if response is SSE
      const contentType = response.headers.get('content-type');
      if (!contentType?.includes('text/event-stream')) {
        throw new Error('Expected Server-Sent Events response');
      }

      // Handle SSE stream using Response.body reader
      const reader = response.body?.getReader();
      if (!reader) {
        throw new Error('No response body reader available');
      }

      const decoder = new TextDecoder();
      let buffer = '';
      let shouldStop = false;

      try {
        while (!shouldStop) {
          const { done, value } = await reader.read();

          if (done) {
            break;
          }

          buffer += decoder.decode(value, { stream: true });

          // Process complete SSE messages
          const lines = buffer.split('\n');
          buffer = lines.pop() ?? ''; // Keep incomplete line in buffer

          let currentEventType = '';

          for (const rawLine of lines) {
            const sanitizedLine = rawLine.trimEnd();
            if (sanitizedLine.trim() === '') continue; // Skip empty lines

            if (sanitizedLine.startsWith('event:')) {
              currentEventType = sanitizedLine.substring(6).trim();
              continue;
            }

            if (sanitizedLine.startsWith('data:')) {
              const eventData = sanitizedLine.substring(5).trim();

              try {
                const rawData = JSON.parse(eventData) as unknown;

                if (!isRecord(rawData)) {
                  continue;
                }

                const normalizedData = normalizeEventData(rawData, totalFilesRef.current);
                const parsedType =
                  typeof normalizedData['type'] === 'string'
                    ? String(normalizedData['type'])
                    : null;
                const eventType =
                  currentEventType ||
                  parsedType ||
                  getEventTypeFromData(normalizedData) ||
                  'unknown';

                const event: OcrEvent = {
                  type: eventType,
                  data: normalizedData,
                };

                setEvents(prev => [...prev, event]);

                if (isCompletionEvent(event.type)) {
                  shouldStop = true;
                  break;
                }
              } catch (e) {
                console.warn('Failed to parse SSE data:', eventData, e);
              }
            }
          }
        }
      } finally {
        reader.releaseLock();
      }

      setIsProcessing(false);
      totalFilesRef.current = 0;

    } catch (error) {
      console.error('OCR processing error:', error);
      setEvents(prev => [...prev, {
        type: 'processing_error',
        data: {
          error_message: error instanceof Error ? error.message : 'Unknown error occurred',
          timestamp: new Date().toISOString()
        }
      }]);
      setIsProcessing(false);
    }
  }, [clearEvents]);

  return {
    isProcessing,
    events,
    processImages,
    clearEvents
  };
}
