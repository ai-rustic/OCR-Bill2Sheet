/**
 * TypeScript Types for Upload Image UI Components
 *
 * Adapted from contracts/upload-components.ts for frontend implementation
 */

// Core data types
export enum UploadStatus {
  UPLOADING = 'uploading',
  COMPLETED = 'completed',
  ERROR = 'error'
}

export interface UploadedImage {
  id: string;
  file: File;
  fileName: string;
  fileSize: number;
  fileType: string;
  progress: number;
  status: UploadStatus;
  previewUrl: string | null;
  error: string | null;
  uploadedAt: Date;
}

export interface AcceptedFileTypes {
  mimeTypes: string[];
  extensions: string[];
  maxSize: number;
}

export interface ValidationResult {
  isValid: boolean;
  errors: string[];
}

// Component Props Interfaces

/**
 * Main upload area component that handles drag & drop and file selection
 */
export interface UploadAreaProps {
  /** Callback when files are added via drag/drop or file selection */
  onFilesAdded: (files: File[]) => void;
  /** Configuration for accepted file types and validation */
  acceptedTypes: AcceptedFileTypes;
  /** Whether the upload area should be disabled */
  isDisabled?: boolean;
  /** Maximum number of files allowed (unlimited if not set) */
  maxFiles?: number;
  /** Current drag state for visual feedback */
  isDragActive?: boolean;
  /** Handler for drag enter events */
  onDragEnter?: () => void;
  /** Handler for drag leave events */
  onDragLeave?: () => void;
}

/**
 * Individual image preview component with progress and actions
 */
export interface ImagePreviewProps {
  /** The uploaded image data and metadata */
  image: UploadedImage;
  /** Callback to delete this image from the upload list */
  onDelete: (id: string) => void;
  /** Optional callback to retry failed uploads */
  onRetry?: (id: string) => void;
  /** Whether to show full size or thumbnail */
  size?: 'small' | 'medium' | 'large';
}

/**
 * Progress indicator component for upload status
 */
export interface ProgressIndicatorProps {
  /** Progress percentage (0-100) */
  progress: number;
  /** Current upload status */
  status: UploadStatus;
  /** File name for display */
  fileName: string;
  /** Optional error message */
  error?: string | null;
}

/**
 * Error alert component for validation errors
 */
export interface ErrorAlertProps {
  /** Array of error messages to display */
  errors: string[];
  /** Callback to dismiss errors */
  onDismiss: () => void;
  /** Whether the alert is currently visible */
  isVisible: boolean;
}

/**
 * File list grid component that displays all uploaded images
 */
export interface FileListProps {
  /** Array of uploaded images to display */
  images: UploadedImage[];
  /** Callback to delete an image */
  onDelete: (id: string) => void;
  /** Callback to retry a failed upload */
  onRetry?: (id: string) => void;
  /** Grid layout configuration */
  columns?: number;
  /** Whether to show progress for uploading files */
  showProgress?: boolean;
}

// State Management Interfaces

/**
 * Hook for managing upload state and operations
 */
export interface UseUploadReturn {
  /** Array of all uploaded images */
  images: UploadedImage[];
  /** Whether any uploads are currently in progress */
  isUploading: boolean;
  /** Whether drag is currently active over drop zone */
  isDragActive: boolean;
  /** Array of current validation errors */
  errors: string[];
  /** Add new files to the upload queue */
  addFiles: (files: File[]) => void;
  /** Remove an image from the upload list */
  removeImage: (id: string) => void;
  /** Retry a failed upload */
  retryUpload: (id: string) => void;
  /** Clear all errors */
  clearErrors: () => void;
  /** Clear all completed uploads */
  clearCompleted: () => void;
  /** Set drag active state */
  setDragActive: (active: boolean) => void;
  /** Get upload statistics */
  stats: {
    total: number;
    completed: number;
    uploading: number;
    errors: number;
  };
}

// Utility function contracts

/**
 * File validation utilities
 */
export interface FileValidation {
  /** Check if file type is accepted */
  isValidFileType: (file: File, acceptedTypes: AcceptedFileTypes) => boolean;
  /** Check if file size is within limits */
  isValidFileSize: (file: File, maxSize: number) => boolean;
  /** Validate a file against all rules */
  validateFile: (file: File, acceptedTypes: AcceptedFileTypes) => ValidationResult;
  /** Generate unique file ID */
  generateFileId: (file: File) => string;
  /** Format file size for display */
  formatFileSize: (bytes: number) => string;
}

/**
 * Preview generation utilities
 */
export interface PreviewGeneration {
  /** Generate preview URL from file */
  generatePreview: (file: File) => Promise<string>;
  /** Cleanup preview URL when no longer needed */
  cleanupPreview: (url: string) => void;
  /** Generate thumbnail with specific dimensions */
  generateThumbnail: (file: File, width: number, height: number) => Promise<string>;
}

/**
 * Upload simulation utilities (for demo purposes)
 */
export interface UploadSimulation {
  /** Simulate upload progress for a file */
  simulateUpload: (
    file: File,
    onProgress: (progress: number) => void,
    onComplete: () => void,
    onError: (error: string) => void
  ) => () => void; // Returns cleanup function
}

// Default configuration constants
export const DEFAULT_ACCEPTED_TYPES: AcceptedFileTypes = {
  mimeTypes: ['image/jpeg', 'image/png', 'image/jfif'],
  extensions: ['.jpg', '.jpeg', '.png', '.jfif'],
  maxSize: 10 * 1024 * 1024 // 10MB
};