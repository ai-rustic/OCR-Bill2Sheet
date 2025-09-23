"use client"

import * as React from "react"
import { cn } from "@/lib/utils"

/**
 * Upload component theme configuration
 */
export interface UploadTheme {
  /** Color scheme for the upload components */
  colors: {
    primary: string
    secondary: string
    success: string
    warning: string
    error: string
    info: string
    neutral: string
  }
  /** Border radius settings */
  borderRadius: {
    small: string
    medium: string
    large: string
    full: string
  }
  /** Spacing scale */
  spacing: {
    xs: string
    sm: string
    md: string
    lg: string
    xl: string
  }
  /** Typography settings */
  typography: {
    fontFamily: string
    fontSize: {
      xs: string
      sm: string
      base: string
      lg: string
      xl: string
    }
    fontWeight: {
      normal: string
      medium: string
      semibold: string
      bold: string
    }
  }
  /** Animation settings */
  animations: {
    duration: {
      fast: string
      normal: string
      slow: string
    }
    easing: {
      default: string
      bounce: string
      elastic: string
    }
  }
}

/**
 * Default Shadcn-compatible theme
 */
export const DEFAULT_UPLOAD_THEME: UploadTheme = {
  colors: {
    primary: "hsl(var(--primary))",
    secondary: "hsl(var(--secondary))",
    success: "hsl(142 76% 36%)", // Green-600
    warning: "hsl(38 92% 50%)", // Amber-500
    error: "hsl(var(--destructive))",
    info: "hsl(217 91% 60%)", // Blue-500
    neutral: "hsl(var(--muted))"
  },
  borderRadius: {
    small: "calc(var(--radius) - 2px)",
    medium: "var(--radius)",
    large: "calc(var(--radius) + 2px)",
    full: "9999px"
  },
  spacing: {
    xs: "0.25rem", // 1
    sm: "0.5rem",  // 2
    md: "0.75rem", // 3
    lg: "1rem",    // 4
    xl: "1.5rem"   // 6
  },
  typography: {
    fontFamily: "var(--font-sans)",
    fontSize: {
      xs: "0.75rem",
      sm: "0.875rem",
      base: "1rem",
      lg: "1.125rem",
      xl: "1.25rem"
    },
    fontWeight: {
      normal: "400",
      medium: "500",
      semibold: "600",
      bold: "700"
    }
  },
  animations: {
    duration: {
      fast: "150ms",
      normal: "200ms",
      slow: "300ms"
    },
    easing: {
      default: "cubic-bezier(0.4, 0, 0.2, 1)",
      bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
      elastic: "cubic-bezier(0.175, 0.885, 0.32, 1.275)"
    }
  }
}

/**
 * Alternative theme variants
 */
export const UPLOAD_THEME_VARIANTS = {
  minimal: {
    ...DEFAULT_UPLOAD_THEME,
    colors: {
      ...DEFAULT_UPLOAD_THEME.colors,
      primary: "hsl(220 13% 91%)", // Neutral-200
      secondary: "hsl(220 13% 91%)"
    },
    borderRadius: {
      small: "0.125rem",
      medium: "0.25rem",
      large: "0.375rem",
      full: "9999px"
    }
  },
  modern: {
    ...DEFAULT_UPLOAD_THEME,
    borderRadius: {
      small: "0.5rem",
      medium: "0.75rem",
      large: "1rem",
      full: "9999px"
    },
    animations: {
      duration: {
        fast: "100ms",
        normal: "150ms",
        slow: "250ms"
      },
      easing: {
        default: "cubic-bezier(0.25, 0.46, 0.45, 0.94)",
        bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
        elastic: "cubic-bezier(0.175, 0.885, 0.32, 1.275)"
      }
    }
  },
  playful: {
    ...DEFAULT_UPLOAD_THEME,
    colors: {
      ...DEFAULT_UPLOAD_THEME.colors,
      primary: "hsl(262 83% 58%)", // Purple-500
      success: "hsl(142 76% 36%)",  // Green-600
      warning: "hsl(38 92% 50%)",   // Amber-500
      error: "hsl(0 84% 60%)",      // Red-500
      info: "hsl(199 89% 48%)"      // Sky-500
    },
    borderRadius: {
      small: "0.5rem",
      medium: "1rem",
      large: "1.5rem",
      full: "9999px"
    },
    animations: {
      duration: {
        fast: "200ms",
        normal: "300ms",
        slow: "500ms"
      },
      easing: {
        default: "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
        bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
        elastic: "cubic-bezier(0.25, 0.46, 0.45, 0.94)"
      }
    }
  }
} as const

