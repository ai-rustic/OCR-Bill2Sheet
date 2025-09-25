"use client"

import * as React from "react"
import { AlertTriangle, RefreshCw } from "lucide-react"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { cn } from "@/lib/utils"

/**
 * Props for ErrorBoundary component
 */
interface ErrorBoundaryProps {
  children: React.ReactNode
  fallback?: React.ComponentType<ErrorFallbackProps>
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void
  className?: string
}

/**
 * Props for error fallback component
 */
interface ErrorFallbackProps {
  error: Error
  resetError: () => void
  className?: string
}

/**
 * Error boundary state
 */
interface ErrorBoundaryState {
  hasError: boolean
  error: Error | null
  errorInfo: React.ErrorInfo | null
}

/**
 * Error boundary for upload components
 * Catches and handles runtime errors gracefully
 */
export class UploadErrorBoundary extends React.Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props)
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null
    }
  }

  static getDerivedStateFromError(error: Error): Partial<ErrorBoundaryState> {
    return {
      hasError: true,
      error
    }
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    this.setState({
      error,
      errorInfo
    })

    // Log error to console in development
    if (process.env.NODE_ENV === 'development') {
      console.error('Upload component error:', error, errorInfo)
    }

    // Call external error handler
    this.props.onError?.(error, errorInfo)
  }

  resetError = () => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null
    })
  }

  render() {
    if (this.state.hasError && this.state.error) {
      const FallbackComponent = this.props.fallback || DefaultErrorFallback

      return (
        <FallbackComponent
          error={this.state.error}
          resetError={this.resetError}
          className={this.props.className}
        />
      )
    }

    return this.props.children
  }
}

/**
 * Default error fallback component
 */
function DefaultErrorFallback({ error, resetError, className }: ErrorFallbackProps) {
  return (
    <Card className={cn("border-red-200 bg-red-50", className)}>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2 text-red-700">
          <AlertTriangle className="h-5 w-5" />
          <span>Upload Error</span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <Alert variant="destructive">
          <AlertTriangle className="h-4 w-4" />
          <AlertTitle>Something went wrong</AlertTitle>
          <AlertDescription>
            {error.message || 'An unexpected error occurred in the upload component.'}
          </AlertDescription>
        </Alert>

        <div className="flex items-center space-x-2">
          <Button onClick={resetError} size="sm">
            <RefreshCw className="h-4 w-4 mr-2" />
            Try Again
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => window.location.reload()}
          >
            Reload Page
          </Button>
        </div>

        {process.env.NODE_ENV === 'development' && (
          <details className="text-xs text-red-600 bg-red-100 p-2 rounded border">
            <summary className="cursor-pointer font-medium">Error Details (Development)</summary>
            <pre className="mt-2 whitespace-pre-wrap break-words">
              {error.stack}
            </pre>
          </details>
        )}
      </CardContent>
    </Card>
  )
}

/**
 * Compact error fallback for smaller spaces
 */
export function CompactErrorFallback({ error, resetError, className }: ErrorFallbackProps) {
  return (
    <div className={cn("p-4 border border-red-200 bg-red-50 rounded-lg", className)}>
      <div className="flex items-center space-x-2 text-red-700 mb-2">
        <AlertTriangle className="h-4 w-4" />
        <span className="text-sm font-medium">Upload Error</span>
      </div>
      <p className="text-sm text-red-600 mb-3">
        {error.message || 'Something went wrong'}
      </p>
      <Button onClick={resetError} size="sm" variant="outline">
        <RefreshCw className="h-3 w-3 mr-1" />
        Retry
      </Button>
    </div>
  )
}

/**
 * Hook for handling upload-specific errors
 */
export function useUploadErrorHandler() {
  const [errors, setErrors] = React.useState<string[]>([])
  const [criticalError, setCriticalError] = React.useState<Error | null>(null)

  const addError = React.useCallback((error: string) => {
    setErrors(prev => [...prev, error])
  }, [])

  const removeError = React.useCallback((index: number) => {
    setErrors(prev => prev.filter((_, i) => i !== index))
  }, [])

  const clearErrors = React.useCallback(() => {
    setErrors([])
  }, [])

  const handleCriticalError = React.useCallback((error: Error) => {
    setCriticalError(error)
    console.error('Critical upload error:', error)
  }, [])

  const clearCriticalError = React.useCallback(() => {
    setCriticalError(null)
  }, [])

  const handleFileError = React.useCallback((fileName: string, errorMessage: string) => {
    addError(`${fileName}: ${errorMessage}`)
  }, [addError])

  const handleValidationErrors = React.useCallback((validationErrors: string[]) => {
    validationErrors.forEach(addError)
  }, [addError])

  return {
    errors,
    criticalError,
    addError,
    removeError,
    clearErrors,
    handleCriticalError,
    clearCriticalError,
    handleFileError,
    handleValidationErrors,
    hasErrors: errors.length > 0 || criticalError !== null
  }
}

/**
 * Enhanced error reporting hook
 */
export function useErrorReporting(options?: {
  onError?: (error: Error, context: string) => void
  maxRetries?: number
}) {
  const { onError, maxRetries = 3 } = options || {}
  const [retryCount, setRetryCount] = React.useState(0)

  const reportError = React.useCallback((error: Error, context: string) => {
    console.error(`Upload error in ${context}:`, error)
    onError?.(error, context)
  }, [onError])

  const withErrorHandling = React.useCallback(
    <T extends unknown[], R>(
      fn: (...args: T) => R | Promise<R>,
      context: string
    ) => {
      return async (...args: T): Promise<R | null> => {
        try {
          const result = await fn(...args)
          setRetryCount(0) // Reset on success
          return result
        } catch (error) {
          const err = error instanceof Error ? error : new Error(String(error))
          reportError(err, context)

          if (retryCount < maxRetries) {
            setRetryCount(prev => prev + 1)
            // Exponential backoff
            await new Promise(resolve => setTimeout(resolve, Math.pow(2, retryCount) * 1000))
            return withErrorHandling(fn, context)(...args)
          }

          return null
        }
      }
    },
    [reportError, retryCount, maxRetries]
  )

  return {
    reportError,
    withErrorHandling,
    retryCount,
    canRetry: retryCount < maxRetries
  }
}

/**
 * Network error detection and handling
 */
export function useNetworkErrorHandler() {
  const [isOnline, setIsOnline] = React.useState(typeof navigator !== 'undefined' ? navigator.onLine : true)
  const [networkErrors, setNetworkErrors] = React.useState<string[]>([])

  React.useEffect(() => {
    const handleOnline = () => {
      setIsOnline(true)
      setNetworkErrors([])
    }

    const handleOffline = () => {
      setIsOnline(false)
      setNetworkErrors(['Network connection lost. Please check your internet connection.'])
    }

    window.addEventListener('online', handleOnline)
    window.addEventListener('offline', handleOffline)

    return () => {
      window.removeEventListener('online', handleOnline)
      window.removeEventListener('offline', handleOffline)
    }
  }, [])

  const handleNetworkError = React.useCallback((error: Error) => {
    if (!isOnline) {
      setNetworkErrors(['Upload failed: No internet connection'])
    } else if (error.message.includes('Failed to fetch') || error.message.includes('Network')) {
      setNetworkErrors(['Upload failed: Network error. Please try again.'])
    } else {
      setNetworkErrors(['Upload failed: Server error. Please try again later.'])
    }
  }, [isOnline])

  return {
    isOnline,
    networkErrors,
    handleNetworkError,
    clearNetworkErrors: () => setNetworkErrors([])
  }
}

export default UploadErrorBoundary
