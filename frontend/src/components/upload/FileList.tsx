"use client"

import * as React from "react"
import { Grid, List, Trash2, RotateCcw, CheckCircle, AlertCircle } from "lucide-react"
import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import { FileListProps, UploadStatus } from "@/types/upload"
import { ImagePreview, ListImagePreview } from "./ImagePreview"
import { formatUploadStats } from "@/utils/fileFormatting"

/**
 * File list grid component that displays all uploaded images
 */
export function FileList({
  images,
  onDelete,
  onRetry,
  columns = 3,
  showProgress = true,
  className,
  ...props
}: FileListProps & { className?: string }) {
  const [viewMode, setViewMode] = React.useState<'grid' | 'list'>('grid')

  if (images.length === 0) {
    return (
      <div className={cn("text-center py-12", className)} {...props}>
        <div className="text-gray-400 text-lg mb-2">📁</div>
        <div className="text-gray-500">No files uploaded yet</div>
        <div className="text-sm text-gray-400 mt-1">
          Drop files or click browse to get started
        </div>
      </div>
    )
  }

  const stats = {
    total: images.length,
    completed: images.filter(img => img.status === UploadStatus.COMPLETED).length,
    uploading: images.filter(img => img.status === UploadStatus.UPLOADING).length,
    errors: images.filter(img => img.status === UploadStatus.ERROR).length
  }

  return (
    <div className={cn("space-y-4", className)} {...props}>
      {/* Header with stats and view controls */}
      <div className="flex items-center justify-between">
        <div className="space-y-1">
          <div className="text-lg font-semibold text-gray-900">
            Uploaded Files ({images.length})
          </div>
          <div className="text-sm text-gray-600">
            {formatUploadStats(stats)}
          </div>
        </div>

        <div className="flex items-center space-x-2">
          {/* Bulk actions */}
          {stats.errors > 0 && onRetry && (
            <Button
              size="sm"
              variant="outline"
              onClick={() => {
                images
                  .filter(img => img.status === UploadStatus.ERROR)
                  .forEach(img => onRetry(img.id))
              }}
              className="text-blue-600"
            >
              <RotateCcw className="h-4 w-4 mr-1" />
              Retry All Failed
            </Button>
          )}

          {stats.completed > 0 && (
            <Button
              size="sm"
              variant="outline"
              onClick={() => {
                images
                  .filter(img => img.status === UploadStatus.COMPLETED)
                  .forEach(img => onDelete(img.id))
              }}
              className="text-red-600"
            >
              <Trash2 className="h-4 w-4 mr-1" />
              Clear Completed
            </Button>
          )}

          {/* View mode toggle */}
          <div className="flex rounded-md border">
            <Button
              size="sm"
              variant={viewMode === 'grid' ? 'default' : 'ghost'}
              onClick={() => setViewMode('grid')}
              className="rounded-r-none"
            >
              <Grid className="h-4 w-4" />
            </Button>
            <Button
              size="sm"
              variant={viewMode === 'list' ? 'default' : 'ghost'}
              onClick={() => setViewMode('list')}
              className="rounded-l-none"
            >
              <List className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </div>

      {/* File list */}
      {viewMode === 'grid' ? (
        <GridFileList
          images={images}
          onDelete={onDelete}
          onRetry={onRetry}
          columns={columns}
          showProgress={showProgress}
        />
      ) : (
        <ListFileList
          images={images}
          onDelete={onDelete}
          onRetry={onRetry}
          showProgress={showProgress}
        />
      )}
    </div>
  )
}

/**
 * Grid layout for file list
 */
function GridFileList({
  images,
  onDelete,
  onRetry,
  columns,
  showProgress
}: {
  images: FileListProps['images']
  onDelete: FileListProps['onDelete']
  onRetry?: FileListProps['onRetry']
  columns: number
  showProgress: boolean
}) {
  return (
    <div
      className={cn(
        "grid gap-4",
        columns === 2 && "grid-cols-1 sm:grid-cols-2",
        columns === 3 && "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3",
        columns === 4 && "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4",
        columns === 5 && "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5"
      )}
    >
      {images.map((image) => (
        <ImagePreview
          key={image.id}
          image={image}
          onDelete={onDelete}
          onRetry={onRetry}
          size="medium"
        />
      ))}
    </div>
  )
}

/**
 * List layout for file list
 */
function ListFileList({
  images,
  onDelete,
  onRetry,
  showProgress
}: {
  images: FileListProps['images']
  onDelete: FileListProps['onDelete']
  onRetry?: FileListProps['onRetry']
  showProgress: boolean
}) {
  return (
    <div className="space-y-2">
      {images.map((image) => (
        <ListImagePreview
          key={image.id}
          image={image}
          onDelete={onDelete}
          onRetry={onRetry}
        />
      ))}
    </div>
  )
}

