"use client"

import * as React from "react"
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import type { AcceptedFileTypes, UploadedImage, ImageProcessingState } from "@/types/upload"
import { DEFAULT_ACCEPTED_TYPES } from "@/types/upload"
import { useUpload } from "@/hooks/useUpload"
import { useOcrProcessing } from "@/hooks/useOcrProcessing"
import { useGlobalDragPrevention } from "@/hooks/useDragAndDrop"
import { UploadArea } from "./UploadArea"
import { FileList } from "./FileList"
import { ErrorAlert } from "./ErrorAlert"
import { UploadErrorBoundary, useUploadErrorHandler, useNetworkErrorHandler } from "./ErrorBoundary"
import { useResponsive, ResponsiveLayout, ResponsiveGrid, DEFAULT_RESPONSIVE_CONFIG } from "./ResponsiveUtils"
import { formatUploadStats } from "@/utils/fileFormatting"

/**
 * Props for the main UploadImageUI component
 */
export interface UploadImageUIProps {
  /** Configuration for accepted file types and validation */
  acceptedTypes?: AcceptedFileTypes
  /** Maximum number of files allowed (unlimited if not set) */
  maxFiles?: number
  /** Whether the upload area should be disabled */
  disabled?: boolean
  /** Callback when files are successfully uploaded */
  onUploadComplete?: (files: File[]) => void
  /** Callback when upload errors occur */
  onUploadError?: (errors: string[]) => void
  /** Callback when files are added to upload queue */
  onFilesAdded?: (files: File[]) => void
  /** Callback when upload progress changes */
  onUploadProgress?: (images: UploadedImage[]) => void
  /** Callback when a file is removed */
  onFileRemoved?: (fileId: string, fileName: string) => void
  /** Callback when drag and drop events occur */
  onDragEvents?: {
    onDragEnter?: () => void
    onDragLeave?: () => void
    onDrop?: (files: File[]) => void
  }
  /** Global drag prevention (prevents browser from opening dropped files) */
  preventGlobalDrag?: boolean
  /** Enable enhanced error handling and reporting */
  enhancedErrorHandling?: boolean
  /** Callback for critical errors that require user attention */
  onCriticalError?: (error: Error) => void
  /** Enable responsive design optimization */
  responsive?: boolean
  /** Custom responsive configuration */
  responsiveConfig?: typeof DEFAULT_RESPONSIVE_CONFIG
  /** Custom CSS class */
  className?: string
  /** Show upload statistics */
  showStats?: boolean
  /** Show file list */
  showFileList?: boolean
  /** Initial layout mode */
  initialLayout?: 'compact' | 'standard' | 'detailed'
}

/**
 * Main UploadImageUI container component
 * Orchestrates all upload functionality with complete user flow integration
 */
