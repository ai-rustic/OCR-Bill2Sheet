import type { AcceptedFileTypes, ValidationResult } from '@/types/upload';

/**
 * File validation utilities for upload image feature
 */

/**
 * Check if file type is accepted based on MIME type and extension
 */
export function isValidFileType(file: File, acceptedTypes: AcceptedFileTypes): boolean {
  // Check MIME type
  const isValidMimeType = acceptedTypes.mimeTypes.includes(file.type);

  // Check file extension
  const fileExtension = file.name.toLowerCase().split('.').pop();
  const isValidExtension = fileExtension ?
    acceptedTypes.extensions.some(ext => ext.toLowerCase().replace('.', '') === fileExtension) :
    false;

  return isValidMimeType && isValidExtension;
}

/**
 * Check if file size is within limits
 */
export function isValidFileSize(file: File, maxSize: number): boolean {
  return file.size <= maxSize;
}

/**
 * Validate a file against all rules
 */
export function validateFile(file: File, acceptedTypes: AcceptedFileTypes): ValidationResult {
  const errors: string[] = [];

  // Check file type
  if (!isValidFileType(file, acceptedTypes)) {
    const allowedExtensions = acceptedTypes.extensions.join(', ');
    errors.push(`File type not supported. Only ${allowedExtensions} files are allowed.`);
  }

  // Check file size
  if (!isValidFileSize(file, acceptedTypes.maxSize)) {
    const maxSizeMB = Math.round(acceptedTypes.maxSize / (1024 * 1024));
    errors.push(`File size too large. Maximum size is ${maxSizeMB}MB.`);
  }

  // Additional validations
  if (file.size === 0) {
    errors.push('File is empty.');
  }

  if (!file.name || file.name.trim() === '') {
    errors.push('File name is required.');
  }

  return {
    isValid: errors.length === 0,
    errors
  };
}

/**
 * Generate unique file ID using file properties and timestamp
 */
export function generateFileId(file: File): string {
  const timestamp = Date.now();
  const fileInfo = `${file.name}-${file.size}-${file.lastModified}`;

  // Simple hash function for generating ID
  let hash = 0;
  for (let i = 0; i < fileInfo.length; i++) {
    const char = fileInfo.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32bit integer
  }

  return `file_${Math.abs(hash)}_${timestamp}`;
}

/**
 * Format file size for display in human-readable format
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
}

/**
 * Get file extension from filename
 */
export function getFileExtension(filename: string): string {
  return filename.split('.').pop()?.toLowerCase() || '';
}

/**
 * Check if file is an image based on MIME type
 */
export function isImageFile(file: File): boolean {
  return file.type.startsWith('image/');
}

/**
 * Batch validate multiple files
 */
export function validateFiles(files: File[], acceptedTypes: AcceptedFileTypes): {
  validFiles: File[];
  invalidFiles: { file: File; errors: string[] }[];
  totalErrors: string[];
} {
  const validFiles: File[] = [];
  const invalidFiles: { file: File; errors: string[] }[] = [];
  const totalErrors: string[] = [];

  files.forEach(file => {
    const result = validateFile(file, acceptedTypes);
    if (result.isValid) {
      validFiles.push(file);
    } else {
      invalidFiles.push({ file, errors: result.errors });
      totalErrors.push(...result.errors);
    }
  });

  return { validFiles, invalidFiles, totalErrors };
}