/**
 * Compact file list for smaller spaces
 */
export function CompactFileList({
  images,
  onDelete,
  onRetry,
  showProgress = true,
  maxVisible = 5,
  className,
  ...props
}: FileListProps & {
  maxVisible?: number
  className?: string
}) {
  const [showAll, setShowAll] = React.useState(false)

  const visibleImages = showAll ? images : images.slice(0, maxVisible)
  const hasMore = images.length > maxVisible

  if (images.length === 0) {
    return (
      <div className={cn("text-center py-8 text-sm text-gray-500", className)} {...props}>
        No files uploaded
      </div>
    )
  }

  return (
    <div className={cn("space-y-3", className)} {...props}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="text-sm font-medium text-gray-900">
          Files ({images.length})
        </div>
        {images.length > 0 && (
          <Button
            size="sm"
            variant="ghost"
            onClick={() => images.forEach(img => onDelete(img.id))}
            className="text-red-600 text-xs"
          >
            Clear All
          </Button>
        )}
      </div>

      {/* File list */}
      <div className="space-y-2">
        {visibleImages.map((image) => (
          <div
            key={image.id}
            className="flex items-center space-x-2 p-2 rounded border text-sm"
          >
            {/* Status icon */}
            <div className="flex-shrink-0">
              {image.status === UploadStatus.UPLOADING && (
                <div className="h-4 w-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
              )}
              {image.status === UploadStatus.COMPLETED && (
                <CheckCircle className="h-4 w-4 text-green-500" />
              )}
              {image.status === UploadStatus.ERROR && (
                <AlertCircle className="h-4 w-4 text-red-500" />
              )}
            </div>

            {/* File name */}
            <div className="flex-1 min-w-0 truncate text-gray-900">
              {image.fileName}
            </div>

            {/* Progress */}
            {showProgress && image.status === UploadStatus.UPLOADING && (
              <div className="text-xs text-gray-500">
                {Math.round(image.progress)}%
              </div>
            )}

            {/* Actions */}
            <div className="flex-shrink-0 flex space-x-1">
              {image.status === UploadStatus.ERROR && onRetry && (
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => onRetry(image.id)}
                  className="h-6 w-6 p-0 text-blue-600"
                >
                  <RotateCcw className="h-3 w-3" />
                </Button>
              )}
              <Button
                size="sm"
                variant="ghost"
                onClick={() => onDelete(image.id)}
                className="h-6 w-6 p-0 text-red-600"
              >
                <Trash2 className="h-3 w-3" />
              </Button>
            </div>
          </div>
        ))}
      </div>

      {/* Show more/less toggle */}
      {hasMore && (
        <Button
          size="sm"
          variant="ghost"
          onClick={() => setShowAll(!showAll)}
          className="w-full text-xs text-gray-600"
        >
          {showAll
            ? `Show Less`
            : `Show ${images.length - maxVisible} More Files`
          }
        </Button>
      )}
    </div>
  )
}

/**
 * Status summary component for file list
 */
export function FileListSummary({
  images,
  className,
  ...props
}: {
  images: FileListProps['images']
  className?: string
}) {
  const stats = {
    total: images.length,
    completed: images.filter(img => img.status === UploadStatus.COMPLETED).length,
    uploading: images.filter(img => img.status === UploadStatus.UPLOADING).length,
    errors: images.filter(img => img.status === UploadStatus.ERROR).length
  }

  if (stats.total === 0) {
    return null
  }

  return (
    <div className={cn("flex items-center space-x-4 text-sm", className)} {...props}>
      <div className="flex items-center space-x-1">
        <div className="h-3 w-3 bg-gray-400 rounded-full" />
        <span>{stats.total} Total</span>
      </div>
      {stats.completed > 0 && (
        <div className="flex items-center space-x-1">
          <div className="h-3 w-3 bg-green-500 rounded-full" />
          <span>{stats.completed} Completed</span>
        </div>
      )}
      {stats.uploading > 0 && (
        <div className="flex items-center space-x-1">
          <div className="h-3 w-3 bg-blue-500 rounded-full" />
          <span>{stats.uploading} Uploading</span>
        </div>
      )}
      {stats.errors > 0 && (
        <div className="flex items-center space-x-1">
          <div className="h-3 w-3 bg-red-500 rounded-full" />
          <span>{stats.errors} Failed</span>
        </div>
      )}
    </div>
  )
}

export default FileList