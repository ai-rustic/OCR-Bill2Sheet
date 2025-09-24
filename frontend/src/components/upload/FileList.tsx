"use client"

import * as React from "react"
import {
  Loader2,
  CheckCircle2,
  AlertTriangle,
  Inbox,
  Image as ImageIcon,
  Grid,
  List,
  RotateCcw,
  Trash2
} from "lucide-react"

import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { cn } from "@/lib/utils"
import { FileListProps, UploadStatus, type ImageProcessingState } from "@/types/upload"
import {
  formatUploadStats,
  formatFileSize,
  formatRelativeTime,
  getFileTypeDisplayName
} from "@/utils/fileFormatting"

interface FileListComponentProps
  extends FileListProps,
    React.HTMLAttributes<HTMLDivElement> {
  className?: string
}

const gridColumnClasses: Record<number, string> = {
  2: "sm:grid-cols-2",
  3: "sm:grid-cols-2 xl:grid-cols-3",
  4: "sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4",
  5: "sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-5"
}

type StatusToken = {
  label: string
  chipClass: string
  iconClass: string
  icon: React.ComponentType<React.SVGProps<SVGSVGElement>>
}

const STATUS_TOKENS: Record<UploadStatus, StatusToken> = {
  [UploadStatus.UPLOADING]: {
    label: "Uploading",
    chipClass: "border-sky-200 bg-sky-50 text-sky-700",
    iconClass: "text-sky-500",
    icon: Loader2
  },
  [UploadStatus.COMPLETED]: {
    label: "Ready",
    chipClass: "border-emerald-200 bg-emerald-50 text-emerald-700",
    iconClass: "text-emerald-500",
    icon: CheckCircle2
  },
  [UploadStatus.ERROR]: {
    label: "Needs attention",
    chipClass: "border-rose-200 bg-rose-50 text-rose-700",
    iconClass: "text-rose-500",
    icon: AlertTriangle
  }
}

const PROCESSING_BADGES: Record<Exclude<ImageProcessingState, 'processing'>, { label: string; className: string }> = {
  finished: {
    label: "Finished",
    className: "border-emerald-200 bg-emerald-50 text-emerald-700"
  },
  error: {
    label: "Processing failed",
    className: "border-rose-200 bg-rose-50 text-rose-700"
  }
}

