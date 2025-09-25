"use client"

import * as React from "react"
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Progress } from "@/components/ui/progress"
import { CheckCircle, XCircle, Clock, AlertCircle, FileText, Database } from "lucide-react"
import { cn } from "@/lib/utils"
import { OcrEvent } from "@/hooks/useOcrProcessing"

interface ProcessingStatusProps {
  events: OcrEvent[];
  isProcessing: boolean;
  className?: string;
}

const getString = (data: OcrEvent['data'], key: string): string | undefined => {
  const value = data[key];
  return typeof value === 'string' ? value : undefined;
}

const getNumber = (data: OcrEvent['data'], key: string): number | undefined => {
  const value = data[key];
  return typeof value === 'number' ? value : undefined;
}

const getRecord = (data: OcrEvent['data'], key: string): OcrEvent['data'] | undefined => {
  const value = data[key];
  return typeof value === 'object' && value !== null ? (value as OcrEvent['data']) : undefined;
}

/**
 * Component to display OCR processing status and events
 */
export function ProcessingStatus({ events, isProcessing, className }: ProcessingStatusProps) {
  if (events.length === 0 && !isProcessing) {
    return null;
  }

  const getEventIcon = (eventType: string) => {
    switch (eventType) {
      case 'upload_started':
      case 'image_received':
      case 'image_validation_start':
        return <Clock className="h-4 w-4 text-blue-500" />;
      case 'image_validation_success':
      case 'gemini_processing_success':
      case 'bill_data_saved':
      case 'processing_complete':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'image_validation_error':
      case 'gemini_processing_error':
      case 'processing_error':
        return <XCircle className="h-4 w-4 text-red-500" />;
      case 'gemini_processing_start':
        return <FileText className="h-4 w-4 text-purple-500" />;
      case 'all_images_validated':
        return <CheckCircle className="h-4 w-4 text-blue-500" />;
      default:
        return <AlertCircle className="h-4 w-4 text-gray-500" />;
    }
  };

  const getEventColor = (eventType: string) => {
    switch (eventType) {
      case 'processing_complete':
      case 'image_validation_success':
      case 'gemini_processing_success':
      case 'bill_data_saved':
        return 'bg-green-50 border-green-200';
      case 'processing_error':
      case 'image_validation_error':
      case 'gemini_processing_error':
        return 'bg-red-50 border-red-200';
      case 'gemini_processing_start':
        return 'bg-purple-50 border-purple-200';
      default:
        return 'bg-blue-50 border-blue-200';
    }
  };

  const formatEventMessage = (event: OcrEvent) => {
    const { type, data } = event;
    const fileIndex = getNumber(data, 'file_index');
    const fileLabel = getString(data, 'file_name') ?? (typeof fileIndex === 'number' ? `File ${fileIndex + 1}` : 'File');

    switch (type) {
      case 'upload_started': {
        const totalFiles = getNumber(data, 'total_files') ?? 0
        return `Started processing ${totalFiles} files`
      }
      case 'image_received': {
        const sizeBytes = getNumber(data, 'size_bytes')
        const sizeText = typeof sizeBytes === 'number' ? `${Math.round(sizeBytes / 1024)}KB` : 'Unknown size'
        return `Received: ${fileLabel} (${sizeText})`
      }
      case 'image_validation_start':
        return `Validating: ${fileLabel}`
      case 'image_validation_success': {
        const fileInfo = getRecord(data, 'file_info')
        const infoName = fileInfo ? getString(fileInfo, 'file_name') : undefined
        return `Validated: ${infoName ?? fileLabel}`
      }
      case 'image_validation_error': {
        const errorMessage = getString(data, 'error_message') ?? 'Validation error'
        return `Validation failed: ${errorMessage}`
      }
      case 'all_images_validated': {
        const successfulCount = getNumber(data, 'successful_count') ?? 0
        const totalProcessed = getNumber(data, 'total_processed') ?? 0
        return `All images validated: ${successfulCount}/${totalProcessed} successful`
      }
      case 'gemini_processing_start':
        return `Starting AI analysis: ${fileLabel}`
      case 'gemini_processing_success':
        return `AI analysis completed: ${fileLabel}`
      case 'gemini_processing_error': {
        const errorMessage = getString(data, 'error_message') ?? 'Unknown AI error'
        return `AI analysis failed: ${errorMessage}`
      }
      case 'bill_data_saved': {
        const billIdString = getString(data, 'bill_id')
        const billIdNumber = getNumber(data, 'bill_id')
        const billId = billIdString ?? (typeof billIdNumber === 'number' ? billIdNumber.toString() : 'unknown')
        return `Bill data saved (ID: ${billId})`
      }
      case 'processing_complete': {
        const successfulFiles = getNumber(data, 'successful_files') ?? 0
        const totalFiles = getNumber(data, 'total_files') ?? 0
        const duration = getNumber(data, 'duration_ms')
        const durationText = typeof duration === 'number' ? `${duration}ms` : 'unknown duration'
        return `Processing complete! ${successfulFiles}/${totalFiles} files processed successfully in ${durationText}`
      }
      case 'processing_error': {
        const errorMessage = getString(data, 'error_message') ?? 'Unknown error'
        return `Processing failed: ${errorMessage}`
      }
      default:
        return JSON.stringify(data)
    }
  };

  const completedEvents = events.filter(e =>
    ['processing_complete', 'processing_error'].includes(e.type)
  ).length;

  const uploadStartedEvent = events.find(e => e.type === 'upload_started');
  const totalFiles = uploadStartedEvent ? getNumber(uploadStartedEvent.data, 'total_files') ?? 0 : 0;
  const processedFiles = events.filter(e =>
    ['image_validation_success', 'image_validation_error'].includes(e.type)
  ).length;

  const progress = totalFiles > 0 ? (processedFiles / totalFiles) * 100 : 0;

  return (
    <Card className={cn("w-full", className)}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="text-lg">Processing Status</CardTitle>
          {isProcessing ? (
            <Badge variant="secondary" className="bg-blue-100 text-blue-800">
              Processing...
            </Badge>
          ) : completedEvents > 0 ? (
            <Badge variant="secondary" className="bg-green-100 text-green-800">
              Completed
            </Badge>
          ) : null}
        </div>

        {isProcessing && totalFiles > 0 && (
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span>Progress: {processedFiles}/{totalFiles} files</span>
              <span>{Math.round(progress)}%</span>
            </div>
            <Progress value={progress} className="h-2" />
          </div>
        )}
      </CardHeader>

      <CardContent className="pt-0">
        <div className="space-y-2 max-h-64 overflow-y-auto">
          {events.map((event, index) => {
            const timestamp = getString(event.data, 'timestamp');

            return (
              <div
                key={index}
                className={cn(
                  "flex items-start gap-3 p-3 rounded-lg border text-sm",
                  getEventColor(event.type)
                )}
              >
                <div className="flex-shrink-0 mt-0.5">
                  {getEventIcon(event.type)}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="font-medium text-gray-900 mb-1">
                    {event.type.replace(/([A-Z])/g, ' $1').trim()}
                  </div>
                  <div className="text-gray-700 break-words">
                    {formatEventMessage(event)}
                  </div>
                  {timestamp && (
                    <div className="text-xs text-gray-500 mt-1">
                      {new Date(timestamp).toLocaleTimeString()}
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </CardContent>
    </Card>
  );
}
