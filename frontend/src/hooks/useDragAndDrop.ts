import { useState, useCallback, useRef, DragEvent } from 'react';
import type { AcceptedFileTypes } from '@/types/upload';
import { validateFiles } from '@/utils/fileValidation';

/**
 * Hook for managing drag and drop state and operations
 */
export interface UseDragAndDropReturn {
  /** Whether drag is currently active over the drop zone */
  isDragActive: boolean;
  /** Whether drag is over the drop zone (more specific than isDragActive) */
  isDragOver: boolean;
  /** Whether the dragged files are valid for upload */
  isValidDrag: boolean;
  /** Current drag error message if files are invalid */
  dragError: string | null;
  /** Number of files being dragged */
  dragFileCount: number;
  /** Drag event handlers to bind to drop zone element */
  dragHandlers: {
    onDragEnter: (e: DragEvent<HTMLElement>) => void;
    onDragLeave: (e: DragEvent<HTMLElement>) => void;
    onDragOver: (e: DragEvent<HTMLElement>) => void;
    onDrop: (e: DragEvent<HTMLElement>) => void;
  };
  /** Reset all drag state */
  resetDragState: () => void;
}

/**
 * Custom hook for drag and drop functionality
 */
export function useDragAndDrop(
  onFilesDropped: (files: File[]) => void,
  acceptedTypes: AcceptedFileTypes,
  options: {
    maxFiles?: number;
    disabled?: boolean;
  } = {}
): UseDragAndDropReturn {
  const { maxFiles, disabled = false } = options;

  const [isDragActive, setIsDragActive] = useState(false);
  const [isDragOver, setIsDragOver] = useState(false);
  const [isValidDrag, setIsValidDrag] = useState(true);
  const [dragError, setDragError] = useState<string | null>(null);
  const [dragFileCount, setDragFileCount] = useState(0);

  // Track drag counter to handle drag enter/leave correctly
  const dragCounter = useRef(0);

  /**
   * Reset all drag state
   */
  const resetDragState = useCallback(() => {
    setIsDragActive(false);
    setIsDragOver(false);
    setIsValidDrag(true);
    setDragError(null);
    setDragFileCount(0);
    dragCounter.current = 0;
  }, []);

  /**
   * Extract files from drag event
   */
  const getFilesFromDragEvent = useCallback((e: DragEvent<HTMLElement>): File[] => {
    const files: File[] = [];

    if (e.dataTransfer?.items) {
      // Use DataTransferItemList interface
      Array.from(e.dataTransfer.items).forEach(item => {
        if (item.kind === 'file') {
          const file = item.getAsFile();
          if (file) files.push(file);
        }
      });
    } else if (e.dataTransfer?.files) {
      // Use FileList interface
      Array.from(e.dataTransfer.files).forEach(file => {
        files.push(file);
      });
    }

    return files;
  }, []);

  /**
   * Validate dragged files
   */
  const validateDraggedFiles = useCallback((files: File[]) => {
    // Check file count limit
    if (maxFiles && files.length > maxFiles) {
      setIsValidDrag(false);
      setDragError(`Maximum ${maxFiles} files allowed`);
      return;
    }

    // Validate file types and size
    const { validFiles, totalErrors } = validateFiles(files, acceptedTypes);

    if (totalErrors.length > 0) {
      setIsValidDrag(false);
      setDragError(totalErrors[0]); // Show first error
    } else {
      setIsValidDrag(true);
      setDragError(null);
    }

    setDragFileCount(files.length);
  }, [acceptedTypes, maxFiles]);

  /**
   * Handle drag enter event
   */
  const handleDragEnter = useCallback((e: DragEvent<HTMLElement>) => {
    e.preventDefault();
    e.stopPropagation();

    if (disabled) return;

    dragCounter.current++;

    // Only process on first drag enter
    if (dragCounter.current === 1) {
      setIsDragActive(true);

      // Validate files if available
      const files = getFilesFromDragEvent(e);
      if (files.length > 0) {
        validateDraggedFiles(files);
      }
    }
  }, [disabled, getFilesFromDragEvent, validateDraggedFiles]);

  /**
   * Handle drag leave event
   */
  const handleDragLeave = useCallback((e: DragEvent<HTMLElement>) => {
    e.preventDefault();
    e.stopPropagation();

    if (disabled) return;

    dragCounter.current--;

    // Only reset when leaving the drop zone completely
    if (dragCounter.current === 0) {
      setIsDragActive(false);
      setIsDragOver(false);
      setIsValidDrag(true);
      setDragError(null);
      setDragFileCount(0);
    }
  }, [disabled]);

  /**
   * Handle drag over event
   */
  const handleDragOver = useCallback((e: DragEvent<HTMLElement>) => {
    e.preventDefault();
    e.stopPropagation();

    if (disabled) return;

    // Set appropriate drop effect
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = isValidDrag ? 'copy' : 'none';
    }

    setIsDragOver(true);

    // Re-validate files if not already done
    if (dragFileCount === 0) {
      const files = getFilesFromDragEvent(e);
      if (files.length > 0) {
        validateDraggedFiles(files);
      }
    }
  }, [disabled, isValidDrag, dragFileCount, getFilesFromDragEvent, validateDraggedFiles]);

  /**
   * Handle drop event
   */
  const handleDrop = useCallback((e: DragEvent<HTMLElement>) => {
    e.preventDefault();
    e.stopPropagation();

    if (disabled) {
      resetDragState();
      return;
    }

    const files = getFilesFromDragEvent(e);

    // Reset drag state
    resetDragState();

    // Process dropped files if valid
    if (files.length > 0 && isValidDrag) {
      onFilesDropped(files);
    }
  }, [disabled, isValidDrag, getFilesFromDragEvent, onFilesDropped, resetDragState]);

  return {
    isDragActive,
    isDragOver,
    isValidDrag,
    dragError,
    dragFileCount,
    dragHandlers: {
      onDragEnter: handleDragEnter,
      onDragLeave: handleDragLeave,
      onDragOver: handleDragOver,
      onDrop: handleDrop
    },
    resetDragState
  };
}

