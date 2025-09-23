"use client"

import * as React from "react"
import { cn } from "@/lib/utils"

/**
 * Responsive configuration for upload components
 */
export interface ResponsiveConfig {
  /** Breakpoints for different screen sizes */
  breakpoints: {
    sm: number    // Small devices (phones)
    md: number    // Medium devices (tablets)
    lg: number    // Large devices (desktops)
    xl: number    // Extra large devices
  }
  /** Layout adjustments per breakpoint */
  layouts: {
    mobile: {
      columns: number
      uploadAreaHeight: string
      showDetailedStats: boolean
      compactMode: boolean
    }
    tablet: {
      columns: number
      uploadAreaHeight: string
      showDetailedStats: boolean
      compactMode: boolean
    }
    desktop: {
      columns: number
      uploadAreaHeight: string
      showDetailedStats: boolean
      compactMode: boolean
    }
  }
}

/**
 * Default responsive configuration
 */
export const DEFAULT_RESPONSIVE_CONFIG: ResponsiveConfig = {
  breakpoints: {
    sm: 640,
    md: 768,
    lg: 1024,
    xl: 1280
  },
  layouts: {
    mobile: {
      columns: 1,
      uploadAreaHeight: 'min-h-32',
      showDetailedStats: false,
      compactMode: true
    },
    tablet: {
      columns: 2,
      uploadAreaHeight: 'min-h-40',
      showDetailedStats: true,
      compactMode: false
    },
    desktop: {
      columns: 3,
      uploadAreaHeight: 'min-h-48',
      showDetailedStats: true,
      compactMode: false
    }
  }
}

/**
 * Hook for responsive behavior
 */
