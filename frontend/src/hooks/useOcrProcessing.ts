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
  // Check if data has known field patterns from backend SSE events
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

/**
 * Helper function to check if event type indicates completion
 */
function isCompletionEvent(eventType: string): boolean {
  return ['processing_complete', 'processing_error'].includes(eventType);
}

/**
 * Custom hook for processing images via OCR API with SSE
 */
export function useOcrProcessing(): UseOcrProcessingReturn {
  const [isProcessing, setIsProcessing] = useState(false);
  const [events, setEvents] = useState<OcrEvent[]>([]);
  const eventSourceRef = useRef<EventSource | null>(null);

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

    try {
      // First, upload the files to get the session started
      const formData = new FormData();
      files.forEach(file => {
        formData.append('images', file);
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

      try {
        while (true) {
          const { done, value } = await reader.read();

          if (done) break;

          buffer += decoder.decode(value, { stream: true });

          // Process complete SSE messages
          const lines = buffer.split('\n');
          buffer = lines.pop() || ''; // Keep incomplete line in buffer

          let currentEventType = '';

          for (const line of lines) {
            if (line.trim() === '') continue; // Skip empty lines

            if (line.startsWith('event: ')) {
              currentEventType = line.substring(7).trim();
              continue;
            }

            if (line.startsWith('data: ')) {
              const eventData = line.substring(6).trim();

              try {
                const rawData = JSON.parse(eventData) as unknown;

                if (!isRecord(rawData)) {
                  continue;
                }

                const parsedData = rawData;
                const parsedType =
                  typeof parsedData.type === 'string' ? parsedData.type : null;
                const eventType =
                  currentEventType ||
                  parsedType ||
                  getEventTypeFromData(parsedData) ||
                  'unknown';

                const event: OcrEvent = {
                  type: eventType,
                  data: parsedData
                };

                setEvents(prev => [...prev, event]);

                // Check if processing is complete
                if (isCompletionEvent(event.type)) {
                  setIsProcessing(false);
                  return; // Exit the function
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
