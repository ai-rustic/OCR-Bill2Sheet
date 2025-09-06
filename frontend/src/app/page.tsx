'use client';

import { useState } from 'react';
import { UploadImagesPanel } from '@/components/upload-images-panel';
import { OCRResultsPanel } from '@/components/ocr-results-panel';

interface FileWithPreview {
  file: File;
  preview: string;
  id: string;
}

interface OCRResult {
  success: boolean;
  data?: Record<string, unknown>;
  error?: string;
}

export default function Home() {
  const [selectedFiles, setSelectedFiles] = useState<FileWithPreview[]>([]);
  const [showResults, setShowResults] = useState(false);
  const [ocrResults, setOcrResults] = useState<OCRResult | null>(null);
  const [isUploading, setIsUploading] = useState(false);

  const handleFileUpload = async (files: File[]) => {
    if (files.length === 0) return;
    
    setIsUploading(true);
    
    try {
      const formData = new FormData();
      
      // Add all files to form data
      files.forEach((file) => {
        formData.append('bill_images', file);
      });
      
      // Add total count for backend reference
      formData.append('image_count', files.length.toString());

      const response = await fetch('http://localhost:2011/api/ocr-bill', {
        method: 'POST',
        body: formData,
      });

      if (response.ok) {
        // Check if response is JSON (OCR result) or blob (Excel file)
        const contentType = response.headers.get('content-type');
        
        if (contentType && contentType.includes('application/json')) {
          // Handle JSON response (OCR results)
          const result = await response.json();
          return result;
        } else {
          // Handle blob response (Excel file download)
          const blob = await response.blob();
          const url = window.URL.createObjectURL(blob);
          
          // Create download link
          const a = document.createElement('a');
          a.href = url;
          a.download = `bill-data-${Date.now()}.xlsx`;
          document.body.appendChild(a);
          a.click();
          
          // Cleanup
          window.URL.revokeObjectURL(url);
          document.body.removeChild(a);
          
          alert(`Successfully processed ${files.length} images! Excel file downloaded.`);
        }
      } else {
        const error = await response.text();
        throw new Error(error);
      }
    } catch (error) {
      console.error('Upload error:', error);
      throw error;
    } finally {
      setIsUploading(false);
    }
  };

  const handleProcessOCR = async () => {
    if (selectedFiles.length > 0) {
      // Start animation by showing results panel first
      setTimeout(() => {
        setShowResults(true);
      }, 100);
      
      setOcrResults(null);
      
      try {
        // Call the upload handler and wait for result
        await handleFileUpload(selectedFiles.map(f => f.file));
        
        // Add delay to show the animation effect
        setTimeout(() => {
          setOcrResults({
            success: true,
            data: {
              message: "OCR processing completed successfully",
              images_processed: selectedFiles.length,
              extracted_data: "Sample extracted data will appear here..."
            }
          });
        }, 500);
      } catch (error) {
        setTimeout(() => {
          setOcrResults({
            success: false,
            error: error instanceof Error ? error.message : "Processing failed"
          });
        }, 500);
      }
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className={`w-full transition-all duration-700 ease-in-out ${showResults ? 'max-w-full' : 'max-w-2xl mx-auto'}`}>
        <div className={`transition-all duration-700 ease-in-out ${showResults ? 'lg:flex lg:h-screen lg:box-border' : ''}`}>
          <UploadImagesPanel
            selectedFiles={selectedFiles}
            onFilesChange={setSelectedFiles}
            onProcessOCR={handleProcessOCR}
            isUploading={isUploading}
            showResults={showResults}
          />
          <OCRResultsPanel
            showResults={showResults}
            isUploading={isUploading}
            ocrResults={ocrResults}
          />
        </div>
      </div>
    </div>
  );
}
