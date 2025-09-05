
# Bill OCR to Excel

Một tool trích xuất dữ liệu từ ảnh hóa đơn (bill) và xuất ra file Excel tự động.

---

## 🛠 Tech Stack

- **Frontend:** Next.js, Shadcn UI, TypeScript  
- **Backend:** Rust, Axum, umya-spreadsheet, reqwest/hyper, Serde  
- **OCR:** Google Gemini API  
- **Khác:** Docker (option), Cloud/VPS deploy

---

## 🚀 Luồng hoạt động

1. **User upload** ảnh bill trên giao diện web (Next.js, Shadcn UI)
2. **Frontend gửi ảnh** lên backend (Axum, Rust API)
3. **Backend gửi ảnh** tới Gemini OCR API để nhận diện ký tự (OCR)
4. **Backend parse** kết quả text: bóc tách từng trường thông tin cần thiết (sản phẩm, giá, số lượng...)
5. **Backend xuất file Excel** bằng thư viện umya-spreadsheet với định dạng chuẩn
6. **Trả về file Excel** cho frontend để user tải xuống

---

## 📦 Sơ đồ hệ thống

```
sequenceDiagram
    participant User
    participant Frontend as NextJS FE
    participant Backend as Rust Axum BE
    participant Gemini as Gemini OCR API

    User->>Frontend: Upload bill image
    Frontend->>Backend: Send image (POST multipart)
    Backend->>Gemini: Gửi ảnh để OCR
    Gemini-->>Backend: Nhận text kết quả OCR
    Backend->>Backend: Parse text & tạo Excel
    Backend-->>Frontend: Trả file Excel (.xlsx)
    Frontend-->>User: Cho phép tải file Excel
```

---

> Thông tin config key cho Gemini OCR và các biến môi trường nằm trong file `.env.example`.

---

## 📄 Mô tả API backend

- `POST /api/ocr-bill`
    - Request: multipart/form-data (`bill_image`)
    - Response: file Excel `.xlsx` (application/vnd.openxmlformats-officedocument.spreadsheetml.sheet)

---

## ✨ Tính năng mở rộng (phát triển thêm)

- Lưu các bill đã xử lý
- Từ khóa tìm kiếm, lọc bill
- Xuất nhiều định dạng (CSV, PDF)
- Gửi file qua email hoặc lưu cloud (Google Drive, S3)
- Xử lý nhiều mẫu bill khác nhau

---