export function useResponsive(config: ResponsiveConfig = DEFAULT_RESPONSIVE_CONFIG) {
  const [screenSize, setScreenSize] = React.useState<'mobile' | 'tablet' | 'desktop'>('desktop')
  const [windowSize, setWindowSize] = React.useState({ width: 0, height: 0 })

  React.useEffect(() => {
    const handleResize = () => {
      const width = window.innerWidth
      const height = window.innerHeight

      setWindowSize({ width, height })

      if (width < config.breakpoints.md) {
        setScreenSize('mobile')
      } else if (width < config.breakpoints.lg) {
        setScreenSize('tablet')
      } else {
        setScreenSize('desktop')
      }
    }

    // Set initial size
    handleResize()

    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [config.breakpoints])

  const currentLayout = config.layouts[screenSize]
  const isMobile = screenSize === 'mobile'
  const isTablet = screenSize === 'tablet'
  const isDesktop = screenSize === 'desktop'

  return {
    screenSize,
    windowSize,
    isMobile,
    isTablet,
    isDesktop,
    layout: currentLayout,
    columns: currentLayout.columns,
    uploadAreaHeight: currentLayout.uploadAreaHeight,
    showDetailedStats: currentLayout.showDetailedStats,
    compactMode: currentLayout.compactMode
  }
}

/**
 * Responsive grid container component
 */
export function ResponsiveGrid({
  children,
  columns = 3,
  gap = 4,
  className,
  ...props
}: {
  children: React.ReactNode
  columns?: number
  gap?: number
  className?: string
} & React.HTMLAttributes<HTMLDivElement>) {
  const gridClasses = cn(
    "grid gap-" + gap,
    columns === 1 && "grid-cols-1",
    columns === 2 && "grid-cols-1 sm:grid-cols-2",
    columns === 3 && "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3",
    columns === 4 && "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4",
    columns === 5 && "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5",
    className
  )

  return (
    <div className={gridClasses} {...props}>
      {children}
    </div>
  )
}

/**
 * Mobile-optimized upload area component
 */
export function MobileUploadArea({
  children,
  className,
  ...props
}: {
  children: React.ReactNode
  className?: string
} & React.HTMLAttributes<HTMLDivElement>) {
  const { isMobile } = useResponsive()

  return (
    <div
      className={cn(
        "w-full",
        isMobile && "touch-manipulation", // Improve touch responsiveness
        className
      )}
      style={{
        // Prevent zoom on double tap for mobile
        touchAction: isMobile ? 'manipulation' : 'auto'
      }}
      {...props}
    >
      {children}
    </div>
  )
}

/**
 * Responsive layout wrapper
 */
export function ResponsiveLayout({
  children,
  config = DEFAULT_RESPONSIVE_CONFIG,
  className,
  ...props
}: {
  children: React.ReactNode
  config?: ResponsiveConfig
  className?: string
} & React.HTMLAttributes<HTMLDivElement>) {
  const responsive = useResponsive(config)

  const layoutClasses = cn(
    "w-full mx-auto",
    responsive.isMobile && "px-4 py-2",
    responsive.isTablet && "px-6 py-4 max-w-4xl",
    responsive.isDesktop && "px-8 py-6 max-w-6xl",
    className
  )

  return (
    <div className={layoutClasses} {...props}>
      {children}
    </div>
  )
}

/**
 * Touch-friendly button wrapper for mobile
 */
export function TouchButton({
  children,
  className,
  ...props
}: {
  children: React.ReactNode
  className?: string
} & React.ButtonHTMLAttributes<HTMLButtonElement>) {
  const { isMobile } = useResponsive()

  return (
    <button
      className={cn(
        isMobile && "min-h-[44px] min-w-[44px]", // iOS touch target guidelines
        className
      )}
      {...props}
    >
      {children}
    </button>
  )
}

/**
 * Responsive text size utility
 */
export function useResponsiveText() {
  const { isMobile, isTablet } = useResponsive()

  const getTextSize = (base: string) => {
    if (isMobile) {
      switch (base) {
        case 'xs': return 'text-xs'
        case 'sm': return 'text-sm'
        case 'base': return 'text-sm'
        case 'lg': return 'text-base'
        case 'xl': return 'text-lg'
        case '2xl': return 'text-xl'
        default: return base
      }
    }
    if (isTablet) {
      switch (base) {
        case 'xs': return 'text-xs'
        case 'sm': return 'text-sm'
        case 'base': return 'text-base'
        case 'lg': return 'text-lg'
        case 'xl': return 'text-xl'
        case '2xl': return 'text-2xl'
        default: return base
      }
    }
    return base // Desktop uses base sizes
  }

  return { getTextSize }
}

/**
 * Responsive spacing utility
 */
export function useResponsiveSpacing() {
  const { isMobile, isTablet } = useResponsive()

  const getSpacing = (base: number) => {
    if (isMobile) {
      return Math.max(1, Math.floor(base * 0.75)) // 25% less spacing on mobile
    }
    if (isTablet) {
      return Math.max(1, Math.floor(base * 0.875)) // 12.5% less spacing on tablet
    }
    return base // Desktop uses base spacing
  }

  return { getSpacing }
}

/**
 * Responsive container query hook
 */
export function useContainerQuery(ref: React.RefObject<HTMLElement>) {
  const [containerWidth, setContainerWidth] = React.useState(0)

  React.useEffect(() => {
    const element = ref.current
    if (!element) return

    const resizeObserver = new ResizeObserver(entries => {
      for (const entry of entries) {
        setContainerWidth(entry.contentRect.width)
      }
    })

    resizeObserver.observe(element)
    return () => resizeObserver.disconnect()
  }, [ref])

  const getContainerColumns = () => {
    if (containerWidth < 400) return 1
    if (containerWidth < 600) return 2
    if (containerWidth < 800) return 3
    if (containerWidth < 1000) return 4
    return 5
  }

  return {
    containerWidth,
    columns: getContainerColumns(),
    isNarrow: containerWidth < 400,
    isMedium: containerWidth >= 400 && containerWidth < 800,
    isWide: containerWidth >= 800
  }
}

/**
 * Responsive image sizing
 */
export function useResponsiveImageSize() {
  const { isMobile, isTablet } = useResponsive()

  const getImageSize = (baseSize: 'small' | 'medium' | 'large') => {
    if (isMobile) {
      switch (baseSize) {
        case 'small': return { width: '80px', height: '80px' }
        case 'medium': return { width: '120px', height: '120px' }
        case 'large': return { width: '160px', height: '160px' }
      }
    }
    if (isTablet) {
      switch (baseSize) {
        case 'small': return { width: '100px', height: '100px' }
        case 'medium': return { width: '150px', height: '150px' }
        case 'large': return { width: '200px', height: '200px' }
      }
    }
    // Desktop
    switch (baseSize) {
      case 'small': return { width: '120px', height: '120px' }
      case 'medium': return { width: '180px', height: '180px' }
      case 'large': return { width: '240px', height: '240px' }
    }
  }

  return { getImageSize }
}

export default {
  useResponsive,
  ResponsiveGrid,
  MobileUploadArea,
  ResponsiveLayout,
  TouchButton,
  useResponsiveText,
  useResponsiveSpacing,
  useContainerQuery,
  useResponsiveImageSize,
  DEFAULT_RESPONSIVE_CONFIG
}