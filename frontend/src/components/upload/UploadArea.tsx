"use client"

import * as React from "react"
import { Upload, FileImage, AlertCircle } from "lucide-react"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import type { UploadAreaProps } from "@/types/upload"
import { useDragAndDrop } from "@/hooks/useDragAndDrop"
import { formatFileSelectionSummary } from "@/utils/fileFormatting"

/**
 * Main upload area component that handles drag & drop and file selection
 */
export function UploadArea({
  onFilesAdded,
  acceptedTypes,
  isDisabled = false,
  maxFiles,
  isDragActive: externalDragActive,
  onDragEnter,
  onDragLeave,
  className,
  ...props
}: UploadAreaProps & { className?: string }) {
  const fileInputRef = React.useRef<HTMLInputElement>(null)

  // Use drag and drop hook for handling drag events
  const {
    isDragActive,
    isDragOver,
    isValidDrag,
    dragError,
    dragFileCount,
    dragHandlers,
    resetDragState
  } = useDragAndDrop(
    onFilesAdded,
    acceptedTypes,
    {
      maxFiles,
      disabled: isDisabled
    }
  )

  // Use external drag state if provided, otherwise use internal state
  const dragActiveState = externalDragActive !== undefined ? externalDragActive : isDragActive

  // Handle file input change
  const handleFileInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(event.target.files || [])
    if (files.length > 0) {
      onFilesAdded(files)
    }
    // Reset the input value to allow selecting the same files again
    event.target.value = ''
  }

  // Handle click to open file dialog
  const handleClick = () => {
    if (!isDisabled && fileInputRef.current) {
      fileInputRef.current.click()
    }
  }

  // Handle keyboard events for accessibility
  const handleKeyDown = (event: React.KeyboardEvent) => {
    if ((event.key === 'Enter' || event.key === ' ') && !isDisabled) {
      event.preventDefault()
      handleClick()
    }
  }

  // Format accepted file types for display
  const acceptedExtensions = acceptedTypes.extensions.join(', ')
  const maxSizeText = formatFileSize(acceptedTypes.maxSize)

  // Combine drag handlers with external callbacks
  const combinedDragHandlers = {
    onDragEnter: (e: React.DragEvent<HTMLElement>) => {
      dragHandlers.onDragEnter(e)
      onDragEnter?.()
    },
    onDragLeave: (e: React.DragEvent<HTMLElement>) => {
      dragHandlers.onDragLeave(e)
      onDragLeave?.()
    },
    onDragOver: dragHandlers.onDragOver,
    onDrop: dragHandlers.onDrop
  }

  return (
    <Card
      className={cn(
        "relative cursor-pointer transition-all duration-200 border-2 border-dashed",
        "hover:border-blue-300 hover:bg-blue-50/50",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500",
        dragActiveState && "border-blue-400 bg-blue-50",
        isDragOver && isValidDrag && "border-green-400 bg-green-50",
        isDragOver && !isValidDrag && "border-red-400 bg-red-50",
        isDisabled && "cursor-not-allowed opacity-50 hover:border-gray-300 hover:bg-gray-50",
        className
      )}
      tabIndex={isDisabled ? -1 : 0}
      role="button"
      aria-label="Click to select files or drag and drop files here"
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      {...combinedDragHandlers}
      {...props}
    >
      {/* Hidden file input */}
      <input
        ref={fileInputRef}
        type="file"
        multiple
        accept={acceptedTypes.mimeTypes.join(',')}
        onChange={handleFileInputChange}
        className="hidden"
        disabled={isDisabled}
        aria-hidden="true"
      />

      <div className="flex flex-col items-center justify-center p-8 text-center space-y-4">
        {/* Icon */}
        <div className="relative">
          {dragActiveState ? (
            <div className="relative">
              <FileImage
                className={cn(
                  "h-16 w-16 transition-colors duration-200",
                  isValidDrag ? "text-green-500" : "text-red-500"
                )}
              />
              {!isValidDrag && (
                <AlertCircle className="absolute -bottom-1 -right-1 h-6 w-6 text-red-500 bg-white rounded-full" />
              )}
            </div>
          ) : (
            <Upload className="h-16 w-16 text-gray-400" />
          )}
        </div>

        {/* Main text */}
        <div className="space-y-2">
          {dragActiveState ? (
            <div className="space-y-1">
              <h3 className={cn(
                "text-lg font-semibold",
                isValidDrag ? "text-green-700" : "text-red-700"
              )}>
                {isValidDrag ? "Drop files to upload" : "Invalid files"}
              </h3>
              {dragError && (
                <p className="text-sm text-red-600">{dragError}</p>
              )}
              {isValidDrag && dragFileCount > 0 && (
                <p className="text-sm text-green-600">
                  {dragFileCount} file{dragFileCount !== 1 ? 's' : ''} ready to upload
                </p>
              )}
            </div>
          ) : (
            <div className="space-y-1">
              <h3 className="text-lg font-semibold text-gray-900">
                {isDisabled ? "Upload disabled" : "Drop files here or click to browse"}
              </h3>
              <p className="text-sm text-gray-600">
                {isDisabled
                  ? "File upload is currently disabled"
                  : "Select multiple files to upload them all at once"
                }
              </p>
            </div>
          )}
        </div>

        {/* File restrictions */}
        {!isDisabled && (
          <div className="text-xs text-gray-500 space-y-1">
            <div>Accepted formats: {acceptedExtensions}</div>
            <div>Maximum file size: {maxSizeText}</div>
            {maxFiles && (
              <div>Maximum {maxFiles} files allowed</div>
            )}
          </div>
        )}

        {/* Browse button */}
        {!dragActiveState && !isDisabled && (
          <Button
            type="button"
            variant="outline"
            className="mt-4"
            onClick={(e) => {
              e.stopPropagation()
              handleClick()
            }}
          >
            Browse Files
          </Button>
        )}
      </div>
    </Card>
  )
}

