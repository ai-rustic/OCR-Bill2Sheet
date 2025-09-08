'use client';

import { useState } from 'react';
import { UploadImagesPanel } from '@/components/upload-images-panel';
import { OCRResultsPanel } from '@/components/ocr-results-panel';

interface FileWithPreview {
  file: File;
  preview: string;
  id: string;
}

interface BillMeta {
  bill_number: string;
  seller: string;
  buyer: string;
  seller_tax_code: string;
  buyer_tax_code: string;
  bill_date: string;
  total_amount: string;
  vat_amount: string;
  payment_method: string;
  address: string;
}

interface LineItem {
  no: number;
  product_name: string;
  quantity: string;
  unit: string;
  unit_price: string;
  subtotal: string;
}

interface BillData {
  bill_meta: BillMeta;
  line_items: LineItem[];
  notes: string;
}

interface OCRResult {
  success: boolean;
  bill_data?: BillData;
  message?: string;
  processing_timestamp?: string;
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

      const response = await fetch('http://10.1.4.189:8888/api/ocr-bill', {
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
          
          return { success: true, message: `Successfully processed ${files.length} images! Excel file downloaded.` };
        }
      } else {
        const errorText = await response.text();
        let errorMessage = 'Processing failed';
        
        try {
          const errorJson = JSON.parse(errorText);
          errorMessage = errorJson.error || errorJson.message || errorText;
        } catch {
          errorMessage = errorText || 'Unknown error occurred';
        }
        
        throw new Error(errorMessage);
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
        const result = await handleFileUpload(selectedFiles.map(f => f.file));
        
        // Add delay to show the animation effect
        setTimeout(() => {
          if (result) {
            setOcrResults({
              success: true,
              ...result
            });
          } else {
            setOcrResults({
              success: false,
              error: "No response received from server"
            });
          }
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
        <div className={`transition-all duration-700 ease-in-out ${showResults ? 'lg:flex lg:box-border' : ''}`}>
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
