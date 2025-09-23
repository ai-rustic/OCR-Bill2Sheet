/**
 * Upload Image UI Components Export Index
 *
 * Centralized exports for all upload-related components, hooks, and utilities
 * This provides a clean API for consuming the upload image feature
 */

// Main container components
export { default as UploadImageUI, SimpleUploadImageUI, CompactUploadImageUI } from './UploadImageUI'
export type { UploadImageUIProps } from './UploadImageUI'

// Core UI components
export { default as UploadArea, CompactUploadArea, InlineUploadArea } from './UploadArea'
export { default as FileList, CompactFileList, FileListSummary } from './FileList'
export { default as ImagePreview, ListImagePreview } from './ImagePreview'
export { default as ProgressIndicator, CompactProgressIndicator, AnimatedProgressIndicator } from './ProgressIndicator'
export { default as ErrorAlert, CompactErrorAlert, ToastErrorAlert, InlineErrorAlert } from './ErrorAlert'

// Re-export hooks for convenience
export { useUpload, useUploadWithConfig } from '@/hooks/useUpload'
export { useDragAndDrop, useSimpleDragAndDrop, useDragAndDropSupport, useGlobalDragPrevention } from '@/hooks/useDragAndDrop'

// Re-export types for convenience
export type {
  UploadedImage,
  UploadStatus,
  AcceptedFileTypes,
  ValidationResult,
  UploadAreaProps,
  ImagePreviewProps,
  ProgressIndicatorProps,
  ErrorAlertProps,
  FileListProps,
  UseUploadReturn
} from '@/types/upload'

// Re-export utilities for convenience
export {
  validateFile,
  validateFiles,
  isValidFileType,
  isValidFileSize,
  generateFileId
} from '@/utils/fileValidation'

export {
  generatePreview,
  generateThumbnail,
  cleanupPreview
} from '@/utils/previewGeneration'

export {
  simulateUpload,
  simulateBatchUpload,
  simulateRealisticUpload,
  UploadQueue
} from '@/utils/uploadSimulation'

export {
  formatFileSize,
  formatRelativeTime,
  formatProgress,
  formatUploadSpeed,
  formatTimeRemaining,
  truncateFilename,
  getFileTypeDisplayName,
  formatUploadStats,
  formatDimensions,
  getResolutionCategory,
  createFileSummary,
  formatValidationErrors,
  formatFileSelectionSummary,
  convertBytesToUnit,
  isPreviewable,
  getFileExtensionColor
} from '@/utils/fileFormatting'

// Default configuration
export { DEFAULT_ACCEPTED_TYPES } from '@/types/upload'

/**
 * Convenience hook for quick setup with common configuration
 */
export function useQuickUpload(options?: {
  maxFiles?: number
  autoRetry?: boolean
  onComplete?: (files: File[]) => void
  onError?: (errors: string[]) => void
}) {
  const upload = useUpload()

  const handleFilesAdded = React.useCallback((files: File[]) => {
    if (options?.maxFiles && upload.images.length + files.length > options.maxFiles) {
      const available = options.maxFiles - upload.images.length
      if (available > 0) {
        upload.addFiles(files.slice(0, available))
      }
      if (options.onError) {
        options.onError([`Maximum ${options.maxFiles} files allowed`])
      }
    } else {
      upload.addFiles(files)
    }
  }, [upload, options])

  // Monitor for completion
  React.useEffect(() => {
    const completed = upload.images.filter(img => img.status === 'completed')
    if (completed.length > 0 && options?.onComplete) {
      options.onComplete(completed.map(img => img.file))
    }
  }, [upload.images, options])

  // Monitor for errors
  React.useEffect(() => {
    if (upload.errors.length > 0 && options?.onError) {
      options.onError(upload.errors)
    }
  }, [upload.errors, options])

  return {
    ...upload,
    addFiles: handleFilesAdded
  }
}