export function FileList({
  images,
  onDelete,
  onRetry,
  columns = 3,
  showProgress = true,
  processingStatuses,
  className,
  ...props
}: FileListComponentProps) {
  const [viewMode, setViewMode] = React.useState<"grid" | "list">("grid")

  const stats = {
    total: images.length,
    completed: images.filter((img) => img.status === UploadStatus.COMPLETED).length,
    uploading: images.filter((img) => img.status === UploadStatus.UPLOADING).length,
    errors: images.filter((img) => img.status === UploadStatus.ERROR).length
  }

  if (stats.total === 0) {
    return <EmptyState className={className} {...props} />
  }

  return (
    <section className={cn("space-y-6 w-full", className)} {...props}>
      <header className="flex flex-col gap-4">
        <div className="flex flex-wrap items-end justify-between gap-4">
          <div className="space-y-1">
            <p className="text-sm font-medium text-slate-500">Uploaded bills</p>
            <h2 className="text-lg font-semibold text-slate-900">
              {stats.total} file{stats.total !== 1 ? "s" : ""} ready for review
            </h2>
            <p className="text-sm text-slate-500">{formatUploadStats(stats)}</p>
          </div>

          <div className="flex flex-wrap items-center gap-2">
            {stats.errors > 0 && onRetry && (
              <Button
                type="button"
                size="sm"
                variant="outline"
                className="border-rose-200 bg-rose-50 text-rose-600 hover:bg-rose-50/80"
                onClick={() => {
                  images
                    .filter((img) => img.status === UploadStatus.ERROR)
                    .forEach((img) => onRetry(img.id))
                }}
              >
                <RotateCcw className="h-4 w-4" />
                Retry failed
              </Button>
            )}

            {stats.completed > 0 && (
              <Button
                type="button"
                size="sm"
                variant="outline"
                className="border-slate-200 bg-white text-slate-600 hover:text-rose-600"
                onClick={() => {
                  images
                    .filter((img) => img.status === UploadStatus.COMPLETED)
                    .forEach((img) => onDelete(img.id))
                }}
              >
                <Trash2 className="h-4 w-4" />
                Clear completed
              </Button>
            )}

            <div className="flex items-center gap-1 rounded-lg border border-slate-200 bg-white p-1 shadow-sm">
              <Button
                type="button"
                size="icon"
                variant={viewMode === "grid" ? "default" : "ghost"}
                className={cn(
                  "size-8 rounded-md",
                  viewMode === "grid" && "bg-slate-900 text-white hover:bg-slate-900/90"
                )}
                onClick={() => setViewMode("grid")}
                aria-label="Grid view"
              >
                <Grid className="h-4 w-4" />
              </Button>
              <Button
                type="button"
                size="icon"
                variant={viewMode === "list" ? "default" : "ghost"}
                className={cn(
                  "size-8 rounded-md",
                  viewMode === "list" && "bg-slate-900 text-white hover:bg-slate-900/90"
                )}
                onClick={() => setViewMode("list")}
                aria-label="List view"
              >
                <List className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>

        <div className="flex flex-wrap items-center gap-2">
          <Badge variant="outline" className="border-slate-200 bg-white text-slate-600">
            Total {stats.total}
          </Badge>
          {stats.completed > 0 && (
            <Badge variant="outline" className="border-emerald-200 bg-emerald-50 text-emerald-700">
              {stats.completed} completed
            </Badge>
          )}
          {stats.uploading > 0 && (
            <Badge variant="outline" className="border-sky-200 bg-sky-50 text-sky-700">
              {stats.uploading} uploading
            </Badge>
          )}
          {stats.errors > 0 && (
            <Badge variant="outline" className="border-rose-200 bg-rose-50 text-rose-700">
              {stats.errors} failed
            </Badge>
          )}
        </div>
      </header>

      {viewMode === "grid" ? (
        <div
          className={cn(
            "grid grid-cols-1 gap-4",
            gridColumnClasses[columns] ?? "sm:grid-cols-2 xl:grid-cols-3"
          )}
        >
          {images.map((image) => (
            <FileCard
              key={image.id}
              image={image}
              onDelete={onDelete}
              onRetry={onRetry}
              showProgress={showProgress}
              processingState={processingStatuses?.[image.id]}
            />
          ))}
        </div>
      ) : (
        <div className="space-y-3">
          {images.map((image) => (
            <FileRow
              key={image.id}
              image={image}
              onDelete={onDelete}
              onRetry={onRetry}
              showProgress={showProgress}
              processingState={processingStatuses?.[image.id]}
            />
          ))}
        </div>
      )}
    </section>
  )
}

interface FileLayoutProps {
  image: FileListProps["images"][number]
  onDelete: FileListProps["onDelete"]
  onRetry?: FileListProps["onRetry"]
  showProgress: boolean
  processingState?: ImageProcessingState
}

