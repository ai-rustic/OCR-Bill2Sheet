'use client';

import { useState } from 'react';
import { BillUpload } from '@/components/bill-upload';

export default function Home() {
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
        // Get the Excel file from response
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
      } else {
        const error = await response.text();
        alert(`Error: ${error}`);
      }
    } catch (error) {
      console.error('Upload error:', error);
      alert('Failed to process the bill images. Please try again.');
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-4xl mx-auto">
        <BillUpload onUpload={handleFileUpload} isUploading={isUploading} />
      </div>
    </div>
  );
}
