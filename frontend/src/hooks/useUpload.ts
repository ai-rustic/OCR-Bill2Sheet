import { useState, useCallback, useRef } from 'react';
import {
  UploadedImage,
  UploadStatus,
  AcceptedFileTypes,
  UseUploadReturn,
  DEFAULT_ACCEPTED_TYPES
} from '@/types/upload';
import { validateFiles, generateFileId } from '@/utils/fileValidation';
import { generatePreview, cleanupPreview } from '@/utils/previewGeneration';
import { simulateUpload } from '@/utils/uploadSimulation';

/**
 * Custom hook for managing upload state and operations
 * Provides complete state management for the upload image feature
 */
export function useUpload(acceptedTypes: AcceptedFileTypes = DEFAULT_ACCEPTED_TYPES): UseUploadReturn {
  const [images, setImages] = useState<UploadedImage[]>([]);
  const [isDragActive, setIsDragActive] = useState(false);
  const [errors, setErrors] = useState<string[]>([]);

  // Keep track of active upload cleanup functions
  const uploadCleanups = useRef<Map<string, () => void>>(new Map());

  // Calculate derived state
  const isUploading = images.some(img => img.status === UploadStatus.UPLOADING);
  const stats = {
    total: images.length,
    completed: images.filter(img => img.status === UploadStatus.COMPLETED).length,
    uploading: images.filter(img => img.status === UploadStatus.UPLOADING).length,
    errors: images.filter(img => img.status === UploadStatus.ERROR).length
  };

  /**
   * Add new files to the upload queue
   */
  const addFiles = useCallback(async (files: File[]) => {
    // Validate files
    const { validFiles, invalidFiles, totalErrors } = validateFiles(files, acceptedTypes);

    // Add validation errors to state
    if (totalErrors.length > 0) {
      setErrors(prev => [...prev, ...totalErrors]);
    }

    // Process valid files
    if (validFiles.length > 0) {
      const newImages: UploadedImage[] = validFiles.map(file => ({
        id: generateFileId(file),
        file,
        fileName: file.name,
        fileSize: file.size,
        fileType: file.type,
        progress: 0,
        status: UploadStatus.UPLOADING,
        previewUrl: null,
        error: null,
        uploadedAt: new Date()
      }));

      // Add images to state
      setImages(prev => [...prev, ...newImages]);

      // Start upload simulation and preview generation for each file
      newImages.forEach(async (image) => {
        try {
          // Generate preview
          const previewUrl = await generatePreview(image.file);
          setImages(prev => prev.map(img =>
            img.id === image.id ? { ...img, previewUrl } : img
          ));

          // Start upload simulation
          const cleanup = simulateUpload(
            image.file,
            // onProgress
            (progress) => {
              setImages(prev => prev.map(img =>
                img.id === image.id ? { ...img, progress } : img
              ));
            },
            // onComplete
            () => {
              setImages(prev => prev.map(img =>
                img.id === image.id
                  ? { ...img, status: UploadStatus.COMPLETED, progress: 100 }
                  : img
              ));
              uploadCleanups.current.delete(image.id);
            },
            // onError
            (error) => {
              setImages(prev => prev.map(img =>
                img.id === image.id
                  ? { ...img, status: UploadStatus.ERROR, error }
                  : img
              ));
              uploadCleanups.current.delete(image.id);
            }
          );

          // Store cleanup function
          uploadCleanups.current.set(image.id, cleanup);

        } catch (error) {
          // Handle preview generation error
          setImages(prev => prev.map(img =>
            img.id === image.id
              ? {
                  ...img,
                  status: UploadStatus.ERROR,
                  error: 'Failed to generate preview'
                }
              : img
          ));
        }
      });
    }
  }, [acceptedTypes]);

  /**
   * Remove an image from the upload list
   */
  const removeImage = useCallback((id: string) => {
    // Cancel upload if in progress
    const cleanup = uploadCleanups.current.get(id);
    if (cleanup) {
      cleanup();
      uploadCleanups.current.delete(id);
    }

    // Find image and cleanup preview URL
    setImages(prev => {
      const imageToRemove = prev.find(img => img.id === id);
      if (imageToRemove?.previewUrl) {
        cleanupPreview(imageToRemove.previewUrl);
      }
      return prev.filter(img => img.id !== id);
    });
  }, []);

  /**
   * Retry a failed upload
   */
  const retryUpload = useCallback((id: string) => {
    const image = images.find(img => img.id === id);
    if (!image || image.status !== UploadStatus.ERROR) return;

    // Reset image status
    setImages(prev => prev.map(img =>
      img.id === id
        ? { ...img, status: UploadStatus.UPLOADING, progress: 0, error: null }
        : img
    ));

    // Start upload simulation again
    const cleanup = simulateUpload(
      image.file,
      // onProgress
      (progress) => {
        setImages(prev => prev.map(img =>
          img.id === id ? { ...img, progress } : img
        ));
      },
      // onComplete
      () => {
        setImages(prev => prev.map(img =>
          img.id === id
            ? { ...img, status: UploadStatus.COMPLETED, progress: 100 }
            : img
        ));
        uploadCleanups.current.delete(id);
      },
      // onError
      (error) => {
        setImages(prev => prev.map(img =>
          img.id === id
            ? { ...img, status: UploadStatus.ERROR, error }
            : img
        ));
        uploadCleanups.current.delete(id);
      }
    );

    // Store cleanup function
    uploadCleanups.current.set(id, cleanup);
  }, [images]);

  /**
   * Clear all errors
   */
  const clearErrors = useCallback(() => {
    setErrors([]);
  }, []);

  const setErrorsCallback = useCallback((newErrors: string[]) => {
    setErrors(newErrors);
  }, []);

  /**
   * Clear all completed uploads
   */
  const clearCompleted = useCallback(() => {
    setImages(prev => {
      const completed = prev.filter(img => img.status === UploadStatus.COMPLETED);
      // Cleanup preview URLs for completed images
      completed.forEach(img => {
        if (img.previewUrl) {
          cleanupPreview(img.previewUrl);
        }
      });
      return prev.filter(img => img.status !== UploadStatus.COMPLETED);
    });
  }, []);

  /**
   * Set drag active state
   */
  const setDragActive = useCallback((active: boolean) => {
    setIsDragActive(active);
  }, []);

  // Cleanup on unmount
  const cleanup = useCallback(() => {
    // Cancel all active uploads
    uploadCleanups.current.forEach(cleanup => cleanup());
    uploadCleanups.current.clear();

    // Cleanup all preview URLs
    images.forEach(img => {
      if (img.previewUrl) {
        cleanupPreview(img.previewUrl);
      }
    });
  }, [images]);

  // Return the hook interface
  return {
    images,
    isUploading,
    isDragActive,
    errors,
    addFiles,
    removeImage,
    retryUpload,
    clearErrors,
    setErrors: setErrorsCallback,
    clearCompleted,
    setDragActive,
    stats
  };
}

