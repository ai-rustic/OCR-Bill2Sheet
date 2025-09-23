/**
 * Upload simulation utilities for demo purposes
 * Simulates file upload progress without actual backend integration
 */

/**
 * Simulate upload progress for a file
 * Returns a cleanup function to cancel the simulation
 */
export function simulateUpload(
  file: File,
  onProgress: (progress: number) => void,
  onComplete: () => void,
  onError: (error: string) => void
): () => void {
  let cancelled = false;
  let timeoutId: NodeJS.Timeout;

  // Simulate different upload speeds based on file size
  const baseDelay = 100; // Base delay in ms
  const fileSizeKB = file.size / 1024;
  const delay = Math.max(50, Math.min(300, baseDelay + (fileSizeKB / 100))); // 50-300ms delay

  // Simulate occasional upload failures (5% chance)
  const shouldFail = Math.random() < 0.05;
  const failPoint = shouldFail ? Math.floor(Math.random() * 70) + 20 : -1; // Fail between 20-90%

  let currentProgress = 0;

  const updateProgress = () => {
    if (cancelled) return;

    // Simulate random progress increments
    const increment = Math.random() * 15 + 5; // 5-20% increments
    currentProgress = Math.min(currentProgress + increment, 100);

    // Check for simulated failure
    if (failPoint > 0 && currentProgress >= failPoint) {
      const errorMessages = [
        'Network connection lost',
        'Server temporarily unavailable',
        'File upload timeout',
        'Invalid file format detected',
        'Upload quota exceeded'
      ];
      const randomError = errorMessages[Math.floor(Math.random() * errorMessages.length)];
      onError(randomError);
      return;
    }

    onProgress(Math.floor(currentProgress));

    if (currentProgress >= 100) {
      // Simulate final processing delay
      timeoutId = setTimeout(() => {
        if (!cancelled) {
          onComplete();
        }
      }, delay);
    } else {
      timeoutId = setTimeout(updateProgress, delay);
    }
  };

  // Start the simulation with a small initial delay
  timeoutId = setTimeout(updateProgress, delay);

  // Return cleanup function
  return () => {
    cancelled = true;
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
  };
}

/**
 * Simulate batch upload for multiple files
 */
export function simulateBatchUpload(
  files: File[],
  onFileProgress: (fileId: string, progress: number) => void,
  onFileComplete: (fileId: string) => void,
  onFileError: (fileId: string, error: string) => void,
  onAllComplete: () => void
): () => void {
  const cleanupFunctions: (() => void)[] = [];
  const fileStatus = new Map<string, 'uploading' | 'completed' | 'error'>();

  files.forEach((file, index) => {
    const fileId = `file_${index}_${Date.now()}`;
    fileStatus.set(fileId, 'uploading');

    const cleanup = simulateUpload(
      file,
      (progress) => onFileProgress(fileId, progress),
      () => {
        fileStatus.set(fileId, 'completed');
        onFileComplete(fileId);
        checkAllComplete();
      },
      (error) => {
        fileStatus.set(fileId, 'error');
        onFileError(fileId, error);
        checkAllComplete();
      }
    );

    cleanupFunctions.push(cleanup);
  });

  function checkAllComplete() {
    const allDone = Array.from(fileStatus.values()).every(
      status => status === 'completed' || status === 'error'
    );
    if (allDone) {
      onAllComplete();
    }
  }

  // Return cleanup function for all uploads
  return () => {
    cleanupFunctions.forEach(cleanup => cleanup());
  };
}

/**
 * Simulate upload with realistic network conditions
 */