/**
 * Simplified drag and drop hook with minimal state
 */
export function useSimpleDragAndDrop(
  onFilesDropped: (files: File[]) => void,
  disabled = false
) {
  const [isDragActive, setIsDragActive] = useState(false);
  const dragCounter = useRef(0);

  const handleDragEnter = useCallback((e: DragEvent<HTMLElement>) => {
    e.preventDefault();
    if (disabled) return;

    dragCounter.current++;
    if (dragCounter.current === 1) {
      setIsDragActive(true);
    }
  }, [disabled]);

  const handleDragLeave = useCallback((e: DragEvent<HTMLElement>) => {
    e.preventDefault();
    if (disabled) return;

    dragCounter.current--;
    if (dragCounter.current === 0) {
      setIsDragActive(false);
    }
  }, [disabled]);

  const handleDragOver = useCallback((e: DragEvent<HTMLElement>) => {
    e.preventDefault();
    if (disabled) return;

    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'copy';
    }
  }, [disabled]);

  const handleDrop = useCallback((e: DragEvent<HTMLElement>) => {
    e.preventDefault();

    dragCounter.current = 0;
    setIsDragActive(false);

    if (disabled) return;

    const files = Array.from(e.dataTransfer?.files || []);
    if (files.length > 0) {
      onFilesDropped(files);
    }
  }, [disabled, onFilesDropped]);

  return {
    isDragActive,
    dragHandlers: {
      onDragEnter: handleDragEnter,
      onDragLeave: handleDragLeave,
      onDragOver: handleDragOver,
      onDrop: handleDrop
    }
  };
}

/**
 * Hook for detecting drag and drop support
 */
export function useDragAndDropSupport() {
  const isSupported = typeof window !== 'undefined' &&
    'DataTransfer' in window &&
    'FileReader' in window;

  return { isSupported };
}

/**
 * Hook for global drag and drop prevention
 * Prevents default browser behavior for drag and drop on the entire page
 */
export function useGlobalDragPrevention() {
  const handleDragOver = useCallback((e: Event) => {
    e.preventDefault();
  }, []);

  const handleDrop = useCallback((e: Event) => {
    e.preventDefault();
  }, []);

  // Add/remove global event listeners
  const enablePrevention = useCallback(() => {
    document.addEventListener('dragover', handleDragOver);
    document.addEventListener('drop', handleDrop);
  }, [handleDragOver, handleDrop]);

  const disablePrevention = useCallback(() => {
    document.removeEventListener('dragover', handleDragOver);
    document.removeEventListener('drop', handleDrop);
  }, [handleDragOver, handleDrop]);

  return {
    enablePrevention,
    disablePrevention
  };
}