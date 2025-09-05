# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Bill OCR to Excel application that extracts data from bill images and exports to Excel files automatically. The application supports multiple image uploads for long receipts that require multiple photos.

**Architecture:**
- **Frontend:** Next.js 15 with TypeScript and Shadcn UI components
- **Backend:** Rust with Axum framework (backend code not yet implemented)
- **OCR:** Google Gemini API
- **Excel Generation:** umya-spreadsheet (Rust)

## Language Requirements

**IMPORTANT:** All code, comments, variable names, function names, and user-facing text MUST be in English. Vietnamese should not be used anywhere in the codebase.

## Development Commands

**CRITICAL:** Never use `npm run dev` for development. Always use `npm run typecheck` and `npm run lint` after any frontend changes.

**Frontend (from `frontend/` directory):**
- `npm run typecheck` - Run TypeScript type checking (ALWAYS run after changes)
- `npm run lint` - Run ESLint (ALWAYS run after changes)  
- `npm run build` - Build for production with Turbopack
- `npm run start` - Start production server

**Backend:** (Not yet implemented - will be Rust with Cargo)
- `cargo run` - Run development server
- `cargo build` - Build for production
- `cargo test` - Run tests
- `cargo clippy` - Run linter

## System Flow

1. User uploads multiple bill images via Next.js frontend (supports up to 20 images, 10MB each)
2. Frontend allows image reordering with arrow buttons for proper bill sequence
3. Frontend sends all images to backend via POST `/api/ocr-bill` with multipart/form-data
4. Backend processes images in order and forwards to Gemini OCR API
5. Backend parses OCR text and extracts structured data
6. Backend generates Excel file using umya-spreadsheet
7. Excel file is returned to frontend for automatic download

## Critical UI Requirements

**MANDATORY:** All UI components MUST use shadcn UI components exclusively. DO NOT create custom components unless absolutely necessary. Always check the available shadcn components first and use them instead of building from scratch.

Use the `mcp__shadcn__getComponents` and `mcp__shadcn__getComponent` tools to explore available components before implementing any UI functionality.

**Current shadcn components in use:**
- Button component with variants (default, outline, ghost)
- Dropzone component with drag-and-drop functionality  
- Banner component with BannerIcon and BannerTitle for instruction displays
- Badge component for basic labeling
- Pill component for enhanced badges with rounded styling
- Lucide React icons (Download, Loader2, X, ArrowUp, ArrowDown, Plus, Info)

## API Endpoints

- `POST /api/ocr-bill`
  - Request: multipart/form-data with fields:
    - `bill_image_0`, `bill_image_1`, etc. (File objects)
    - `image_count` (string - total number of images)
  - Response: Excel file (.xlsx) with MIME type `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`

## File Structure

**Frontend (Next.js App Router):**
```
frontend/src/
├── app/
│   ├── layout.tsx          # Root layout with metadata
│   ├── page.tsx            # Home page with file upload logic
│   ├── globals.css         # Global styles
│   └── favicon files
├── components/
│   ├── ui/                 # Shadcn UI components
│   │   ├── button.tsx
│   │   ├── dropzone.tsx
│   │   ├── banner.tsx      # Banner component for info displays
│   │   ├── badge.tsx       # Basic badge component
│   │   └── pill.tsx        # Enhanced badge with rounded styling
│   └── bill-upload.tsx     # Main upload component
└── lib/
    └── utils.ts            # Utility functions
```

**Backend:** (To be implemented in Rust)

## Key Dependencies

**Frontend:**
- Next.js 15.5.2 with App Router
- React 19.1.0 with TypeScript 5
- Shadcn UI components (Button, Dropzone, Banner, Badge, Pill)
- TailwindCSS 4 with PostCSS
- Lucide React icons
- react-dropzone for file handling
- class-variance-authority & clsx for styling
- @radix-ui/react-use-controllable-state for component state management

**Backend:** (To be added)
- Axum for HTTP server
- umya-spreadsheet for Excel generation
- reqwest for Gemini API calls
- Serde for JSON serialization
- Multipart form handling

## Multi-Image Bill Processing

The application is specifically designed for long receipts that require multiple photos:

1. **Image Ordering:** Users can reorder images using up/down arrow buttons
2. **Visual Preview:** Each image shows a thumbnail with order number (#1, #2, etc.)
3. **File Management:** Individual remove buttons and "Remove All" functionality
4. **Upload State:** Loading states and progress indicators during processing
5. **Instructions:** Built-in user guide for proper image ordering

## Environment Configuration

Environment variables should be configured in `.env` file:
- Gemini OCR API key and configuration
- Backend server configuration
- Other service configurations as needed

## Future Extensions

The system is designed to be extensible for:
- Multiple bill formats and templates
- Bill history and search functionality
- Multiple export formats (CSV, PDF)
- Cloud storage integration (Google Drive, S3)
- Email delivery of processed files
- Real-time processing status updates