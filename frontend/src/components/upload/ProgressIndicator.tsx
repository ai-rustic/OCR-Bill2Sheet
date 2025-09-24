"use client"

import * as React from "react"
import { CheckCircle, XCircle, Loader2 } from "lucide-react"
import { Progress } from "@/components/ui/progress"
import { cn } from "@/lib/utils"
import { ProgressIndicatorProps, UploadStatus } from "@/types/upload"
import { formatProgress } from "@/utils/fileFormatting"
import { useUploadTheme, themeUtils } from "./UploadTheme"

/**
 * Progress indicator component for upload status
 * Shows progress bar, status icon, and error messages
 */
export function ProgressIndicator({
  progress,
  status,
  fileName,
  error,
  className,
  ...props
}: ProgressIndicatorProps & { className?: string }) {
  const theme = useUploadTheme()
  const getStatusIcon = () => {
    const iconClasses = "h-4 w-4"
    switch (status) {
      case UploadStatus.UPLOADING:
        return <Loader2 className={cn(iconClasses, "animate-spin", themeUtils.getColorClass(theme, 'info'))} />
      case UploadStatus.COMPLETED:
        return <CheckCircle className={cn(iconClasses, themeUtils.getColorClass(theme, 'success'))} />
      case UploadStatus.ERROR:
        return <XCircle className={cn(iconClasses, themeUtils.getColorClass(theme, 'error'))} />
      default:
        return null
    }
  }

  const getStatusText = () => {
    switch (status) {
      case UploadStatus.UPLOADING:
        return `Uploading... ${formatProgress(progress)}`
      case UploadStatus.COMPLETED:
        return "Upload complete"
      case UploadStatus.ERROR:
        return error || "Upload failed"
      default:
        return ""
    }
  }

  const getProgressBarColor = () => {
    switch (status) {
      case UploadStatus.UPLOADING:
        return themeUtils.getColorClass(theme, 'info', 'bg')
      case UploadStatus.COMPLETED:
        return themeUtils.getColorClass(theme, 'success', 'bg')
      case UploadStatus.ERROR:
        return themeUtils.getColorClass(theme, 'error', 'bg')
      default:
        return themeUtils.getColorClass(theme, 'neutral', 'bg')
    }
  }

  return (
    <div
      className={cn("space-y-2", className)}
      {...props}
    >
      {/* File name and status icon */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-2 min-w-0 flex-1">
          {getStatusIcon()}
          <span className="text-sm font-medium text-gray-900 truncate">
            {fileName}
          </span>
        </div>
        {status === UploadStatus.UPLOADING && (
          <span className="text-xs text-gray-500 ml-2">
            {formatProgress(progress)}
          </span>
        )}
      </div>

      {/* Progress bar */}
      <div className="relative">
        <Progress
          value={progress}
          className={cn(
            "h-2",
            status === UploadStatus.ERROR && "bg-red-100"
          )}
        />
        {/* Custom progress indicator color */}
        <div
          className={cn(
            "absolute top-0 left-0 h-full rounded-full transition-all duration-300",
            getProgressBarColor()
          )}
          style={{
            width: `${Math.min(progress, 100)}%`,
            opacity: status === UploadStatus.ERROR ? 0.7 : 1
          }}
        />
      </div>

      {/* Status text and error message */}
      <div className="flex items-center justify-between">
        <span
          className={cn(
            "text-xs transition-colors",
            status === UploadStatus.UPLOADING && themeUtils.getColorClass(theme, 'info'),
            status === UploadStatus.COMPLETED && themeUtils.getColorClass(theme, 'success'),
            status === UploadStatus.ERROR && themeUtils.getColorClass(theme, 'error'),
            themeUtils.getAnimationClass(theme, 'fade')
          )}
        >
          {getStatusText()}
        </span>
      </div>

      {/* Error details */}
      {status === UploadStatus.ERROR && error && (
        <div className={cn(
          "text-xs px-2 py-1 rounded border transition-all",
          themeUtils.getColorClass(theme, 'error'),
          "bg-red-50 border-red-200",
          themeUtils.getRadiusClass(theme, 'small'),
          themeUtils.getAnimationClass(theme, 'fade')
        )}>
          {error}
        </div>
      )}
    </div>
  )
}

/**
 * Compact progress indicator for smaller spaces
 */
export function CompactProgressIndicator({
  progress,
  status,
  className,
  ...props
}: Omit<ProgressIndicatorProps, 'fileName'> & { className?: string }) {
  const getStatusIcon = () => {
    switch (status) {
      case UploadStatus.UPLOADING:
        return <Loader2 className="h-3 w-3 animate-spin text-blue-500" />
      case UploadStatus.COMPLETED:
        return <CheckCircle className="h-3 w-3 text-green-500" />
      case UploadStatus.ERROR:
        return <XCircle className="h-3 w-3 text-red-500" />
      default:
        return null
    }
  }

  return (
    <div
      className={cn("flex items-center space-x-2", className)}
      {...props}
    >
      {getStatusIcon()}
      <Progress
        value={progress}
        className="h-1 flex-1"
      />
      {status === UploadStatus.UPLOADING && (
        <span className="text-xs text-gray-500 min-w-fit">
          {formatProgress(progress)}
        </span>
      )}
    </div>
  )
}

/**
 * Progress indicator with animation for better UX
 */
export function AnimatedProgressIndicator({
  progress,
  status,
  fileName,
  error,
  className,
  ...props
}: ProgressIndicatorProps & { className?: string }) {
  const [displayProgress, setDisplayProgress] = React.useState(0)

  // Animate progress changes
  React.useEffect(() => {
    const timer = setTimeout(() => {
      setDisplayProgress(progress)
    }, 100)
    return () => clearTimeout(timer)
  }, [progress])

  return (
    <ProgressIndicator
      progress={displayProgress}
      status={status}
      fileName={fileName}
      error={error}
      className={cn("transition-all duration-300", className)}
      {...props}
    />
  )
}

export default ProgressIndicator