/**
 * Hook variant with additional configuration options
 */
export function useUploadWithConfig(config: {
  acceptedTypes?: AcceptedFileTypes;
  maxFiles?: number;
  autoRetry?: boolean;
  retryAttempts?: number;
}) {
  const {
    acceptedTypes = DEFAULT_ACCEPTED_TYPES,
    maxFiles,
    autoRetry = false,
    retryAttempts = 3
  } = config;

  const upload = useUpload(acceptedTypes);
  const [retryCount, setRetryCount] = useState<Map<string, number>>(new Map());

  // Override addFiles to respect maxFiles limit
  const addFiles = useCallback((files: File[]) => {
    let filesToAdd = files;

    if (maxFiles && upload.images.length + files.length > maxFiles) {
      const availableSlots = maxFiles - upload.images.length;
      if (availableSlots <= 0) {
        upload.clearErrors();
        upload.setErrors([`Maximum ${maxFiles} files allowed`]);
        return;
      }
      filesToAdd = files.slice(0, availableSlots);
    }

    upload.addFiles(filesToAdd);
  }, [upload, maxFiles]);

  // Override retryUpload to implement auto-retry
  const retryUpload = useCallback((id: string) => {
    if (autoRetry) {
      const attempts = retryCount.get(id) || 0;
      if (attempts < retryAttempts) {
        setRetryCount(prev => new Map(prev).set(id, attempts + 1));
        upload.retryUpload(id);
      }
    } else {
      upload.retryUpload(id);
    }
  }, [upload, autoRetry, retryAttempts, retryCount]);

  return {
    ...upload,
    addFiles,
    retryUpload,
    config: {
      acceptedTypes,
      maxFiles,
      autoRetry,
      retryAttempts
    }
  };
}