export function simulateRealisticUpload(
  file: File,
  onProgress: (progress: number) => void,
  onComplete: () => void,
  onError: (error: string) => void,
  options: {
    networkSpeed?: 'slow' | 'medium' | 'fast';
    reliability?: 'poor' | 'good' | 'excellent';
  } = {}
): () => void {
  const { networkSpeed = 'medium', reliability = 'good' } = options;

  // Configure simulation based on network conditions
  const speedConfig = {
    slow: { baseDelay: 500, variance: 200, chunkSize: 5 },
    medium: { baseDelay: 200, variance: 100, chunkSize: 10 },
    fast: { baseDelay: 50, variance: 50, chunkSize: 20 }
  };

  const reliabilityConfig = {
    poor: { failureRate: 0.15, stutterRate: 0.3 },
    good: { failureRate: 0.05, stutterRate: 0.1 },
    excellent: { failureRate: 0.01, stutterRate: 0.02 }
  };

  const speed = speedConfig[networkSpeed];
  const reliability = reliabilityConfig[reliability];

  let cancelled = false;
  let timeoutId: NodeJS.Timeout;
  let currentProgress = 0;

  const updateProgress = () => {
    if (cancelled) return;

    // Simulate network stuttering
    if (Math.random() < reliability.stutterRate) {
      // Stutter: pause for a moment
      timeoutId = setTimeout(updateProgress, speed.baseDelay * 2);
      return;
    }

    // Check for failure
    if (Math.random() < reliability.failureRate && currentProgress > 10) {
      onError('Network error occurred during upload');
      return;
    }

    // Update progress
    const increment = Math.random() * speed.chunkSize + (speed.chunkSize / 2);
    currentProgress = Math.min(currentProgress + increment, 100);
    onProgress(Math.floor(currentProgress));

    if (currentProgress >= 100) {
      onComplete();
    } else {
      const delay = speed.baseDelay + (Math.random() * speed.variance);
      timeoutId = setTimeout(updateProgress, delay);
    }
  };

  // Start simulation
  timeoutId = setTimeout(updateProgress, speed.baseDelay);

  return () => {
    cancelled = true;
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
  };
}

/**
 * Create upload queue manager for controlled batch uploads
 */
export class UploadQueue {
  private queue: Array<{
    file: File;
    id: string;
    callbacks: {
      onProgress: (progress: number) => void;
      onComplete: () => void;
      onError: (error: string) => void;
    };
  }> = [];

  private activeUploads = new Map<string, () => void>();
  private maxConcurrent: number;

  constructor(maxConcurrent = 3) {
    this.maxConcurrent = maxConcurrent;
  }

  addToQueue(
    file: File,
    id: string,
    onProgress: (progress: number) => void,
    onComplete: () => void,
    onError: (error: string) => void
  ): void {
    this.queue.push({
      file,
      id,
      callbacks: { onProgress, onComplete, onError }
    });
    this.processQueue();
  }

  private processQueue(): void {
    while (this.queue.length > 0 && this.activeUploads.size < this.maxConcurrent) {
      const upload = this.queue.shift();
      if (!upload) break;

      const cleanup = simulateUpload(
        upload.file,
        upload.callbacks.onProgress,
        () => {
          this.activeUploads.delete(upload.id);
          upload.callbacks.onComplete();
          this.processQueue(); // Process next in queue
        },
        (error) => {
          this.activeUploads.delete(upload.id);
          upload.callbacks.onError(error);
          this.processQueue(); // Process next in queue
        }
      );

      this.activeUploads.set(upload.id, cleanup);
    }
  }

  cancelUpload(id: string): void {
    const cleanup = this.activeUploads.get(id);
    if (cleanup) {
      cleanup();
      this.activeUploads.delete(id);
    }

    // Remove from queue if not started
    this.queue = this.queue.filter(upload => upload.id !== id);
  }

  cancelAll(): void {
    this.activeUploads.forEach(cleanup => cleanup());
    this.activeUploads.clear();
    this.queue = [];
  }

  getQueueStatus(): {
    queueLength: number;
    activeUploads: number;
    totalUploads: number;
  } {
    return {
      queueLength: this.queue.length,
      activeUploads: this.activeUploads.size,
      totalUploads: this.queue.length + this.activeUploads.size
    };
  }
}