/**
 * Compact upload area for smaller spaces
 */
export function CompactUploadArea({
  onFilesAdded,
  acceptedTypes,
  isDisabled = false,
  maxFiles,
  className,
  ...props
}: Omit<UploadAreaProps, 'isDragActive' | 'onDragEnter' | 'onDragLeave'> & { className?: string }) {
  const fileInputRef = React.useRef<HTMLInputElement>(null)

  const {
    isDragActive,
    isDragOver,
    isValidDrag,
    dragError,
    dragHandlers
  } = useDragAndDrop(
    onFilesAdded,
    acceptedTypes,
    {
      maxFiles,
      disabled: isDisabled
    }
  )

  const handleFileInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(event.target.files || [])
    if (files.length > 0) {
      onFilesAdded(files)
    }
    event.target.value = ''
  }

  const handleClick = () => {
    if (!isDisabled && fileInputRef.current) {
      fileInputRef.current.click()
    }
  }

  return (
    <div
      className={cn(
        "relative cursor-pointer p-4 border-2 border-dashed rounded-lg transition-all duration-200",
        "hover:border-blue-300 hover:bg-blue-50/50",
        isDragActive && "border-blue-400 bg-blue-50",
        isDragOver && isValidDrag && "border-green-400 bg-green-50",
        isDragOver && !isValidDrag && "border-red-400 bg-red-50",
        isDisabled && "cursor-not-allowed opacity-50",
        className
      )}
      onClick={handleClick}
      {...dragHandlers}
      {...props}
    >
      <input
        ref={fileInputRef}
        type="file"
        multiple
        accept={acceptedTypes.mimeTypes.join(',')}
        onChange={handleFileInputChange}
        className="hidden"
        disabled={isDisabled}
      />

      <div className="flex items-center space-x-3">
        <Upload className={cn(
          "h-8 w-8",
          isDragActive && isValidDrag ? "text-green-500" : "text-gray-400"
        )} />
        <div className="flex-1 min-w-0">
          <div className="text-sm font-medium text-gray-900">
            {isDragActive
              ? (isValidDrag ? "Drop to upload" : "Invalid files")
              : "Drop files or click to browse"
            }
          </div>
          <div className="text-xs text-gray-500">
            {dragError || `${acceptedTypes.extensions.join(', ')} up to ${formatFileSize(acceptedTypes.maxSize)}`}
          </div>
        </div>
      </div>
    </div>
  )
}

/**
 * Inline upload area that integrates with existing layouts
 */
export function InlineUploadArea({
  onFilesAdded,
  acceptedTypes,
  isDisabled = false,
  maxFiles,
  className,
  ...props
}: Omit<UploadAreaProps, 'isDragActive' | 'onDragEnter' | 'onDragLeave'> & { className?: string }) {
  const fileInputRef = React.useRef<HTMLInputElement>(null)

  const handleFileInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(event.target.files || [])
    if (files.length > 0) {
      onFilesAdded(files)
    }
    event.target.value = ''
  }

  const handleClick = () => {
    if (!isDisabled && fileInputRef.current) {
      fileInputRef.current.click()
    }
  }

  return (
    <div className={cn("flex items-center space-x-3", className)} {...props}>
      <input
        ref={fileInputRef}
        type="file"
        multiple
        accept={acceptedTypes.mimeTypes.join(',')}
        onChange={handleFileInputChange}
        className="hidden"
        disabled={isDisabled}
      />

      <Button
        type="button"
        variant="outline"
        onClick={handleClick}
        disabled={isDisabled}
        className="flex items-center space-x-2"
      >
        <Upload className="h-4 w-4" />
        <span>Choose Files</span>
      </Button>

      <div className="text-sm text-gray-600">
        {acceptedTypes.extensions.join(', ')} files up to {formatFileSize(acceptedTypes.maxSize)}
        {maxFiles && ` (max ${maxFiles})`}
      </div>
    </div>
  )
}

// Helper function to format file size (if not already imported)
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`
}

export default UploadArea