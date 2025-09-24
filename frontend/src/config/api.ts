/**
 * API Configuration
 */

// Default to localhost:3001 for development, can be overridden with environment variable
export const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

export const API_ENDPOINTS = {
  OCR: `${API_BASE_URL}/api/ocr`,
  HEALTH: `${API_BASE_URL}/api/health`,
  BILLS: `${API_BASE_URL}/api/bills`,
} as const;