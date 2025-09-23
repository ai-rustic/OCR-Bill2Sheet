# Data Model: Upload Image UI

## Core Entities

### UploadedImage
Represents a single image file in the upload interface.

**Properties**:
- `id: string` - Unique identifier for the uploaded image
- `file: File` - HTML5 File object containing the image data
- `fileName: string` - Display name of the file
- `fileSize: number` - Size of the file in bytes
- `fileType: string` - MIME type of the file
- `progress: number` - Upload progress (0-100)
- `status: UploadStatus` - Current status of the upload
- `previewUrl: string | null` - Data URL for image preview thumbnail
- `error: string | null` - Error message if upload failed
- `uploadedAt: Date` - Timestamp when upload started

**Status Enum**:
```typescript
enum UploadStatus {
  UPLOADING = 'uploading',
  COMPLETED = 'completed',
  ERROR = 'error'
}
```

**State Transitions**:
1. File selected/dropped → `UPLOADING` (progress: 0)
2. Progress simulation → `UPLOADING` (progress: 1-99)
3. Upload complete → `COMPLETED` (progress: 100)
4. Validation error → `ERROR` (progress: 0, error message set)

**Validation Rules**:
- File type must be: 'image/jpeg', 'image/png', 'image/jfif'
- File extensions allowed: '.jpg', '.jpeg', '.png', '.jfif'
- File size reasonable for preview generation (< 10MB recommended)
- Duplicate file names handled by appending counter

## Upload Interface State

### UploadAreaState
Manages the overall upload interface state.

**Properties**:
- `images: UploadedImage[]` - Array of all uploaded images
- `isDragActive: boolean` - Whether drag operation is active over drop zone
- `isUploading: boolean` - Whether any uploads are in progress
- `totalFiles: number` - Total number of files processed
- `errors: string[]` - Array of validation error messages

**Computed Properties**:
- `completedCount: number` - Number of completed uploads
- `errorCount: number` - Number of failed uploads
- `activeUploads: UploadedImage[]` - Currently uploading files
- `completedUploads: UploadedImage[]` - Successfully uploaded files

## File Validation Schema

### AcceptedFileTypes
Configuration for allowed file types.

**Properties**:
- `mimeTypes: string[]` - ['image/jpeg', 'image/png', 'image/jfif']
- `extensions: string[]` - ['.jpg', '.jpeg', '.png', '.jfif']
- `maxSize: number` - Maximum file size in bytes (default: 10MB)

**Validation Functions**:
- `isValidFileType(file: File): boolean` - Check MIME type and extension
- `isValidFileSize(file: File): boolean` - Check file size limits
- `validateFile(file: File): ValidationResult` - Complete file validation

## Component Props Interfaces

### UploadAreaProps
```typescript
interface UploadAreaProps {
  onFilesAdded: (files: File[]) => void;
  acceptedTypes: AcceptedFileTypes;
  isDisabled?: boolean;
  maxFiles?: number;
}
```

### ImagePreviewProps
```typescript
interface ImagePreviewProps {
  image: UploadedImage;
  onDelete: (id: string) => void;
  onRetry?: (id: string) => void;
}
```

### ProgressIndicatorProps
```typescript
interface ProgressIndicatorProps {
  progress: number;
  status: UploadStatus;
  fileName: string;
}
```

## Data Flow

### Upload Process
1. User selects/drops files → `onFilesAdded` called
2. Files validated → Invalid files generate errors
3. Valid files added to `images` array with `UPLOADING` status
4. Progress simulation starts → Progress updated 0-100%
5. Preview generated using FileReader API → `previewUrl` set
6. Upload completes → Status changes to `COMPLETED`
7. User can delete → Image removed from array

### Error Handling
1. Invalid file type → Add to `errors` array, show Alert
2. File too large → Add to `errors` array, show Alert
3. Duplicate file → Show warning, allow or rename
4. Preview generation fails → Show placeholder image

This data model supports all functional requirements while maintaining client-side only operation as specified.