function FileCard({ image, onDelete, onRetry, showProgress, processingState }: FileLayoutProps) {
  const [previewError, setPreviewError] = React.useState(false)

  React.useEffect(() => {
    setPreviewError(false)
  }, [image.previewUrl])

  const uploadedAt = React.useMemo(() => {
    if (!image.uploadedAt) return null
    return image.uploadedAt instanceof Date ? image.uploadedAt : new Date(image.uploadedAt)
  }, [image.uploadedAt])

  const statusMeta = STATUS_TOKENS[image.status]
  const StatusIcon = statusMeta.icon

  return (
    <article className="group flex h-full flex-col overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm transition hover:border-slate-300 hover:shadow-md">
        <div className="relative aspect-[4/3] overflow-hidden bg-slate-100">
          {image.previewUrl && !previewError ? (
            <img
              src={image.previewUrl}
              alt={image.fileName}
              className="h-full w-full object-cover transition duration-300 group-hover:scale-105"
              onError={() => setPreviewError(true)}
            />
          ) : (
            <div className="flex h-full w-full items-center justify-center text-slate-400">
              <ImageIcon className="h-10 w-10" />
            </div>
          )}

          {processingState === 'processing' && (
            <div className="pointer-events-none absolute inset-0 flex items-center justify-center bg-white/70 backdrop-blur-sm">
              <Loader2 className="h-6 w-6 text-sky-600 animate-spin" />
            </div>
          )}

          <div className="absolute left-3 top-3 flex items-center gap-2">
            <Badge
              variant="outline"
              className={cn(
              "gap-1.5 border px-2.5 py-1 text-xs font-medium shadow-sm",
              statusMeta.chipClass
            )}
          >
            <StatusIcon
              className={cn(
                "h-3.5 w-3.5",
                statusMeta.iconClass,
                image.status === UploadStatus.UPLOADING && "animate-spin"
              )}
            />
            {statusMeta.label}
          </Badge>
        </div>

        <div className="absolute right-3 top-3 flex items-center gap-1">
          {image.status === UploadStatus.ERROR && onRetry && (
            <Button
              type="button"
              size="icon"
              variant="ghost"
              className="size-8 rounded-full bg-white/95 text-rose-600 shadow-sm hover:bg-white"
              onClick={() => onRetry(image.id)}
              aria-label={`Retry upload for ${image.fileName}`}
            >
              <RotateCcw className="h-4 w-4" />
            </Button>
          )}
          <Button
            type="button"
            size="icon"
            variant="ghost"
            className="size-8 rounded-full bg-white/95 text-slate-600 shadow-sm hover:bg-white"
            onClick={() => onDelete(image.id)}
            aria-label={`Remove ${image.fileName}`}
          >
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      </div>

        <div className="flex flex-1 flex-col gap-3 p-4">
          <div className="space-y-1">
            <h3 className="text-sm font-semibold text-slate-900 line-clamp-2">
              {image.fileName}
            </h3>
            <p className="text-xs text-slate-500">
              {formatFileSize(image.fileSize)} · {getFileTypeDisplayName(image.fileType)}
              {uploadedAt && ` · ${formatRelativeTime(uploadedAt)}`}
            </p>
          </div>

          {processingState && processingState !== 'processing' && (
            <Badge
              variant="outline"
              className={cn(
                "w-fit px-2.5 py-1 text-[11px] font-medium",
                PROCESSING_BADGES[processingState].className
              )}
            >
              {PROCESSING_BADGES[processingState].label}
            </Badge>
          )}

          {showProgress && image.status === UploadStatus.UPLOADING && (
            <UploadProgressBar progress={image.progress} />
          )}

        {image.status === UploadStatus.ERROR && image.error && (
          <p className="text-xs font-medium text-rose-600">
            {image.error}
          </p>
        )}
      </div>
    </article>
  )
}

function FileRow({ image, onDelete, onRetry, showProgress, processingState }: FileLayoutProps) {
  const [previewError, setPreviewError] = React.useState(false)

  React.useEffect(() => {
    setPreviewError(false)
  }, [image.previewUrl])

  const uploadedAt = React.useMemo(() => {
    if (!image.uploadedAt) return null
    return image.uploadedAt instanceof Date ? image.uploadedAt : new Date(image.uploadedAt)
  }, [image.uploadedAt])

  const statusMeta = STATUS_TOKENS[image.status]
  const StatusIcon = statusMeta.icon

  return (
    <article className="flex items-center gap-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm transition hover:border-slate-300 hover:shadow-md">
      <div className="relative h-16 w-16 overflow-hidden rounded-lg bg-slate-100">
        {image.previewUrl && !previewError ? (
          <img
            src={image.previewUrl}
            alt={image.fileName}
            className="h-full w-full object-cover"
            onError={() => setPreviewError(true)}
          />
        ) : (
          <div className="flex h-full w-full items-center justify-center text-slate-400">
            <ImageIcon className="h-8 w-8" />
          </div>
        )}

        {processingState === 'processing' && (
          <div className="pointer-events-none absolute inset-0 flex items-center justify-center bg-white/80 backdrop-blur-sm">
            <Loader2 className="h-5 w-5 text-sky-600 animate-spin" />
          </div>
        )}
      </div>

      <div className="flex flex-1 flex-col gap-2">
        <div className="flex flex-wrap items-start justify-between gap-2">
          <div className="space-y-1">
            <div className="flex items-center gap-2">
              <h3 className="text-sm font-semibold text-slate-900">
                {image.fileName}
              </h3>
              <Badge
                variant="outline"
                className={cn(
                  "gap-1.5 border px-2 py-0.5 text-[11px] font-medium",
                  statusMeta.chipClass
                )}
              >
                <StatusIcon
                  className={cn(
                    "h-3 w-3",
                    statusMeta.iconClass,
                    image.status === UploadStatus.UPLOADING && "animate-spin"
                  )}
                />
                {statusMeta.label}
              </Badge>
              {processingState && processingState !== 'processing' && (
                <Badge
                  variant="outline"
                  className={cn(
                    "gap-1.5 border px-2 py-0.5 text-[11px] font-medium",
                    PROCESSING_BADGES[processingState].className
                  )}
                >
                  {PROCESSING_BADGES[processingState].label}
                </Badge>
              )}
            </div>
            <p className="text-xs text-slate-500">
              {formatFileSize(image.fileSize)} · {getFileTypeDisplayName(image.fileType)}
              {uploadedAt && ` · ${formatRelativeTime(uploadedAt)}`}
            </p>
          </div>

          <div className="flex items-center gap-1">
            {image.status === UploadStatus.ERROR && onRetry && (
              <Button
                type="button"
                size="icon"
                variant="ghost"
                className="size-8 rounded-full text-rose-600 hover:bg-rose-50"
                onClick={() => onRetry(image.id)}
                aria-label={`Retry upload for ${image.fileName}`}
              >
                <RotateCcw className="h-4 w-4" />
              </Button>
            )}
            <Button
              type="button"
              size="icon"
              variant="ghost"
              className="size-8 rounded-full text-slate-600 hover:bg-slate-100"
              onClick={() => onDelete(image.id)}
              aria-label={`Remove ${image.fileName}`}
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        </div>

        {showProgress && image.status === UploadStatus.UPLOADING && (
          <UploadProgressBar progress={image.progress} />
        )}

        {image.status === UploadStatus.ERROR && image.error && (
          <p className="text-xs font-medium text-rose-600">
            {image.error}
          </p>
        )}
      </div>
    </article>
  )
}

