"use client"

import * as React from "react"

/**
 * Performance optimization utilities for upload components
 */

/**
 * Debounced value hook for optimizing frequent updates
 */
export function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = React.useState(value)

  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value)
    }, delay)

    return () => {
      clearTimeout(handler)
    }
  }, [value, delay])

  return debouncedValue
}

/**
 * Throttled callback hook for limiting function execution rate
 */
export function useThrottle<T extends (...args: unknown[]) => void>(
  callback: T,
  delay: number
): T {
  const lastCall = React.useRef<number>(0)
  const timeoutRef = React.useRef<NodeJS.Timeout | null>(null)

  return React.useCallback(
    ((...args: Parameters<T>) => {
      const now = Date.now()

      if (now - lastCall.current >= delay) {
        lastCall.current = now
        callback(...args)
      } else {
        if (timeoutRef.current) {
          clearTimeout(timeoutRef.current)
        }
        timeoutRef.current = setTimeout(() => {
          lastCall.current = Date.now()
          callback(...args)
        }, delay - (now - lastCall.current))
      }
    }) as T,
    [callback, delay]
  )
}

/**
 * Memoized image preview hook
 */
export function useMemoizedImagePreview(file: File | null) {
  return React.useMemo(() => {
    if (!file) return null
    return URL.createObjectURL(file)
  }, [file])
}

/**
 * Virtual scrolling hook for large file lists
 */
export function useVirtualizedList<T>({
  items,
  itemHeight,
  containerHeight,
  overscan = 5
}: {
  items: T[]
  itemHeight: number
  containerHeight: number
  overscan?: number
}) {
  const [scrollTop, setScrollTop] = React.useState(0)

  const visibleStartIndex = Math.floor(scrollTop / itemHeight)
  const visibleEndIndex = Math.min(
    visibleStartIndex + Math.ceil(containerHeight / itemHeight),
    items.length - 1
  )

  const startIndex = Math.max(0, visibleStartIndex - overscan)
  const endIndex = Math.min(items.length - 1, visibleEndIndex + overscan)

  const visibleItems = React.useMemo(() => {
    return items.slice(startIndex, endIndex + 1).map((item, index) => ({
      item,
      index: startIndex + index
    }))
  }, [items, startIndex, endIndex])

  const totalHeight = items.length * itemHeight
  const offsetY = startIndex * itemHeight

  return {
    visibleItems,
    totalHeight,
    offsetY,
    onScroll: React.useCallback((e: React.UIEvent<HTMLDivElement>) => {
      setScrollTop(e.currentTarget.scrollTop)
    }, [])
  }
}

/**
 * Optimized file reader hook with caching
 */
export function useOptimizedFileReader() {
  const cache = React.useRef<Map<string, string>>(new Map())
  const pendingReads = React.useRef<Map<string, Promise<string>>>(new Map())

  const readFile = React.useCallback(async (file: File): Promise<string> => {
    const cacheKey = `${file.name}-${file.size}-${file.lastModified}`

    // Check cache first
    if (cache.current.has(cacheKey)) {
      return cache.current.get(cacheKey)!
    }

    // Check if already reading
    if (pendingReads.current.has(cacheKey)) {
      return pendingReads.current.get(cacheKey)!
    }

    // Start new read
    const readPromise = new Promise<string>((resolve, reject) => {
      const reader = new FileReader()

      reader.onload = () => {
        const result = reader.result as string
        cache.current.set(cacheKey, result)
        pendingReads.current.delete(cacheKey)
        resolve(result)
      }

      reader.onerror = () => {
        pendingReads.current.delete(cacheKey)
        reject(reader.error)
      }

      reader.readAsDataURL(file)
    })

    pendingReads.current.set(cacheKey, readPromise)
    return readPromise
  }, [])

  const clearCache = React.useCallback(() => {
    // Clean up object URLs to prevent memory leaks
    cache.current.forEach(url => {
      if (url.startsWith('blob:')) {
        URL.revokeObjectURL(url)
      }
    })
    cache.current.clear()
    pendingReads.current.clear()
  }, [])

  return { readFile, clearCache }
}

/**
 * Intersection observer hook for lazy loading
 */
export function useIntersectionObserver(
  options: IntersectionObserverInit & { triggerOnce?: boolean } = {}
) {
  const [isIntersecting, setIsIntersecting] = React.useState(false)
  const [entry, setEntry] = React.useState<IntersectionObserverEntry | null>(null)
  const elementRef = React.useRef<HTMLDivElement>(null)
  const { triggerOnce, ...observerOptions } = options

  React.useEffect(() => {
    const element = elementRef.current
    if (!element) return

    const observer = new IntersectionObserver(
      ([entry]) => {
        setIsIntersecting(entry.isIntersecting)
        setEntry(entry)

        if (triggerOnce && entry.isIntersecting) {
          observer.disconnect()
        }
      },
      observerOptions
    )

    observer.observe(element)

    return () => {
      observer.disconnect()
    }
  }, [observerOptions, triggerOnce])

  return { ref: elementRef, isIntersecting, entry }
}