export function UploadImageUI({
  acceptedTypes = DEFAULT_ACCEPTED_TYPES,
  maxFiles,
  disabled = false,
  onUploadComplete,
  onUploadError,
  onFilesAdded,
  onUploadProgress,
  onFileRemoved,
  onDragEvents,
  preventGlobalDrag = true,
  enhancedErrorHandling = true,
  onCriticalError,
  responsive = true,
  responsiveConfig = DEFAULT_RESPONSIVE_CONFIG,
  className,
  showStats = true,
  showFileList = true,
  initialLayout = 'standard',
  ...props
}: UploadImageUIProps) {
  const upload = useUpload(acceptedTypes)
  const ocr = useOcrProcessing()
  const [layout, setLayout] = React.useState(initialLayout)
  const [processingStates, setProcessingStates] = React.useState<Record<string, ImageProcessingState>>({})
  const fileIndexToImageIdRef = React.useRef<Record<number, string>>({})
  const lastProcessedEventIndexRef = React.useRef(0)
  const { enablePrevention, disablePrevention } = useGlobalDragPrevention()

  React.useEffect(() => {
    setProcessingStates((prev) => {
      if (Object.keys(prev).length === 0) {
        return prev
      }

      const validIds = new Set(upload.images.map((img) => img.id))
      let changed = false
      const next = { ...prev }

      Object.keys(next).forEach((id) => {
        if (!validIds.has(id)) {
          delete next[id]
          changed = true
        }
      })

      Object.entries(fileIndexToImageIdRef.current).forEach(([index, id]) => {
        if (!validIds.has(id)) {
          delete fileIndexToImageIdRef.current[Number(index)]
        }
      })

      return changed ? next : prev
    })
  }, [upload.images])

  // Enhanced error handling
  const errorHandler = useUploadErrorHandler()
  const { isOnline, networkErrors, handleNetworkError, clearNetworkErrors } = useNetworkErrorHandler()

  // Responsive design
  const responsiveContext = useResponsive(responsiveConfig)
  const { isMobile, isTablet, layout: responsiveLayout } = responsiveContext

  // Track previous states for event callbacks
  const prevImagesRef = React.useRef<UploadedImage[]>([])
  const prevCompletedCountRef = React.useRef(0)

  // Global drag prevention setup
  React.useEffect(() => {
    if (preventGlobalDrag) {
      enablePrevention()
      return () => disablePrevention()
    }
  }, [preventGlobalDrag, enablePrevention, disablePrevention])

  // Handle drag state for upload area
  const handleDragEnter = React.useCallback(() => {
    upload.setDragActive(true)
    onDragEvents?.onDragEnter?.()
  }, [upload, onDragEvents])

  const handleDragLeave = React.useCallback(() => {
    upload.setDragActive(false)
    onDragEvents?.onDragLeave?.()
  }, [upload, onDragEvents])

  // Handle file addition with max files validation and enhanced error handling
  const handleFilesAdded = React.useCallback((files: File[]) => {
    try {
      // Notify about the drop event
      onDragEvents?.onDrop?.(files)

      // Clear network errors when user tries to upload
      if (networkErrors.length > 0) {
        clearNetworkErrors()
      }

      // Check if offline
      if (!isOnline && enhancedErrorHandling) {
        const error = 'Cannot upload files while offline. Please check your internet connection.'
        errorHandler.addError(error)
        onUploadError?.([error])
        return
      }

      if (maxFiles && upload.images.length + files.length > maxFiles) {
        const availableSlots = maxFiles - upload.images.length
        if (availableSlots <= 0) {
          upload.clearErrors()
          const error = `Maximum ${maxFiles} files allowed. Please remove some files first.`
          if (enhancedErrorHandling) {
            errorHandler.addError(error)
          }
          onUploadError?.([error])
          return
        }
        const limitedFiles = files.slice(0, availableSlots)
        upload.addFiles(limitedFiles)
        onFilesAdded?.(limitedFiles)

        if (files.length > availableSlots) {
          const error = `Only ${availableSlots} files were added. Maximum ${maxFiles} files allowed.`
          if (enhancedErrorHandling) {
            errorHandler.addError(error)
          }
          onUploadError?.([error])
        }
      } else {
        upload.addFiles(files)
        onFilesAdded?.(files)
      }
    } catch (error) {
      const err = error instanceof Error ? error : new Error('Failed to add files')
      if (enhancedErrorHandling) {
        errorHandler.handleCriticalError(err)
        onCriticalError?.(err)
      }
      console.error('Error adding files:', err)
    }
  }, [upload, maxFiles, onUploadError, onFilesAdded, onDragEvents, networkErrors, clearNetworkErrors, isOnline, enhancedErrorHandling, errorHandler, onCriticalError])

  // Enhanced file removal with callback
  const handleFileRemoved = React.useCallback((fileId: string) => {
    const imageToRemove = upload.images.find(img => img.id === fileId)
    if (imageToRemove) {
      onFileRemoved?.(fileId, imageToRemove.fileName)
    }
    upload.removeImage(fileId)
  }, [upload, onFileRemoved])

  const handleProcessImages = React.useCallback(() => {
    if (upload.images.length === 0) {
      return
    }

    const mapping = upload.images.reduce<Record<number, string>>((acc, img, index) => {
      acc[index] = img.id
      return acc
    }, {})

    fileIndexToImageIdRef.current = mapping
    lastProcessedEventIndexRef.current = 0

    const initialStates = upload.images.reduce<Record<string, ImageProcessingState>>((acc, img) => {
      acc[img.id] = 'processing'
      return acc
    }, {})

    setProcessingStates(initialStates)

    ocr.processImages(upload.images.map((img) => img.file)).catch((error) => {
      console.error('Failed to start OCR processing:', error)
    })
  }, [ocr, upload.images])

  React.useEffect(() => {
    if (lastProcessedEventIndexRef.current >= ocr.events.length) {
      return
    }

    let changed = false

    setProcessingStates((prev) => {
      const next = { ...prev }

      for (let index = lastProcessedEventIndexRef.current; index < ocr.events.length; index += 1) {
        const event = ocr.events[index]
        const data = event.data as { file_index?: number; fileIndex?: number } | undefined
        const fileIndex = data?.file_index ?? data?.fileIndex

        if (typeof fileIndex === 'number') {
          const imageId = fileIndexToImageIdRef.current[fileIndex]
          if (!imageId) {
            continue
          }

          let nextState: ImageProcessingState | undefined

          switch (event.type) {
            case 'image_received':
            case 'image_validation_start':
            case 'image_validation_success':
            case 'gemini_processing_start':
              nextState = 'processing'
              break
            case 'gemini_processing_success':
            case 'bill_data_saved':
              nextState = 'finished'
              break
            case 'image_validation_error':
            case 'gemini_processing_error':
              nextState = 'error'
              break
            default:
              break
          }

          if (nextState && next[imageId] !== nextState) {
            next[imageId] = nextState
            changed = true
          }
        } else if (event.type === 'processing_complete') {
          Object.keys(next).forEach((id) => {
            if (next[id] === 'processing') {
              next[id] = 'finished'
              changed = true
            }
          })
        } else if (event.type === 'processing_error') {
          Object.keys(next).forEach((id) => {
            if (next[id] === 'processing') {
              next[id] = 'error'
              changed = true
            }
          })
        }
      }

      return changed ? next : prev
    })

    lastProcessedEventIndexRef.current = ocr.events.length
  }, [ocr.events])

  // Monitor upload progress changes
  React.useEffect(() => {
    const prevImages = prevImagesRef.current
    const currentImages = upload.images

    // Check if images array has changed
    if (prevImages.length !== currentImages.length ||
        prevImages.some((img, i) =>
          !currentImages[i] ||
          img.id !== currentImages[i].id ||
          img.progress !== currentImages[i].progress ||
          img.status !== currentImages[i].status
        )) {
      onUploadProgress?.(currentImages)
      prevImagesRef.current = [...currentImages]
    }
  }, [upload.images, onUploadProgress])

  // Monitor upload completion (only trigger for newly completed files)
  React.useEffect(() => {
    const completedCount = upload.images.filter(img => img.status === 'completed').length
    const prevCompletedCount = prevCompletedCountRef.current

    if (completedCount > prevCompletedCount) {
      const newlyCompleted = upload.images
        .filter(img => img.status === 'completed')
        .slice(prevCompletedCount)
        .map(img => img.file)

      if (newlyCompleted.length > 0 && onUploadComplete) {
        onUploadComplete(newlyCompleted)
      }
    }

    prevCompletedCountRef.current = completedCount
  }, [upload.images, onUploadComplete])

  // Monitor upload errors
  React.useEffect(() => {
    if (upload.errors.length > 0 && onUploadError) {
      onUploadError(upload.errors)
    }
  }, [upload.errors, onUploadError])

  // Determine effective layout based on responsive context
  const effectiveLayout = responsive && responsiveLayout.compactMode ? 'compact' : layout
  const effectiveColumns = responsive ? responsiveLayout.columns : (layout === 'compact' ? 2 : layout === 'detailed' ? 4 : 3)

  const layoutConfig = {
    compact: {
      container: cn(
        responsive && isMobile ? "space-y-3 px-2" : "space-y-4",
        responsive && isTablet ? "space-y-4 px-4" : ""
      ),
      uploadArea: responsive ? responsiveLayout.uploadAreaHeight : "min-h-32",
      showDetailedStats: responsive ? responsiveLayout.showDetailedStats : false
    },
    standard: {
      container: cn(
        responsive && isMobile ? "space-y-4 px-2" : "space-y-6",
        responsive && isTablet ? "space-y-5 px-4" : ""
      ),
      uploadArea: responsive ? responsiveLayout.uploadAreaHeight : "min-h-48",
      showDetailedStats: responsive ? responsiveLayout.showDetailedStats : true
    },
    detailed: {
      container: cn(
        responsive && isMobile ? "space-y-4 px-2" : "space-y-8",
        responsive && isTablet ? "space-y-6 px-4" : ""
      ),
      uploadArea: responsive ? responsiveLayout.uploadAreaHeight : "min-h-64",
      showDetailedStats: responsive ? responsiveLayout.showDetailedStats : true
    }
  }

  const config = layoutConfig[effectiveLayout]

  const content = responsive ? (
    <ResponsiveLayout config={responsiveConfig} className={cn(config.container, className)} {...props}>
      <div className="w-full max-w-4xl mx-auto">
      {/* Header with stats */}
      {showStats && (upload.images.length > 0 || upload.errors.length > 0) && (
        <Card>
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <div>
                <CardTitle className="text-lg">Upload Progress</CardTitle>
                {config.showDetailedStats && (
                  <div className="text-sm text-gray-600 mt-1">
                    {formatUploadStats(upload.stats)}
                  </div>
                )}
              </div>

              {/* Layout toggle - hidden on mobile when responsive */}
              {!(responsive && isMobile) && (
                <div className="flex items-center space-x-2">
                  <div className="text-xs text-gray-500">Layout:</div>
                  <select
                    value={layout}
                    onChange={(e) => setLayout(e.target.value as typeof layout)}
                    className="text-xs border rounded px-2 py-1"
                    disabled={responsive && responsiveLayout.compactMode}
                  >
                    <option value="compact">Compact</option>
                    <option value="standard">Standard</option>
                    <option value="detailed">Detailed</option>
                  </select>
                  {responsive && (
                    <div className="text-xs text-gray-400">
                      ({responsiveContext.screenSize})
                    </div>
                  )}
                </div>
              )}
            </div>

            {/* Quick stats bar */}
            {!config.showDetailedStats && (
              <div className="flex items-center space-x-4 text-sm">
                <span className="text-gray-600">
                  {upload.stats.total} files
                </span>
                {upload.stats.uploading > 0 && (
                  <span className="text-blue-600">
                    {upload.stats.uploading} uploading
                  </span>
                )}
                {upload.stats.completed > 0 && (
                  <span className="text-green-600">
                    {upload.stats.completed} completed
                  </span>
                )}
                {upload.stats.errors > 0 && (
                  <span className="text-red-600">
                    {upload.stats.errors} failed
                  </span>
                )}
              </div>
            )}

            {/* Bulk actions - responsive layout */}
            {upload.images.length > 0 && (
              <div className={cn(
                "pt-2 border-t",
                responsive && isMobile ? "flex flex-col space-y-2" : "flex items-center space-x-2"
              )}>
                {/* Process Images Button */}
                <Button
                  size="sm"
                  onClick={handleProcessImages}
                  disabled={ocr.isProcessing || upload.images.length === 0}
                  className="bg-blue-600 hover:bg-blue-700 text-white"
                >
                  {ocr.isProcessing ? 'Processing...' : 'Process Images'}
                </Button>

                {upload.stats.errors > 0 && (
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => {
                      upload.images
                        .filter(img => img.status === 'error')
                        .forEach(img => upload.retryUpload(img.id))
                    }}
                    className="text-blue-600"
                  >
                    Retry All Failed
                  </Button>
                )}
                {upload.stats.completed > 0 && (
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={upload.clearCompleted}
                    className="text-green-600"
                  >
                    Clear Completed
                  </Button>
                )}
                <Button
                  size="sm"
                  variant="outline"
                  onClick={() => {
                    upload.images.forEach(img => handleFileRemoved(img.id))
                  }}
                  className="text-red-600"
                >
                  Clear All
                </Button>
              </div>
            )}
          </CardHeader>
        </Card>
      )}

      {/* Error alerts */}
      {(upload.errors.length > 0 || (enhancedErrorHandling && errorHandler.hasErrors) || networkErrors.length > 0) && (
        <div className="space-y-2">
          {/* Standard upload errors */}
          {upload.errors.length > 0 && (
            <ErrorAlert
              errors={upload.errors}
              onDismiss={upload.clearErrors}
              isVisible={true}
            />
          )}

          {/* Enhanced error handler errors */}
          {enhancedErrorHandling && errorHandler.errors.length > 0 && (
            <ErrorAlert
              errors={errorHandler.errors}
              onDismiss={errorHandler.clearErrors}
              isVisible={true}
            />
          )}

          {/* Network errors */}
          {networkErrors.length > 0 && (
            <ErrorAlert
              errors={networkErrors}
              onDismiss={clearNetworkErrors}
              isVisible={true}
            />
          )}

          {/* Offline indicator */}
          {!isOnline && (
            <div className="flex items-center space-x-2 p-3 bg-orange-50 border border-orange-200 rounded-md text-orange-700">
              <div className="w-2 h-2 bg-orange-500 rounded-full animate-pulse" />
              <span className="text-sm">You are currently offline. Uploads will resume when connection is restored.</span>
            </div>
          )}
        </div>
      )}

      {/* Upload area */}
      <Card>
        <CardContent className="p-0">
          <UploadArea
            onFilesAdded={handleFilesAdded}
            acceptedTypes={acceptedTypes}
            isDisabled={disabled}
            maxFiles={maxFiles}
            isDragActive={upload.isDragActive}
            onDragEnter={handleDragEnter}
            onDragLeave={handleDragLeave}
            className={cn("border-0 shadow-none", config.uploadArea)}
          />
        </CardContent>
      </Card>

      {/* File list */}
      {showFileList && upload.images.length > 0 && (
        responsive ? (
          <ResponsiveGrid columns={effectiveColumns}>
            <FileList
              images={upload.images}
              onDelete={handleFileRemoved}
              onRetry={upload.retryUpload}
              columns={effectiveColumns}
              showProgress={!isMobile || upload.images.length <= 3} // Hide progress on mobile for many files
              processingStatuses={processingStates}
              className="col-span-full w-full"
            />
          </ResponsiveGrid>
        ) : (
          <FileList
            images={upload.images}
            onDelete={handleFileRemoved}
            onRetry={upload.retryUpload}
            columns={effectiveColumns}
            showProgress={true}
            processingStatuses={processingStates}
            className="w-full"
          />
        )
      )}

      {/* Upload status summary for compact mode */}
      {layout === 'compact' && upload.isUploading && (
        <div className="text-center text-sm text-gray-600">
          {upload.stats.uploading} file{upload.stats.uploading !== 1 ? 's' : ''} uploading...
        </div>
      )}
      </div>
    </ResponsiveLayout>
  ) : (
    <div className={cn("w-full max-w-4xl mx-auto", config.container, className)} {...props}>
      {/* Non-responsive fallback - duplicate content structure */}
      {/* Header with stats */}
      {showStats && (upload.images.length > 0 || upload.errors.length > 0) && (
        <Card>
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <div>
                <CardTitle className="text-lg">Upload Progress</CardTitle>
                {config.showDetailedStats && (
                  <div className="text-sm text-gray-600 mt-1">
                    {formatUploadStats(upload.stats)}
                  </div>
                )}
              </div>
              <div className="flex items-center space-x-2">
                <div className="text-xs text-gray-500">Layout:</div>
                <select
                  value={layout}
                  onChange={(e) => setLayout(e.target.value as typeof layout)}
                  className="text-xs border rounded px-2 py-1"
                >
                  <option value="compact">Compact</option>
                  <option value="standard">Standard</option>
                  <option value="detailed">Detailed</option>
                </select>
              </div>
            </div>
          </CardHeader>
        </Card>
      )}
      {/* Add similar structure for errors, upload area, file list */}
    </div>
  )

  if (enhancedErrorHandling) {
    return (
      <UploadErrorBoundary
        onError={(error, errorInfo) => {
          errorHandler.handleCriticalError(error)
          onCriticalError?.(error)
        }}
        className={className}
      >
        {content}
      </UploadErrorBoundary>
    )
  }

  return content
}