/**
 * Theme context for upload components
 */
const UploadThemeContext = React.createContext<UploadTheme>(DEFAULT_UPLOAD_THEME)

/**
 * Theme provider component
 */
export function UploadThemeProvider({
  theme = DEFAULT_UPLOAD_THEME,
  children
}: {
  theme?: UploadTheme
  children: React.ReactNode
}) {
  return (
    <UploadThemeContext.Provider value={theme}>
      {children}
    </UploadThemeContext.Provider>
  )
}

/**
 * Hook to access upload theme
 */
export function useUploadTheme() {
  const context = React.useContext(UploadThemeContext)
  if (!context) {
    return DEFAULT_UPLOAD_THEME
  }
  return context
}

/**
 * Themed component wrapper
 */
export function withUploadTheme<P extends object>(
  Component: React.ComponentType<P>,
  defaultTheme?: UploadTheme
) {
  return React.forwardRef<any, P & { theme?: UploadTheme }>((props, ref) => {
    const { theme: propTheme, ...otherProps } = props
    const contextTheme = useUploadTheme()
    const effectiveTheme = propTheme || defaultTheme || contextTheme

    return (
      <UploadThemeProvider theme={effectiveTheme}>
        <Component {...(otherProps as P)} ref={ref} />
      </UploadThemeProvider>
    )
  })
}

/**
 * Utility functions for theme-based styling
 */
export const themeUtils = {
  /**
   * Get themed color class
   */
  getColorClass: (theme: UploadTheme, colorName: keyof UploadTheme['colors'], variant: 'bg' | 'text' | 'border' = 'text') => {
    const colorMap = {
      primary: variant === 'bg' ? 'bg-primary' : variant === 'border' ? 'border-primary' : 'text-primary',
      secondary: variant === 'bg' ? 'bg-secondary' : variant === 'border' ? 'border-secondary' : 'text-secondary',
      success: variant === 'bg' ? 'bg-green-600' : variant === 'border' ? 'border-green-600' : 'text-green-600',
      warning: variant === 'bg' ? 'bg-amber-500' : variant === 'border' ? 'border-amber-500' : 'text-amber-500',
      error: variant === 'bg' ? 'bg-destructive' : variant === 'border' ? 'border-destructive' : 'text-destructive',
      info: variant === 'bg' ? 'bg-blue-500' : variant === 'border' ? 'border-blue-500' : 'text-blue-500',
      neutral: variant === 'bg' ? 'bg-muted' : variant === 'border' ? 'border-muted' : 'text-muted-foreground'
    }
    return colorMap[colorName]
  },

  /**
   * Get themed spacing class
   */
  getSpacingClass: (theme: UploadTheme, size: keyof UploadTheme['spacing'], type: 'p' | 'm' | 'gap' | 'space' = 'p') => {
    const spacingMap = {
      xs: type === 'p' ? 'p-1' : type === 'm' ? 'm-1' : type === 'gap' ? 'gap-1' : 'space-x-1',
      sm: type === 'p' ? 'p-2' : type === 'm' ? 'm-2' : type === 'gap' ? 'gap-2' : 'space-x-2',
      md: type === 'p' ? 'p-3' : type === 'm' ? 'm-3' : type === 'gap' ? 'gap-3' : 'space-x-3',
      lg: type === 'p' ? 'p-4' : type === 'm' ? 'm-4' : type === 'gap' ? 'gap-4' : 'space-x-4',
      xl: type === 'p' ? 'p-6' : type === 'm' ? 'm-6' : type === 'gap' ? 'gap-6' : 'space-x-6'
    }
    return spacingMap[size]
  },

  /**
   * Get themed border radius class
   */
  getRadiusClass: (theme: UploadTheme, size: keyof UploadTheme['borderRadius']) => {
    const radiusMap = {
      small: 'rounded-sm',
      medium: 'rounded-md',
      large: 'rounded-lg',
      full: 'rounded-full'
    }
    return radiusMap[size]
  },

  /**
   * Get themed animation class
   */
  getAnimationClass: (theme: UploadTheme, type: 'fade' | 'scale' | 'slide' | 'bounce' = 'fade') => {
    const animationMap = {
      fade: 'transition-opacity duration-200 ease-in-out',
      scale: 'transition-transform duration-200 ease-in-out hover:scale-105',
      slide: 'transition-transform duration-200 ease-in-out',
      bounce: 'transition-all duration-300 ease-bounce'
    }
    return animationMap[type]
  },

  /**
   * Create custom CSS variables from theme
   */
  createCSSVariables: (theme: UploadTheme) => ({
    '--upload-primary': theme.colors.primary,
    '--upload-secondary': theme.colors.secondary,
    '--upload-success': theme.colors.success,
    '--upload-warning': theme.colors.warning,
    '--upload-error': theme.colors.error,
    '--upload-info': theme.colors.info,
    '--upload-neutral': theme.colors.neutral,
    '--upload-radius-sm': theme.borderRadius.small,
    '--upload-radius-md': theme.borderRadius.medium,
    '--upload-radius-lg': theme.borderRadius.large,
    '--upload-radius-full': theme.borderRadius.full,
    '--upload-duration-fast': theme.animations.duration.fast,
    '--upload-duration-normal': theme.animations.duration.normal,
    '--upload-duration-slow': theme.animations.duration.slow
  } as React.CSSProperties)
}

