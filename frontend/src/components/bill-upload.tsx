'use client';

import { useState } from 'react';
import { Dropzone, DropzoneEmptyState } from '@/components/ui/dropzone';
import { Button } from '@/components/ui/button';
import { Banner, BannerIcon, BannerTitle } from '@/components/ui/banner';
import { Pill } from '@/components/ui/pill';
import { Download, Loader2, X, ArrowUp, ArrowDown, Plus, Info } from 'lucide-react';
import Image from 'next/image';

interface BillUploadProps {
  onUpload?: (files: File[]) => void;
  isUploading?: boolean;
}

interface OCRResult {
  success: boolean;
  data?: Record<string, unknown>;
  error?: string;
}

interface FileWithPreview {
  file: File;
  preview: string;
  id: string;
}

export function BillUpload({ onUpload, isUploading = false }: BillUploadProps) {
  const [selectedFiles, setSelectedFiles] = useState<FileWithPreview[]>([]);
  const [showResults, setShowResults] = useState(false);
  const [ocrResults, setOcrResults] = useState<OCRResult | null>(null);

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

  const handleProcessOCR = async () => {
    if (selectedFiles.length > 0 && onUpload) {
      // Start animation by showing results panel first
      setTimeout(() => {
        setShowResults(true);
      }, 100);
      
      setOcrResults(null);
      
      try {
        // Call the parent's upload handler and wait for result
        await onUpload(selectedFiles.map(f => f.file));
        
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
    <div className={`w-full transition-all duration-700 ease-in-out ${showResults ? 'max-w-full' : 'max-w-2xl mx-auto'}`}>
      <div className={`transition-all duration-700 ease-in-out ${showResults ? 'lg:flex lg:h-screen lg:box-border' : ''}`}>
        {/* Upload Panel */}
        <div className={`space-y-4 ${showResults ? 'lg:w-1/2 lg:flex-shrink-0 lg:overflow-hidden lg:px-2 lg:box-border' : 'w-full max-w-2xl mx-auto'}`}>
          <div className="text-center space-y-2">
            <h2 className="text-2xl font-bold text-gray-900">Bill OCR Processor</h2>
            <p className="text-gray-600">Upload multiple images of your bill (for long receipts) to extract data and export to Excel</p>
            
            {/* Instructions for multiple image ordering */}
            <Banner className="bg-blue-50 border border-blue-200 text-blue-900" inset>
              <BannerIcon icon={Info} />
              <BannerTitle className="text-left">
                <div className="space-y-2">
                  <div className="text-sm font-semibold mb-2">
                    📋 Bill Image Ordering Guide
                  </div>
                  <div className="text-sm space-y-1">
                    <div><strong>• When uploading multiple images:</strong> Please arrange images in order from top to bottom of the bill</div>
                    <div><strong>• First image (#1):</strong> Bill header (store information, logo, receipt top)</div>
                    <div><strong>• Following images (#2, #3...):</strong> Product sections in the order they appear on the bill</div>
                    <div><strong>• Last image:</strong> Bill footer (subtotal, tax, total amount, payment info)</div>
                    <div><strong>• Use ↑↓ buttons:</strong> To reorder images if needed for accurate processing</div>
                  </div>
                </div>
              </BannerTitle>
            </Banner>
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
              <div className="border border-gray-200 rounded-lg p-4 space-y-4 max-h-[60vh] overflow-y-auto">
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
                          <Pill variant="secondary" className="text-xs font-semibold">
                            #{index + 1}
                          </Pill>
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

        {/* Results Panel */}
        {showResults && (
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
        )}
      </div>
    </div>
  );
}