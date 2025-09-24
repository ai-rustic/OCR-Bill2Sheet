/**
 * Preview generation utilities for upload image feature
 */

/**
 * Generate preview URL from file using FileReader API
 */
export function generatePreview(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    // Validate that file is an image
    if (!file.type.startsWith('image/')) {
      reject(new Error('File is not an image'));
      return;
    }

    const reader = new FileReader();

    reader.onload = (event) => {
      const result = event.target?.result;
      if (typeof result === 'string') {
        resolve(result);
      } else {
        reject(new Error('Failed to read file as data URL'));
      }
    };

    reader.onerror = () => {
      reject(new Error('Failed to read file'));
    };

    reader.readAsDataURL(file);
  });
}

/**
 * Cleanup preview URL when no longer needed
 * For data URLs created by FileReader, no cleanup is needed
 * For blob URLs created by URL.createObjectURL, this would revoke them
 */
export function cleanupPreview(url: string): void {
  // Check if it's a blob URL (starts with blob:)
  if (url.startsWith('blob:')) {
    URL.revokeObjectURL(url);
  }
  // Data URLs (data:image/...) don't need cleanup
  // They are garbage collected automatically
}

/**
 * Generate thumbnail with specific dimensions using Canvas API
 */
export function generateThumbnail(
  file: File,
  width: number,
  height: number
): Promise<string> {
  return new Promise((resolve, reject) => {
    // Validate that file is an image
    if (!file.type.startsWith('image/')) {
      reject(new Error('File is not an image'));
      return;
    }

    const img = new Image();
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');

    if (!ctx) {
      reject(new Error('Canvas 2D context not supported'));
      return;
    }

    // Create object URL for the image
    const objectUrl = URL.createObjectURL(file);
    img.src = objectUrl;

    img.onload = () => {
      try {
        // Set canvas dimensions
        canvas.width = width;
        canvas.height = height;

        // Calculate scaling to maintain aspect ratio
        const aspectRatio = img.width / img.height;
        let drawWidth = width;
        let drawHeight = height;
        let offsetX = 0;
        let offsetY = 0;

        if (aspectRatio > width / height) {
          // Image is wider than target
          drawHeight = width / aspectRatio;
          offsetY = (height - drawHeight) / 2;
        } else {
          // Image is taller than target
          drawWidth = height * aspectRatio;
          offsetX = (width - drawWidth) / 2;
        }

        // Clear canvas and draw image
        ctx.clearRect(0, 0, width, height);
        ctx.drawImage(img, offsetX, offsetY, drawWidth, drawHeight);

        // Convert to data URL
        const thumbnailDataUrl = canvas.toDataURL('image/jpeg', 0.8);
        URL.revokeObjectURL(objectUrl);
        resolve(thumbnailDataUrl);
      } catch (error) {
        URL.revokeObjectURL(objectUrl);
        reject(error);
      }
    };

    img.onerror = () => {
      URL.revokeObjectURL(objectUrl);
      reject(new Error('Failed to load image'));
    };
  });
}

/**
 * Generate preview URL using URL.createObjectURL (alternative method)
 * This is faster than FileReader but requires manual cleanup
 */
export function generateBlobPreview(file: File): string {
  if (!file.type.startsWith('image/')) {
    throw new Error('File is not an image');
  }
  return URL.createObjectURL(file);
}

/**
 * Get image dimensions from file
 */
export function getImageDimensions(file: File): Promise<{ width: number; height: number }> {
  return new Promise((resolve, reject) => {
    if (!file.type.startsWith('image/')) {
      reject(new Error('File is not an image'));
      return;
    }

    const img = new Image();
    const objectUrl = URL.createObjectURL(file);

    img.onload = () => {
      URL.revokeObjectURL(objectUrl);
      resolve({
        width: img.naturalWidth,
        height: img.naturalHeight
      });
    };

    img.onerror = () => {
      URL.revokeObjectURL(objectUrl);
      reject(new Error('Failed to load image'));
    };

    img.src = objectUrl;
  });
}

/**
 * Check if browser supports required APIs
 */
export function isPreviewSupported(): boolean {
  return (
    typeof FileReader !== 'undefined' &&
    typeof URL !== 'undefined' &&
    typeof URL.createObjectURL === 'function'
  );
}

/**
 * Generate multiple preview sizes for different use cases
 */
export async function generateMultiplePreviews(file: File): Promise<{
  full: string;
  thumbnail: string;
  small: string;
}> {
  const [full, thumbnail, small] = await Promise.all([
    generatePreview(file),
    generateThumbnail(file, 150, 150),
    generateThumbnail(file, 64, 64)
  ]);

  return { full, thumbnail, small };
}

/**
 * Batch generate previews for multiple files
 */
export async function generatePreviewsForFiles(files: File[]): Promise<{
  successful: { file: File; preview: string }[];
  failed: { file: File; error: string }[];
}> {
  const successful: { file: File; preview: string }[] = [];
  const failed: { file: File; error: string }[] = [];

  await Promise.allSettled(
    files.map(async (file) => {
      try {
        const preview = await generatePreview(file);
        successful.push({ file, preview });
      } catch (error) {
        failed.push({
          file,
          error: error instanceof Error ? error.message : 'Unknown error'
        });
      }
    })
  );

  return { successful, failed };
}