"use client"

import * as React from "react"
import { Trash2, RotateCcw, Eye, Download, X } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import { ImagePreviewProps, UploadStatus } from "@/types/upload"
import { ProgressIndicator, CompactProgressIndicator } from "./ProgressIndicator"
import { formatFileSize, formatRelativeTime, getFileTypeDisplayName } from "@/utils/fileFormatting"

/**
 * Individual image preview component with progress and actions
 */
export function ImagePreview({
  image,
  onDelete,
  onRetry,
  size = "medium",
  className,
  ...props
}: ImagePreviewProps & { className?: string }) {
  const [isImageModalOpen, setIsImageModalOpen] = React.useState(false)
  const [imageLoadError, setImageLoadError] = React.useState(false)

  const sizeConfig = {
    small: {
      container: "w-24 h-24",
      image: "w-full h-16",
      card: "p-2",
      text: "text-xs"
    },
    medium: {
      container: "w-48 h-48",
      image: "w-full h-32",
      card: "p-3",
      text: "text-sm"
    },
    large: {
      container: "w-64 h-64",
      image: "w-full h-48",
      card: "p-4",
      text: "text-sm"
    }
  }

  const config = sizeConfig[size]

  const handleImageError = () => {
    setImageLoadError(true)
  }

  const handleDownload = () => {
    if (image.previewUrl) {
      const link = document.createElement('a')
      link.href = image.previewUrl
      link.download = image.fileName
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
    }
  }

  const handleViewImage = () => {
    setIsImageModalOpen(true)
  }

  return (
    <>
      <Card
        className={cn(
          "relative group overflow-hidden transition-all duration-200 hover:shadow-md",
          config.container,
          image.status === UploadStatus.ERROR && "border-red-200 bg-red-50",
          image.status === UploadStatus.COMPLETED && "border-green-200",
          className
        )}
        {...props}
      >
        <CardContent className={cn("relative", config.card)}>
          {/* Image display */}
          <div className={cn("relative overflow-hidden rounded-md", config.image)}>
            {image.previewUrl && !imageLoadError ? (
              <img
                src={image.previewUrl}
                alt={image.fileName}
                className="w-full h-full object-cover transition-transform duration-200 group-hover:scale-105"
                onError={handleImageError}
              />
            ) : (
              <div className="w-full h-full bg-gray-100 flex items-center justify-center">
                <div className="text-center text-gray-500">
                  <div className="text-2xl mb-1">📁</div>
                  <div className={cn("truncate", config.text)}>
                    {getFileTypeDisplayName(image.fileType)}
                  </div>
                </div>
              </div>
            )}

            {/* Overlay actions */}
            <div className="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-30 transition-all duration-200 flex items-center justify-center opacity-0 group-hover:opacity-100">
              <div className="flex space-x-1">
                {image.previewUrl && !imageLoadError && (
                  <Button
                    size="sm"
                    variant="ghost"
                    className="h-8 w-8 p-0 bg-white bg-opacity-90 hover:bg-opacity-100 text-gray-700"
                    onClick={handleViewImage}
                    title="View image"
                  >
                    <Eye className="h-4 w-4" />
                  </Button>
                )}
                {image.status === UploadStatus.COMPLETED && (
                  <Button
                    size="sm"
                    variant="ghost"
                    className="h-8 w-8 p-0 bg-white bg-opacity-90 hover:bg-opacity-100 text-gray-700"
                    onClick={handleDownload}
                    title="Download image"
                  >
                    <Download className="h-4 w-4" />
                  </Button>
                )}
              </div>
            </div>

            {/* Status indicator overlay */}
            {image.status === UploadStatus.UPLOADING && (
              <div className="absolute inset-0 bg-black bg-opacity-20 flex items-center justify-center">
                <div className="bg-white bg-opacity-90 rounded-full p-2">
                  <CompactProgressIndicator
                    progress={image.progress}
                    status={image.status}
                    className="w-16"
                  />
                </div>
              </div>
            )}
          </div>

          {/* File info */}
          <div className="mt-2 space-y-1">
            <div className={cn("font-medium truncate", config.text)}>
              {image.fileName}
            </div>
            <div className={cn("text-gray-500 flex items-center justify-between", config.text)}>
              <span>{formatFileSize(image.fileSize)}</span>
              {image.status === UploadStatus.COMPLETED && (
                <span>{formatRelativeTime(image.uploadedAt)}</span>
              )}
            </div>
          </div>

          {/* Progress indicator for medium/large sizes */}
          {size !== "small" && image.status !== UploadStatus.COMPLETED && (
            <div className="mt-2">
              <ProgressIndicator
                progress={image.progress}
                status={image.status}
                fileName=""
                error={image.error}
              />
            </div>
          )}

          {/* Action buttons */}
          <div className="absolute top-2 right-2 flex space-x-1">
            {image.status === UploadStatus.ERROR && onRetry && (
              <Button
                size="sm"
                variant="ghost"
                className="h-6 w-6 p-0 bg-white bg-opacity-90 hover:bg-opacity-100 text-blue-600"
                onClick={() => onRetry(image.id)}
                title="Retry upload"
              >
                <RotateCcw className="h-3 w-3" />
              </Button>
            )}
            <Button
              size="sm"
              variant="ghost"
              className="h-6 w-6 p-0 bg-white bg-opacity-90 hover:bg-opacity-100 text-red-600"
              onClick={() => onDelete(image.id)}
              title="Delete image"
            >
              <Trash2 className="h-3 w-3" />
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Image modal */}
      {isImageModalOpen && image.previewUrl && (
        <ImageModal
          src={image.previewUrl}
          alt={image.fileName}
          onClose={() => setIsImageModalOpen(false)}
        />
      )}
    </>
  )
}

