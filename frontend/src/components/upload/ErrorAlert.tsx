"use client"

import * as React from "react"
import { AlertTriangle, X } from "lucide-react"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import type { ErrorAlertProps } from "@/types/upload"
import { formatValidationErrors } from "@/utils/fileFormatting"

/**
 * Error alert component for validation errors
 * Shows dismissible alerts for upload errors
 */
export function ErrorAlert({
  errors,
  onDismiss,
  isVisible,
  className,
  ...props
}: ErrorAlertProps & { className?: string }) {
  if (!isVisible || errors.length === 0) {
    return null
  }

  const hasMultipleErrors = errors.length > 1
  const displayTitle = hasMultipleErrors
    ? `${errors.length} Upload Errors`
    : "Upload Error"

  return (
    <Alert
      variant="destructive"
      className={cn("relative animate-in slide-in-from-top-2", className)}
      {...props}
    >
      <AlertTriangle className="h-4 w-4" />
      <AlertTitle className="flex items-center justify-between">
        <span>{displayTitle}</span>
        <Button
          variant="ghost"
          size="sm"
          className="h-6 w-6 p-0 hover:bg-red-100"
          onClick={onDismiss}
          aria-label="Dismiss errors"
        >
          <X className="h-3 w-3" />
        </Button>
      </AlertTitle>
      <AlertDescription>
        {hasMultipleErrors ? (
          <ul className="list-disc list-inside space-y-1">
            {errors.map((error, index) => (
              <li key={index} className="text-sm">
                {error}
              </li>
            ))}
          </ul>
        ) : (
          <span className="text-sm">{errors[0]}</span>
        )}
      </AlertDescription>
    </Alert>
  )
}

/**
 * Compact error alert for smaller spaces
 */
export function CompactErrorAlert({
  errors,
  onDismiss,
  isVisible,
  className,
  ...props
}: ErrorAlertProps & { className?: string }) {
  if (!isVisible || errors.length === 0) {
    return null
  }

  const errorSummary = formatValidationErrors(errors)

  return (
    <div
      className={cn(
        "flex items-center justify-between bg-red-50 border border-red-200 rounded-md px-3 py-2 text-sm text-red-700 animate-in slide-in-from-top-1",
        className
      )}
      {...props}
    >
      <div className="flex items-center space-x-2">
        <AlertTriangle className="h-4 w-4 text-red-500" />
        <span className="truncate">{errorSummary}</span>
      </div>
      <Button
        variant="ghost"
        size="sm"
        className="h-5 w-5 p-0 hover:bg-red-100 text-red-500 hover:text-red-700"
        onClick={onDismiss}
        aria-label="Dismiss error"
      >
        <X className="h-3 w-3" />
      </Button>
    </div>
  )
}

/**
 * Toast-style error alert that auto-dismisses
 */
export function ToastErrorAlert({
  errors,
  onDismiss,
  isVisible,
  autoDismiss = true,
  dismissAfter = 5000,
  className,
  ...props
}: ErrorAlertProps & {
  autoDismiss?: boolean
  dismissAfter?: number
  className?: string
}) {
  const timeoutRef = React.useRef<NodeJS.Timeout | null>(null)

  React.useEffect(() => {
    if (isVisible && autoDismiss && dismissAfter > 0) {
      timeoutRef.current = setTimeout(() => {
        onDismiss()
      }, dismissAfter)
    }

    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current)
      }
    }
  }, [isVisible, autoDismiss, dismissAfter, onDismiss])

  if (!isVisible || errors.length === 0) {
    return null
  }

  return (
    <div
      className={cn(
        "fixed top-4 right-4 z-50 max-w-md w-full animate-in slide-in-from-top-2 duration-300",
        className
      )}
      {...props}
    >
      <Alert variant="destructive" className="shadow-lg border-red-300">
        <AlertTriangle className="h-4 w-4" />
        <AlertTitle className="flex items-center justify-between">
          <span>Upload Error</span>
          <Button
            variant="ghost"
            size="sm"
            className="h-6 w-6 p-0 hover:bg-red-100"
            onClick={onDismiss}
            aria-label="Dismiss errors"
          >
            <X className="h-3 w-3" />
          </Button>
        </AlertTitle>
        <AlertDescription>
          {errors.length === 1 ? (
            <span>{errors[0]}</span>
          ) : (
            <div className="space-y-1">
              <span>Multiple errors occurred:</span>
              <ul className="list-disc list-inside space-y-1 ml-2">
                {errors.slice(0, 3).map((error, index) => (
                  <li key={index} className="text-sm">
                    {error}
                  </li>
                ))}
                {errors.length > 3 && (
                  <li className="text-sm text-red-600">
                    ...and {errors.length - 3} more
                  </li>
                )}
              </ul>
            </div>
          )}
        </AlertDescription>
      </Alert>
    </div>
  )
}

/**
 * Inline error alert for form-like displays
 */
export function InlineErrorAlert({
  errors,
  onDismiss,
  isVisible,
  showDismiss = true,
  className,
  ...props
}: ErrorAlertProps & {
  showDismiss?: boolean
  className?: string
}) {
  if (!isVisible || errors.length === 0) {
    return null
  }

  return (
    <div
      className={cn(
        "flex items-start space-x-2 p-3 bg-red-50 border border-red-200 rounded-md text-red-700 text-sm animate-in fade-in duration-200",
        className
      )}
      {...props}
    >
      <AlertTriangle className="h-4 w-4 text-red-500 mt-0.5 flex-shrink-0" />
      <div className="flex-1 min-w-0">
        {errors.length === 1 ? (
          <span>{errors[0]}</span>
        ) : (
          <div className="space-y-1">
            {errors.map((error, index) => (
              <div key={index}>{error}</div>
            ))}
          </div>
        )}
      </div>
      {showDismiss && (
        <Button
          variant="ghost"
          size="sm"
          className="h-5 w-5 p-0 hover:bg-red-100 text-red-500 hover:text-red-700 flex-shrink-0"
          onClick={onDismiss}
          aria-label="Dismiss errors"
        >
          <X className="h-3 w-3" />
        </Button>
      )}
    </div>
  )
}

export default ErrorAlert