function UploadProgressBar({ progress }: { progress: number }) {
  const safeProgress = Math.min(Math.max(progress, 6), 100)

  return (
    <div className="space-y-1">
      <div className="relative h-1.5 overflow-hidden rounded-full bg-slate-100">
        <div
          className="absolute inset-y-0 left-0 rounded-full bg-sky-500 transition-all"
          style={{ width: `${safeProgress}%` }}
        />
      </div>
      <p className="text-xs font-medium text-slate-600">
        {Math.round(progress)}% uploaded
      </p>
    </div>
  )
}

function EmptyState({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "flex flex-col items-center justify-center rounded-xl border border-dashed border-slate-200 bg-slate-50 px-6 py-16 text-center shadow-inner",
        className
      )}
      {...props}
    >
      <div className="flex h-14 w-14 items-center justify-center rounded-full bg-white shadow-sm ring-1 ring-slate-200">
        <Inbox className="h-6 w-6 text-slate-400" />
      </div>
      <h3 className="mt-4 text-base font-semibold text-slate-700">Chua có hoá don</h3>
      <p className="mt-1 max-w-sm text-sm text-slate-500">
        Kéo th? ho?c b?m nút t?i hoá don d? xem chúng xu?t hi?n t?i dây.
      </p>
    </div>
  )
}

export function CompactFileList({
  images,
  onDelete,
  onRetry,
  showProgress = true,
  maxVisible = 5,
  className,
  processingStatuses,
  ...props
}: FileListProps & {
  maxVisible?: number
  className?: string
} & React.HTMLAttributes<HTMLDivElement>) {
  const [showAll, setShowAll] = React.useState(false)

  const visibleImages = showAll ? images : images.slice(0, maxVisible)
  const hasMore = images.length > maxVisible

  if (images.length === 0) {
    return (
      <EmptyState className={className} {...props} />
    )
  }

  return (
    <div className={cn("space-y-3", className)} {...props}>
      <div className="flex items-center justify-between">
        <div className="text-sm font-semibold text-slate-900">
          Files ({images.length})
        </div>
        <Button
          type="button"
          size="sm"
          variant="ghost"
          className="text-slate-500 hover:text-rose-600"
          onClick={() => images.forEach((img) => onDelete(img.id))}
        >
          Clear all
        </Button>
      </div>

      <div className="space-y-2">
        {visibleImages.map((image) => (
          <CompactRow
            key={image.id}
            image={image}
            onDelete={onDelete}
            onRetry={onRetry}
            showProgress={showProgress}
            processingState={processingStatuses?.[image.id]}
          />
        ))}
      </div>

      {hasMore && (
        <Button
          type="button"
          size="sm"
          variant="ghost"
          className="w-full text-slate-500"
          onClick={() => setShowAll((prev) => !prev)}
        >
          {showAll
            ? "Show less"
            : `Show ${images.length - maxVisible} more`}
        </Button>
      )}
    </div>
  )
}

