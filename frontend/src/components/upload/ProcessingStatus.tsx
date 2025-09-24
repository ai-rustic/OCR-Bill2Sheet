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

    switch (type) {
      case 'upload_started':
        return `Started processing ${data.total_files} files`;
      case 'image_received':
        return `Received: ${data.file_name || `File ${data.file_index + 1}`} (${Math.round(data.size_bytes / 1024)}KB)`;
      case 'image_validation_start':
        return `Validating: ${data.file_name || `File ${data.file_index + 1}`}`;
      case 'image_validation_success':
        return `✓ Validated: ${data.file_info?.file_name || `File ${data.file_index + 1}`}`;
      case 'image_validation_error':
        return `✗ Validation failed: ${data.error_message}`;
      case 'all_images_validated':
        return `All images validated: ${data.successful_count}/${data.total_processed} successful`;
      case 'gemini_processing_start':
        return `🤖 Starting AI analysis: ${data.file_name || `File ${data.file_index + 1}`}`;
      case 'gemini_processing_success':
        return `✓ AI analysis completed: ${data.file_name || `File ${data.file_index + 1}`}`;
      case 'gemini_processing_error':
        return `✗ AI analysis failed: ${data.error_message}`;
      case 'bill_data_saved':
        return `💾 Bill data saved (ID: ${data.bill_id})`;
      case 'processing_complete':
        return `✅ Processing complete! ${data.successful_files}/${data.total_files} files processed successfully in ${data.duration_ms}ms`;
      case 'processing_error':
        return `❌ Processing failed: ${data.error_message}`;
      default:
        return JSON.stringify(data);
    }
  };

  const completedEvents = events.filter(e =>
    ['processing_complete', 'processing_error'].includes(e.type)
  ).length;

  const totalFiles = events.find(e => e.type === 'upload_started')?.data?.total_files || 0;
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
          {events.map((event, index) => (
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
                {event.data.timestamp && (
                  <div className="text-xs text-gray-500 mt-1">
                    {new Date(event.data.timestamp).toLocaleTimeString()}
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}