/**
 * Lazy image component with intersection observer
 */
export const LazyImage = React.memo<{
  src: string
  alt: string
  className?: string
  placeholder?: React.ReactNode
  onLoad?: () => void
  onError?: () => void
}>(({ src, alt, className, placeholder, onLoad, onError }) => {
  const { ref, isIntersecting } = useIntersectionObserver({
    threshold: 0.1,
    triggerOnce: true
  })
  const [isLoaded, setIsLoaded] = React.useState(false)
  const [hasError, setHasError] = React.useState(false)

  const handleLoad = React.useCallback(() => {
    setIsLoaded(true)
    onLoad?.()
  }, [onLoad])

  const handleError = React.useCallback(() => {
    setHasError(true)
    onError?.()
  }, [onError])

  return (
    <div ref={ref} className={className}>
      {isIntersecting && !hasError ? (
        <img
          src={src}
          alt={alt}
          onLoad={handleLoad}
          onError={handleError}
          className={`transition-opacity duration-300 ${
            isLoaded ? 'opacity-100' : 'opacity-0'
          }`}
          loading="lazy"
        />
      ) : (
        placeholder || <div className="bg-gray-100 animate-pulse" />
      )}
    </div>
  )
})

LazyImage.displayName = 'LazyImage'

/**
 * Memoized file size calculator
 */
export function useMemoizedFileSize(files: File[]) {
  return React.useMemo(() => {
    return files.reduce((total, file) => total + file.size, 0)
  }, [files])
}

/**
 * Optimized upload progress tracker
 */
export function useOptimizedProgressTracker() {
  const progressCache = React.useRef<Map<string, number>>(new Map())
  const lastUpdate = React.useRef<Map<string, number>>(new Map())
  const THROTTLE_DELAY = 100 // ms

  const updateProgress = React.useCallback((fileId: string, progress: number) => {
    const now = Date.now()
    const lastUpdateTime = lastUpdate.current.get(fileId) || 0

    // Throttle updates but always allow 0% and 100%
    if (
      progress === 0 ||
      progress === 100 ||
      now - lastUpdateTime >= THROTTLE_DELAY
    ) {
      progressCache.current.set(fileId, progress)
      lastUpdate.current.set(fileId, now)
      return true // Indicates update was processed
    }

    return false // Indicates update was throttled
  }, [])

  const getProgress = React.useCallback((fileId: string) => {
    return progressCache.current.get(fileId) || 0
  }, [])

  const clearProgress = React.useCallback((fileId: string) => {
    progressCache.current.delete(fileId)
    lastUpdate.current.delete(fileId)
  }, [])

  return { updateProgress, getProgress, clearProgress }
}

/**
 * Memory usage monitor hook
 */
type PerformanceWithMemory = Performance & {
  memory: {
    usedJSHeapSize: number
    totalJSHeapSize: number
    jsHeapSizeLimit: number
  }
}

export function useMemoryMonitor() {
  const [memoryUsage, setMemoryUsage] = React.useState<{
    used: number
    total: number
    percentage: number
  } | null>(null)

  React.useEffect(() => {
    const updateMemoryUsage = () => {
      if ('memory' in performance) {
        const memory = (performance as PerformanceWithMemory).memory
        setMemoryUsage({
          used: memory.usedJSHeapSize,
          total: memory.totalJSHeapSize,
          percentage: (memory.usedJSHeapSize / memory.totalJSHeapSize) * 100
        })
      }
    }

    updateMemoryUsage()
    const interval = setInterval(updateMemoryUsage, 5000) // Update every 5 seconds

    return () => clearInterval(interval)
  }, [])

  return memoryUsage
}

/**
 * Performance metrics collector
 */
export function usePerformanceMetrics() {
  const metrics = React.useRef<{
    renderCount: number
    lastRenderTime: number
    averageRenderTime: number
  }>({
    renderCount: 0,
    lastRenderTime: 0,
    averageRenderTime: 0
  })

  React.useEffect(() => {
    const startTime = performance.now()

    return () => {
      const endTime = performance.now()
      const renderTime = endTime - startTime

      metrics.current.renderCount++
      metrics.current.lastRenderTime = renderTime
      metrics.current.averageRenderTime =
        (metrics.current.averageRenderTime * (metrics.current.renderCount - 1) + renderTime) /
        metrics.current.renderCount
    }
  })

  return metrics.current
}

export default {
  useDebounce,
  useThrottle,
  useMemoizedImagePreview,
  useVirtualizedList,
  useOptimizedFileReader,
  useIntersectionObserver,
  LazyImage,
  useMemoizedFileSize,
  useOptimizedProgressTracker,
  useMemoryMonitor,
  usePerformanceMetrics
}