function CompactRow({ image, onDelete, onRetry, showProgress, processingState }: FileLayoutProps) {
  const uploadedAt = image.uploadedAt instanceof Date
    ? image.uploadedAt
    : new Date(image.uploadedAt)

  return (
    <div className="flex items-center gap-3 rounded-lg border border-slate-200 bg-white p-3 text-sm shadow-sm">
      <div className="relative h-10 w-10 overflow-hidden rounded-md bg-slate-100">
        {image.previewUrl ? (
          <img
            src={image.previewUrl}
            alt={image.fileName}
            className="h-full w-full object-cover"
          />
        ) : (
          <div className="flex h-full w-full items-center justify-center text-slate-400">
            <ImageIcon className="h-5 w-5" />
          </div>
        )}
        {processingState === 'processing' && (
          <div className="pointer-events-none absolute inset-0 flex items-center justify-center bg-white/80 backdrop-blur-sm">
            <Loader2 className="h-4 w-4 text-sky-600 animate-spin" />
          </div>
        )}
      </div>

      <div className="min-w-0 flex-1">
        <div className="flex items-center justify-between gap-2">
          <span className="truncate font-medium text-slate-900">
            {image.fileName}
          </span>
          <span className="text-xs text-slate-500">
            {formatFileSize(image.fileSize)}
          </span>
        </div>
        <div className="mt-1 flex items-center gap-2 text-xs text-slate-500">
          <span>{STATUS_TOKENS[image.status].label}</span>
          {image.status === UploadStatus.COMPLETED && (
            <span>{formatRelativeTime(uploadedAt)}</span>
          )}
          {processingState && processingState !== 'processing' && (
            <Badge
              variant="outline"
              className={cn(
                "border px-2 py-0.5 text-[10px] font-medium",
                PROCESSING_BADGES[processingState].className
              )}
            >
              {PROCESSING_BADGES[processingState].label}
            </Badge>
          )}
        </div>
        {showProgress && image.status === UploadStatus.UPLOADING && (
          <div className="mt-2">
            <UploadProgressBar progress={image.progress} />
          </div>
        )}
      </div>

      <div className="flex items-center gap-1">
        {image.status === UploadStatus.ERROR && onRetry && (
          <Button
            type="button"
            size="icon"
            variant="ghost"
            className="size-7 rounded-full text-rose-600 hover:bg-rose-50"
            onClick={() => onRetry(image.id)}
            aria-label={`Retry upload for ${image.fileName}`}
          >
            <RotateCcw className="h-4 w-4" />
          </Button>
        )}
        <Button
          type="button"
          size="icon"
          variant="ghost"
          className="size-7 rounded-full text-slate-600 hover:bg-slate-100"
          onClick={() => onDelete(image.id)}
          aria-label={`Remove ${image.fileName}`}
        >
          <Trash2 className="h-4 w-4" />
        </Button>
      </div>
    </div>
  )
}

export function FileListSummary({
  images,
  className,
  ...props
}: {
  images: FileListProps["images"]
  className?: string
} & React.HTMLAttributes<HTMLDivElement>) {
  const stats = React.useMemo(() => ({
    total: images.length,
    completed: images.filter((img) => img.status === UploadStatus.COMPLETED).length,
    uploading: images.filter((img) => img.status === UploadStatus.UPLOADING).length,
    errors: images.filter((img) => img.status === UploadStatus.ERROR).length
  }), [images])

  if (stats.total === 0) {
    return null
  }

  return (
    <div className={cn("flex flex-wrap items-center gap-2 text-sm", className)} {...props}>
      <Badge variant="outline" className="border-slate-200 bg-white text-slate-600">
        Total {stats.total}
      </Badge>
      {stats.completed > 0 && (
        <Badge variant="outline" className="border-emerald-200 bg-emerald-50 text-emerald-700">
          {stats.completed} completed
        </Badge>
      )}
      {stats.uploading > 0 && (
        <Badge variant="outline" className="border-sky-200 bg-sky-50 text-sky-700">
          {stats.uploading} uploading
        </Badge>
      )}
      {stats.errors > 0 && (
        <Badge variant="outline" className="border-rose-200 bg-rose-50 text-rose-700">
          {stats.errors} failed
        </Badge>
      )}
    </div>
  )
}

export default FileList
