# Research: Upload Image UI

## Technical Decisions

### File Handling Approach
**Decision**: HTML5 File API with React useState for local state management
**Rationale**: Native browser support, no external dependencies, perfect for UI-only requirements
**Alternatives considered**: FormData with actual uploads (rejected - no backend integration needed)

### Drag & Drop Implementation
**Decision**: HTML5 Drag and Drop API with React event handlers
**Rationale**: Standard web approach, good browser support, integrates well with Shadcn components
**Alternatives considered**: Third-party libraries like react-dropzone (rejected - adds dependency, constitution prefers minimal external deps)

### File Type Validation
**Decision**: JavaScript file extension and MIME type checking
**Rationale**: Client-side validation is sufficient for UI demo, immediate user feedback
**Alternatives considered**: Server-side validation (not applicable - no backend integration)

### Progress Tracking
**Decision**: Simulated progress with setTimeout to demonstrate UI states
**Rationale**: Shows realistic upload progress behavior without backend, good for UI demonstration
**Alternatives considered**: Instant completion (rejected - doesn't demonstrate progress UI properly)

### Image Previews
**Decision**: FileReader API to generate data URLs for preview thumbnails
**Rationale**: Standard approach for client-side image previews, no external service needed
**Alternatives considered**: Object URLs (rejected - requires cleanup), Canvas-based resizing (too complex for demo)

### Component Architecture
**Decision**: Composition of small Shadcn UI components (Card, Button, Progress, Alert)
**Rationale**: Follows constitutional requirements, highly customizable, consistent styling
**Alternatives considered**: Custom components (forbidden by constitution)

## Implementation Patterns

### State Management
Use React useState hooks for:
- Array of uploaded images with metadata
- Drag state for visual feedback
- Error states for validation feedback

### Event Handling
- `onDragOver` / `onDragLeave` for visual feedback
- `onDrop` for file processing
- `onChange` for file input fallback
- File validation on selection/drop

### Error Handling
- File type validation with user-friendly messages
- File size limits (reasonable for preview generation)
- Duplicate file detection with warning

## Shadcn Component Usage

### Required Components
- **Card**: Container for upload area and image previews
- **Button**: File selection trigger and delete actions
- **Progress**: Upload progress bars
- **Alert**: Error and success messages
- **Badge**: File type indicators
- **Grid**: Responsive image preview layout

### Icons (lucide-react)
- Upload, X (delete), Image, AlertCircle, CheckCircle

All components verified as available in standard Shadcn UI installation.