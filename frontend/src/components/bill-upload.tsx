'use client';

import { useState } from 'react';
import { Dropzone, DropzoneEmptyState } from '@/components/ui/dropzone';
import { Button } from '@/components/ui/button';
import { Download, Loader2, X, ArrowUp, ArrowDown, Plus } from 'lucide-react';
import Image from 'next/image';

interface BillUploadProps {
  onUpload?: (files: File[]) => void;
  isUploading?: boolean;
}

interface FileWithPreview {
  file: File;
  preview: string;
  id: string;
}

export function BillUpload({ onUpload, isUploading = false }: BillUploadProps) {
  const [selectedFiles, setSelectedFiles] = useState<FileWithPreview[]>([]);

  const handleFileDrop = (files: File[]) => {
    const newFiles = files.map(file => {
      const reader = new FileReader();
      const id = Math.random().toString(36).substring(7);
      
      return new Promise<FileWithPreview>((resolve) => {
        reader.onload = () => {
          resolve({
            file,
            preview: reader.result as string,
            id
          });
        };
        reader.readAsDataURL(file);
      });
    });

    Promise.all(newFiles).then(filesWithPreviews => {
      setSelectedFiles(prev => [...prev, ...filesWithPreviews]);
    });
  };

  const handleRemoveFile = (id: string) => {
    setSelectedFiles(prev => prev.filter(f => f.id !== id));
  };

  const handleRemoveAllFiles = () => {
    setSelectedFiles([]);
  };

  const handleMoveUp = (id: string) => {
    setSelectedFiles(prev => {
      const index = prev.findIndex(f => f.id === id);
      if (index > 0) {
        const newFiles = [...prev];
        [newFiles[index - 1], newFiles[index]] = [newFiles[index], newFiles[index - 1]];
        return newFiles;
      }
      return prev;
    });
  };

  const handleMoveDown = (id: string) => {
    setSelectedFiles(prev => {
      const index = prev.findIndex(f => f.id === id);
      if (index < prev.length - 1) {
        const newFiles = [...prev];
        [newFiles[index], newFiles[index + 1]] = [newFiles[index + 1], newFiles[index]];
        return newFiles;
      }
      return prev;
    });
  };

  const handleProcessOCR = () => {
    if (selectedFiles.length > 0 && onUpload) {
      onUpload(selectedFiles.map(f => f.file));
    }
  };

  return (
    <div className="w-full max-w-2xl mx-auto space-y-4">
      <div className="text-center space-y-2">
        <h2 className="text-2xl font-bold text-gray-900">Bill OCR Processor</h2>
        <p className="text-gray-600">Upload multiple images of your bill (for long receipts) to extract data and export to Excel</p>
        
        {/* Instructions for multiple image ordering */}
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 mt-4">
          <div className="flex items-start space-x-3">
            <div className="flex-shrink-0">
              <svg className="w-5 h-5 text-blue-600 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
              </svg>
            </div>
            <div className="text-left">
              <h4 className="text-sm font-semibold text-blue-800 mb-2">
                📋 Bill Image Ordering Guide
              </h4>
              <div className="text-sm text-blue-700 space-y-1">
                <p><strong>• When uploading multiple images:</strong> Please arrange images in order from top to bottom of the bill</p>
                <p><strong>• First image (#1):</strong> Bill header (store information, logo, receipt top)</p>
                <p><strong>• Following images (#2, #3...):</strong> Product sections in the order they appear on the bill</p>
                <p><strong>• Last image:</strong> Bill footer (subtotal, tax, total amount, payment info)</p>
                <p><strong>• Use ↑↓ buttons:</strong> To reorder images if needed for accurate processing</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="space-y-4">
        {/* Upload dropzone - always visible */}
        <Dropzone
          onDrop={handleFileDrop}
          accept={{
            'image/*': ['.png', '.jpg', '.jpeg', '.gif', '.bmp', '.webp']
          }}
          maxFiles={20} // Allow up to 20 images for long bills
          maxSize={10 * 1024 * 1024} // 10MB per file
          className="min-h-32"
          disabled={isUploading}
        >
          <DropzoneEmptyState>
            <div className="flex flex-col items-center justify-center space-y-3">
              <div className="flex size-10 items-center justify-center rounded-md bg-blue-100 text-blue-600">
                <Plus size={20} />
              </div>
              <div className="text-center space-y-1">
                <p className="font-medium text-gray-900">
                  {selectedFiles.length === 0 ? 'Upload bill images' : 'Add more images'}
                </p>
                <p className="text-sm text-gray-500">
                  Drag and drop or click to select multiple images
                </p>
                <p className="text-xs text-gray-400">
                  Supports PNG, JPG, JPEG, GIF, BMP, WEBP (max 10MB each)
                </p>
              </div>
            </div>
          </DropzoneEmptyState>
        </Dropzone>

        {/* Selected files preview */}
        {selectedFiles.length > 0 && (
          <div className="border border-gray-200 rounded-lg p-4 space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="font-medium text-gray-900">
                Bill Images ({selectedFiles.length} {selectedFiles.length === 1 ? 'image' : 'images'})
              </h3>
              <Button
                variant="outline"
                size="sm"
                onClick={handleRemoveAllFiles}
                disabled={isUploading}
              >
                <X size={16} className="mr-2" />
                Remove All
              </Button>
            </div>

            {/* Images grid */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              {selectedFiles.map((fileWithPreview, index) => (
                <div key={fileWithPreview.id} className="border border-gray-200 rounded-lg p-3 space-y-3">
                  {/* File header with order info */}
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-2">
                      <span className="bg-blue-100 text-blue-800 text-xs font-semibold px-2 py-1 rounded">
                        #{index + 1}
                      </span>
                      <div>
                        <p className="font-medium text-sm text-gray-900 truncate max-w-32">
                          {fileWithPreview.file.name}
                        </p>
                        <p className="text-xs text-gray-500">
                          {(fileWithPreview.file.size / 1024 / 1024).toFixed(2)} MB
                        </p>
                      </div>
                    </div>
                    
                    <div className="flex space-x-1">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleMoveUp(fileWithPreview.id)}
                        disabled={isUploading || index === 0}
                        className="h-6 w-6 p-0"
                      >
                        <ArrowUp size={12} />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleMoveDown(fileWithPreview.id)}
                        disabled={isUploading || index === selectedFiles.length - 1}
                        className="h-6 w-6 p-0"
                      >
                        <ArrowDown size={12} />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleRemoveFile(fileWithPreview.id)}
                        disabled={isUploading}
                        className="h-6 w-6 p-0"
                      >
                        <X size={12} />
                      </Button>
                    </div>
                  </div>

                  {/* Image preview */}
                  <div className="border border-gray-200 rounded overflow-hidden">
                    <Image
                      src={fileWithPreview.preview}
                      alt={`Bill page ${index + 1}`}
                      width={300}
                      height={200}
                      className="w-full h-32 object-cover bg-gray-50"
                    />
                  </div>
                </div>
              ))}
            </div>

            {/* Process button */}
            <div className="pt-4 border-t">
              <Button
                onClick={handleProcessOCR}
                disabled={isUploading || selectedFiles.length === 0}
                className="w-full"
                size="lg"
              >
                {isUploading ? (
                  <>
                    <Loader2 size={20} className="animate-spin mr-2" />
                    Processing {selectedFiles.length} images...
                  </>
                ) : (
                  <>
                    <Download size={20} className="mr-2" />
                    Process {selectedFiles.length} images & Download Excel
                  </>
                )}
              </Button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}