/**
 * Styled components using theme
 */
export const ThemedComponents = {
  Container: React.forwardRef<
    HTMLDivElement,
    React.HTMLAttributes<HTMLDivElement> & { variant?: 'default' | 'elevated' | 'outlined' }
  >(({ className, variant = 'default', style, ...props }, ref) => {
    const theme = useUploadTheme()

    const variantClasses = {
      default: "bg-background border-border",
      elevated: "bg-card shadow-md border-border",
      outlined: "bg-background border-2 border-border"
    }

    return (
      <div
        ref={ref}
        className={cn(
          "rounded-md border transition-colors",
          variantClasses[variant],
          className
        )}
        style={{
          ...themeUtils.createCSSVariables(theme),
          ...style
        }}
        {...props}
      />
    )
  }),

  StatusIndicator: React.forwardRef<
    HTMLDivElement,
    React.HTMLAttributes<HTMLDivElement> & {
      status: 'uploading' | 'completed' | 'error' | 'pending'
      size?: 'sm' | 'md' | 'lg'
    }
  >(({ className, status, size = 'md', ...props }, ref) => {
    const theme = useUploadTheme()

    const statusColors = {
      uploading: themeUtils.getColorClass(theme, 'info', 'bg'),
      completed: themeUtils.getColorClass(theme, 'success', 'bg'),
      error: themeUtils.getColorClass(theme, 'error', 'bg'),
      pending: themeUtils.getColorClass(theme, 'neutral', 'bg')
    }

    const sizes = {
      sm: 'w-2 h-2',
      md: 'w-3 h-3',
      lg: 'w-4 h-4'
    }

    return (
      <div
        ref={ref}
        className={cn(
          "rounded-full transition-colors",
          statusColors[status],
          sizes[size],
          status === 'uploading' && 'animate-pulse',
          className
        )}
        {...props}
      />
    )
  })
}

export default {
  UploadThemeProvider,
  useUploadTheme,
  withUploadTheme,
  themeUtils,
  ThemedComponents,
  DEFAULT_UPLOAD_THEME,
  UPLOAD_THEME_VARIANTS
}