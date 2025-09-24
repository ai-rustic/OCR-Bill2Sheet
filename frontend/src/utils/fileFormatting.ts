/**
 * File formatting utilities for upload image feature
 * Handles file size formatting, date formatting, and other display utilities
 */

/**
 * Format file size in human-readable format
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
}

/**
 * Format date in relative time (e.g., "2 minutes ago", "1 hour ago")
 */
export function formatRelativeTime(date: Date): string {
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSeconds = Math.floor(diffMs / 1000);
  const diffMinutes = Math.floor(diffSeconds / 60);
  const diffHours = Math.floor(diffMinutes / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffSeconds < 60) {
    return 'Just now';
  } else if (diffMinutes < 60) {
    return `${diffMinutes} minute${diffMinutes !== 1 ? 's' : ''} ago`;
  } else if (diffHours < 24) {
    return `${diffHours} hour${diffHours !== 1 ? 's' : ''} ago`;
  } else if (diffDays < 7) {
    return `${diffDays} day${diffDays !== 1 ? 's' : ''} ago`;
  } else {
    return date.toLocaleDateString();
  }
}

/**
 * Format upload progress as percentage string
 */
export function formatProgress(progress: number): string {
  return `${Math.round(progress)}%`;
}

/**
 * Format upload speed (bytes per second)
 */
export function formatUploadSpeed(bytesPerSecond: number): string {
  return `${formatFileSize(bytesPerSecond)}/s`;
}

/**
 * Format estimated time remaining
 */
export function formatTimeRemaining(seconds: number): string {
  if (seconds < 60) {
    return `${Math.round(seconds)}s remaining`;
  } else if (seconds < 3600) {
    const minutes = Math.round(seconds / 60);
    return `${minutes}m remaining`;
  } else {
    const hours = Math.round(seconds / 3600);
    return `${hours}h remaining`;
  }
}

/**
 * Truncate filename if too long
 */
export function truncateFilename(filename: string, maxLength = 30): string {
  if (filename.length <= maxLength) return filename;

  const extension = filename.split('.').pop() || '';
  const nameWithoutExt = filename.slice(0, filename.lastIndexOf('.'));
  const maxNameLength = maxLength - extension.length - 4; // 4 for "..." and "."

  if (maxNameLength <= 0) return filename;

  return `${nameWithoutExt.slice(0, maxNameLength)}...${extension}`;
}

/**
 * Get file type display name
 */
export function getFileTypeDisplayName(mimeType: string): string {
  const typeMap: Record<string, string> = {
    'image/jpeg': 'JPEG Image',
    'image/jpg': 'JPEG Image',
    'image/png': 'PNG Image',
    'image/jfif': 'JFIF Image',
    'image/gif': 'GIF Image',
    'image/webp': 'WebP Image',
    'image/svg+xml': 'SVG Image',
    'image/bmp': 'BMP Image',
    'image/tiff': 'TIFF Image'
  };

  return typeMap[mimeType] || 'Image';
}

/**
 * Format upload statistics
 */
export function formatUploadStats(stats: {
  total: number;
  completed: number;
  uploading: number;
  errors: number;
}): string {
  const { total, completed, uploading, errors } = stats;

  if (total === 0) return 'No files';

  const parts: string[] = [];

  if (completed > 0) {
    parts.push(`${completed} completed`);
  }

  if (uploading > 0) {
    parts.push(`${uploading} uploading`);
  }

  if (errors > 0) {
    parts.push(`${errors} failed`);
  }

  return parts.length > 0 ? parts.join(', ') : `${total} total`;
}

/**
 * Format file dimensions
 */
export function formatDimensions(width: number, height: number): string {
  return `${width} × ${height}`;
}

/**
 * Format file resolution category
 */
export function getResolutionCategory(width: number, height: number): string {
  const pixels = width * height;

  if (pixels >= 8000000) return '4K+'; // 8MP+
  if (pixels >= 4000000) return '4K';   // 4MP+
  if (pixels >= 2000000) return 'HD+';  // 2MP+
  if (pixels >= 1000000) return 'HD';   // 1MP+
  if (pixels >= 500000) return 'SD+';   // 0.5MP+
  return 'SD';
}

/**
 * Create a detailed file summary
 */
export function createFileSummary(file: File, dimensions?: { width: number; height: number }): string {
  const parts: string[] = [];

  // File type
  parts.push(getFileTypeDisplayName(file.type));

  // File size
  parts.push(formatFileSize(file.size));

  // Dimensions if available
  if (dimensions) {
    parts.push(formatDimensions(dimensions.width, dimensions.height));
    parts.push(getResolutionCategory(dimensions.width, dimensions.height));
  }

  return parts.join(' • ');
}

/**
 * Format validation errors for display
 */
export function formatValidationErrors(errors: string[]): string {
  if (errors.length === 0) return '';
  if (errors.length === 1) return errors[0];

  return `Multiple issues: ${errors.join('; ')}`;
}

/**
 * Format multiple file selection summary
 */
export function formatFileSelectionSummary(files: File[]): string {
  if (files.length === 0) return 'No files selected';
  if (files.length === 1) return `1 file selected (${formatFileSize(files[0].size)})`;

  const totalSize = files.reduce((sum, file) => sum + file.size, 0);
  return `${files.length} files selected (${formatFileSize(totalSize)})`;
}

/**
 * Convert bytes to appropriate unit
 */
export function convertBytesToUnit(bytes: number, unit: 'B' | 'KB' | 'MB' | 'GB'): number {
  const conversions = {
    B: 1,
    KB: 1024,
    MB: 1024 * 1024,
    GB: 1024 * 1024 * 1024
  };

  return bytes / conversions[unit];
}

/**
 * Check if file size is within reasonable limits for preview
 */
export function isPreviewable(file: File, maxPreviewSize = 50 * 1024 * 1024): boolean {
  return file.size <= maxPreviewSize && file.type.startsWith('image/');
}

/**
 * Generate a color based on file extension for visual distinction
 */
export function getFileExtensionColor(filename: string): string {
  const extension = filename.split('.').pop()?.toLowerCase() || '';
  const colors: Record<string, string> = {
    jpg: '#FF6B35',
    jpeg: '#FF6B35',
    png: '#00A8CC',
    jfif: '#FF9500',
    gif: '#8A2BE2',
    webp: '#32CD32',
    svg: '#FFD700',
    bmp: '#DC143C',
    tiff: '#4169E1'
  };

  return colors[extension] || '#6B7280'; // Default gray
}
