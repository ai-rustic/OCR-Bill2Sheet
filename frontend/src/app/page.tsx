"use client"

import { UploadImageUI } from "@/components/upload/UploadImageUI"
import { Navigation } from "@/components/navigation/Navigation"
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card"

export default function Home() {
  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-4xl mx-auto space-y-8">
        {/* Header */}
        <div className="text-center">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">
            OCR Bill to Sheet
          </h1>
          <p className="text-gray-600">
            Upload your bills and convert them to spreadsheet data using AI
          </p>
        </div>

        {/* Navigation */}
        <Navigation />

        {/* Upload Section */}
        <Card>
          <CardHeader>
            <CardTitle>Upload Bills</CardTitle>
          </CardHeader>
          <CardContent>
            <UploadImageUI
              acceptedTypes={{
                mimeTypes: ["image/jpeg", "image/png", "image/webp", "image/jfif", "application/pdf"],
                extensions: [".jpg", ".jpeg", ".png", ".webp", ".jfif", ".pdf"],
                maxSize: 10 * 1024 * 1024 // 10MB
              }}
              maxFiles={10}
              onUploadComplete={(files) => {
                console.log("Upload completed:", files)
              }}
              onUploadError={(errors) => {
                console.error("Upload errors:", errors)
              }}
              showStats={true}
              showFileList={true}
            />
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