/**
 * Simplified UploadImageUI for basic use cases
 */
export function SimpleUploadImageUI({
  acceptedTypes = DEFAULT_ACCEPTED_TYPES,
  maxFiles,
  disabled = false,
  onUploadComplete,
  onUploadError,
  className,
  ...props
}: Omit<UploadImageUIProps, 'showStats' | 'showFileList' | 'initialLayout'>) {
  return (
    <UploadImageUI
      acceptedTypes={acceptedTypes}
      maxFiles={maxFiles}
      disabled={disabled}
      onUploadComplete={onUploadComplete}
      onUploadError={onUploadError}
      className={className}
      showStats={false}
      showFileList={true}
      initialLayout="compact"
      {...props}
    />
  )
}

/**
 * Compact UploadImageUI for dashboard or sidebar use
 */
export function CompactUploadImageUI({
  acceptedTypes = DEFAULT_ACCEPTED_TYPES,
  maxFiles = 5,
  disabled = false,
  onUploadComplete,
  onUploadError,
  className,
  ...props
}: Omit<UploadImageUIProps, 'showStats' | 'showFileList' | 'initialLayout'>) {
  const upload = useUpload(acceptedTypes)

  const handleFilesAdded = React.useCallback((files: File[]) => {
    if (maxFiles && upload.images.length + files.length > maxFiles) {
      const availableSlots = maxFiles - upload.images.length
      if (availableSlots <= 0) {
        onUploadError?.([`Maximum ${maxFiles} files allowed`])
        return
      }
      upload.addFiles(files.slice(0, availableSlots))
    } else {
      upload.addFiles(files)
    }
  }, [upload, maxFiles, onUploadError])

  return (
    <div className={cn("space-y-3", className)} {...props}>
      {/* Compact upload area */}
      <UploadArea
        onFilesAdded={handleFilesAdded}
        acceptedTypes={acceptedTypes}
        isDisabled={disabled}
        maxFiles={maxFiles}
        className="min-h-24"
      />

      {/* Error display */}
      {upload.errors.length > 0 && (
        <div className="text-xs text-red-600 p-2 bg-red-50 rounded border border-red-200">
          {upload.errors[0]}
        </div>
      )}

      {/* Compact file list */}
      {upload.images.length > 0 && (
        <div className="space-y-1">
          {upload.images.slice(0, 3).map((image) => (
            <div
              key={image.id}
              className="flex items-center space-x-2 text-xs p-2 bg-gray-50 rounded"
            >
              <div className={cn(
                "w-2 h-2 rounded-full",
                image.status === 'uploading' && "bg-blue-500",
                image.status === 'completed' && "bg-green-500",
                image.status === 'error' && "bg-red-500"
              )} />
              <span className="flex-1 truncate">{image.fileName}</span>
              {image.status === 'uploading' && (
                <span className="text-gray-500">{Math.round(image.progress)}%</span>
              )}
              <Button
                size="sm"
                variant="ghost"
                onClick={() => upload.removeImage(image.id)}
                className="h-4 w-4 p-0 text-red-500"
              >
                ×
              </Button>
            </div>
          ))}
          {upload.images.length > 3 && (
            <div className="text-xs text-gray-500 text-center">
              +{upload.images.length - 3} more files
            </div>
          )}
        </div>
      )}
    </div>
  )
}

export default UploadImageUI
