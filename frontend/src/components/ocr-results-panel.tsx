'use client';

import { Loader2 } from 'lucide-react';

interface OCRResult {
  success: boolean;
  data?: Record<string, unknown>;
  error?: string;
}

interface OCRResultsPanelProps {
  showResults: boolean;
  isUploading: boolean;
  ocrResults: OCRResult | null;
}

export function OCRResultsPanel({ showResults, isUploading, ocrResults }: OCRResultsPanelProps) {
  if (!showResults) {
    return null;
  }

  return (
    <div className="space-y-4 animate-slide-in-right lg:w-1/2 lg:flex-shrink-0 lg:overflow-hidden lg:px-2 lg:box-border">
      <div className="text-center space-y-2">
        <h2 className="text-2xl font-bold text-gray-900">OCR Results</h2>
        <p className="text-gray-600">Extracted data from your bill images</p>
      </div>

      <div className="border border-gray-200 rounded-lg p-6 space-y-4 shadow-lg backdrop-blur-sm bg-white/95 max-h-[70vh] overflow-y-auto">
        {isUploading ? (
          <div className="flex flex-col items-center justify-center py-8 space-y-4">
            <div className="relative">
              <Loader2 size={40} className="animate-spin text-blue-600" />
              <div className="absolute inset-0 animate-ping">
                <div className="w-10 h-10 border-2 border-blue-300 rounded-full opacity-30"></div>
              </div>
            </div>
            <div className="text-center">
              <p className="font-medium text-gray-900">Processing your bill images...</p>
              <p className="text-sm text-gray-500">This may take a few moments</p>
            </div>
          </div>
        ) : ocrResults ? (
          <div className="space-y-4 animate-scale-in">
            {ocrResults.success ? (
              <div className="space-y-4">
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                  <span className="text-sm font-medium text-green-700">Processing completed successfully</span>
                </div>
                <div className="bg-gradient-to-br from-green-50 to-blue-50 rounded-lg p-4 border border-green-200">
                  <h3 className="font-medium text-gray-900 mb-2">Extracted Information:</h3>
                  <div className="bg-white/80 rounded p-3 border">
                    <pre className="text-sm text-gray-700 whitespace-pre-wrap">{JSON.stringify(ocrResults.data, null, 2)}</pre>
                  </div>
                </div>
              </div>
            ) : (
              <div className="space-y-4">
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
                  <span className="text-sm font-medium text-red-700">Processing failed</span>
                </div>
                <div className="bg-red-50 rounded-lg p-4 border border-red-200">
                  <p className="text-sm text-red-700">{ocrResults.error}</p>
                </div>
              </div>
            )}
          </div>
        ) : (
          <div className="text-center py-8">
            <p className="text-gray-500">Click &quot;Process Images&quot; to see results here</p>
          </div>
        )}
      </div>
    </div>
  );
}