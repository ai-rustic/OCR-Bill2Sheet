'use client';

import { useState, useEffect } from 'react';
import { Dropzone, DropzoneEmptyState } from '@/components/ui/dropzone';
import { Button } from '@/components/ui/button';
import { Pill } from '@/components/ui/pill';
import { Download, Loader2, X, ArrowUp, ArrowDown, Plus, Info, ChevronDown, ChevronUp, Camera, FileImage, MousePointer2, Download as DownloadIcon } from 'lucide-react';
import Image from 'next/image';

interface FileWithPreview {
  file: File;
  preview: string;
  id: string;
}

interface UploadImagesPanelProps {
  selectedFiles: FileWithPreview[];
  onFilesChange: (files: FileWithPreview[]) => void;
  onProcessOCR: () => void;
  isUploading: boolean;
  showResults: boolean;
}

export function UploadImagesPanel({ 
  selectedFiles, 
  onFilesChange, 
  onProcessOCR, 
  isUploading, 
  showResults 
}: UploadImagesPanelProps) {
  const [showInstructions, setShowInstructions] = useState(true);
  const [hasFilesBeenSelected, setHasFilesBeenSelected] = useState(false);

  // Auto-collapse instructions when files are first selected
  useEffect(() => {
    if (selectedFiles.length > 0 && !hasFilesBeenSelected) {
      setShowInstructions(false);
      setHasFilesBeenSelected(true);
    } else if (selectedFiles.length === 0) {
      setHasFilesBeenSelected(false);
      setShowInstructions(true);
    }
  }, [selectedFiles.length, hasFilesBeenSelected]);

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
      onFilesChange([...selectedFiles, ...filesWithPreviews]);
    });
  };

  const handleRemoveFile = (id: string) => {
    onFilesChange(selectedFiles.filter(f => f.id !== id));
  };

  const handleRemoveAllFiles = () => {
    onFilesChange([]);
  };

  const handleMoveUp = (id: string) => {
    const index = selectedFiles.findIndex(f => f.id === id);
    if (index > 0) {
      const newFiles = [...selectedFiles];
      [newFiles[index - 1], newFiles[index]] = [newFiles[index], newFiles[index - 1]];
      onFilesChange(newFiles);
    }
  };

  const handleMoveDown = (id: string) => {
    const index = selectedFiles.findIndex(f => f.id === id);
    if (index < selectedFiles.length - 1) {
      const newFiles = [...selectedFiles];
      [newFiles[index], newFiles[index + 1]] = [newFiles[index + 1], newFiles[index]];
      onFilesChange(newFiles);
    }
  };

  return (
    <div className={`space-y-4 ${showResults ? 'lg:w-1/2 lg:flex-shrink-0 lg:px-2 lg:box-border' : 'w-full max-w-2xl mx-auto'}`}>
      <div className="text-center space-y-2">
        <h2 className="text-2xl font-bold text-gray-900">Bill OCR Processor</h2>
        <p className="text-gray-600">Upload multiple images of your bill (for long receipts) to extract data and export to Excel</p>
        
        {/* Collapsible Step-by-Step Instructions */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowInstructions(!showInstructions)}
              className="text-blue-700 hover:text-blue-900 hover:bg-blue-50 p-2 h-auto"
            >
              <Info size={16} className="mr-2" />
              <span className="font-medium">How to use Bill OCR Processor</span>
              {showInstructions ? <ChevronUp size={16} className="ml-2" /> : <ChevronDown size={16} className="ml-2" />}
            </Button>
          </div>
          
          <div 
            className={`overflow-hidden transition-all duration-300 ease-in-out ${
              showInstructions ? 'max-h-96 opacity-100' : 'max-h-0 opacity-0'
            }`}
          >
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 space-y-3">
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                {/* Step 1: Take Photos */}
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-blue-600 text-white rounded-full flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">1</div>
                  <div className="space-y-1">
                    <div className="flex items-center space-x-1 text-blue-900 font-medium text-sm">
                      <Camera size={14} />
                      <span>Take Photos</span>
                    </div>
                    <div className="text-xs text-blue-700">
                      Capture bill from top to bottom with clear lighting
                    </div>
                  </div>
                </div>

                {/* Step 2: Upload Images */}
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-blue-600 text-white rounded-full flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">2</div>
                  <div className="space-y-1">
                    <div className="flex items-center space-x-1 text-blue-900 font-medium text-sm">
                      <FileImage size={14} />
                      <span>Upload Images</span>
                    </div>
                    <div className="text-xs text-blue-700">
                      Drag & drop up to 20 images (10MB each)
                    </div>
                  </div>
                </div>

                {/* Step 3: Arrange Order */}
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-blue-600 text-white rounded-full flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">3</div>
                  <div className="space-y-1">
                    <div className="flex items-center space-x-1 text-blue-900 font-medium text-sm">
                      <MousePointer2 size={14} />
                      <span>Order Images</span>
                    </div>
                    <div className="text-xs text-blue-700">
                      Use ↑↓ buttons to arrange from bill top to bottom
                    </div>
                  </div>
                </div>

                {/* Step 4: Process */}
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-blue-600 text-white rounded-full flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">4</div>
                  <div className="space-y-1">
                    <div className="flex items-center space-x-1 text-blue-900 font-medium text-sm">
                      <DownloadIcon size={14} />
                      <span>Process & Download</span>
                    </div>
                    <div className="text-xs text-blue-700">
                      Click process button to extract data to Excel
                    </div>
                  </div>
                </div>
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
            {/* Fixed header */}
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

            {/* Scrollable images grid */}
            <div className="max-h-[70vh] overflow-y-auto">
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
            </div>

            {/* Fixed process button */}
            <div className="pt-4 border-t">
              <Button
                onClick={onProcessOCR}
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