/**
 * List view image preview for compact display
 */
export function ListImagePreview({
  image,
  onDelete,
  onRetry,
  className,
  ...props
}: Omit<ImagePreviewProps, 'size'> & { className?: string }) {
  const [imageLoadError, setImageLoadError] = React.useState(false)

  const handleImageError = () => {
    setImageLoadError(true)
  }

  return (
    <div
      className={cn(
        "flex items-center space-x-3 p-3 rounded-lg border transition-colors duration-200",
        image.status === UploadStatus.ERROR && "border-red-200 bg-red-50",
        image.status === UploadStatus.COMPLETED && "border-green-200 bg-green-50",
        image.status === UploadStatus.UPLOADING && "border-blue-200 bg-blue-50",
        className
      )}
      {...props}
    >
      {/* Thumbnail */}
      <div className="w-12 h-12 rounded-md overflow-hidden flex-shrink-0">
        {image.previewUrl && !imageLoadError ? (
          <img
            src={image.previewUrl}
            alt={image.fileName}
            className="w-full h-full object-cover"
            onError={handleImageError}
          />
        ) : (
          <div className="w-full h-full bg-gray-100 flex items-center justify-center">
            <span className="text-xl">📁</span>
          </div>
        )}
      </div>

      {/* File info */}
      <div className="flex-1 min-w-0">
        <div className="text-sm font-medium text-gray-900 truncate">
          {image.fileName}
        </div>
        <div className="text-xs text-gray-500 flex items-center space-x-2">
          <span>{formatFileSize(image.fileSize)}</span>
          <span>•</span>
          <span>{getFileTypeDisplayName(image.fileType)}</span>
          {image.status === UploadStatus.COMPLETED && (
            <>
              <span>•</span>
              <span>{formatRelativeTime(image.uploadedAt)}</span>
            </>
          )}
        </div>
        {image.status !== UploadStatus.COMPLETED && (
          <div className="mt-1">
            <CompactProgressIndicator
              progress={image.progress}
              status={image.status}
              error={image.error}
            />
          </div>
        )}
      </div>

      {/* Actions */}
      <div className="flex items-center space-x-1">
        {image.status === UploadStatus.ERROR && onRetry && (
          <Button
            size="sm"
            variant="ghost"
            className="h-8 w-8 p-0 text-blue-600"
            onClick={() => onRetry(image.id)}
            title="Retry upload"
          >
            <RotateCcw className="h-4 w-4" />
          </Button>
        )}
        <Button
          size="sm"
          variant="ghost"
          className="h-8 w-8 p-0 text-red-600"
          onClick={() => onDelete(image.id)}
          title="Delete image"
        >
          <Trash2 className="h-4 w-4" />
        </Button>
      </div>
    </div>
  )
}

/**
 * Full-screen image modal
 */
function ImageModal({
  src,
  alt,
  onClose
}: {
  src: string
  alt: string
  onClose: () => void
}) {
  React.useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose()
      }
    }

    document.addEventListener('keydown', handleEscape)
    return () => document.removeEventListener('keydown', handleEscape)
  }, [onClose])

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-75 p-4"
      onClick={onClose}
    >
      <div className="relative max-w-4xl max-h-full">
        <Button
          size="sm"
          variant="ghost"
          className="absolute top-2 right-2 z-10 h-8 w-8 p-0 bg-white bg-opacity-90 hover:bg-opacity-100 text-gray-700"
          onClick={onClose}
        >
          <X className="h-4 w-4" />
        </Button>
        <img
          src={src}
          alt={alt}
          className="max-w-full max-h-full object-contain rounded-lg"
          onClick={(e) => e.stopPropagation()}
        />
      </div>
    </div>
  )
}

export default ImagePreview