const API_BASE_URL = 'http://localhost:2011';

export interface Bill {
  id: number;
  form_no?: string;
  serial_no?: string;
  invoice_no?: string;
  issued_date?: string;
  seller_name?: string;
  seller_tax_code?: string;
  item_name?: string;
  unit?: string;
  quantity?: number;
  unit_price?: number;
  total_amount?: number;
  vat_rate?: number;
  vat_amount?: number;
}

export interface CreateBill {
  form_no?: string;
  serial_no?: string;
  invoice_no?: string;
  issued_date?: string;
  seller_name?: string;
  seller_tax_code?: string;
  item_name?: string;
  unit?: string;
  quantity?: number;
  unit_price?: number;
  total_amount?: number;
  vat_rate?: number;
  vat_amount?: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
  error?: string;
}

class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string = API_BASE_URL) {
    this.baseUrl = baseUrl;
  }

  private async request<T>(endpoint: string, options?: RequestInit): Promise<ApiResponse<T>> {
    const url = `${this.baseUrl}${endpoint}`;

    try {
      const response = await fetch(url, {
        headers: {
          'Content-Type': 'application/json',
          ...options?.headers,
        },
        ...options,
      });

      const data = await response.json();

      if (!response.ok) {
        return {
          success: false,
          error: data.message || `HTTP error! status: ${response.status}`,
        };
      }

      return data;
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Network error occurred',
      };
    }
  }

  // Bill management endpoints
  async getAllBills(page?: number, limit?: number): Promise<ApiResponse<Bill[]>> {
    const params = new URLSearchParams();
    if (page !== undefined) params.append('page', page.toString());
    if (limit !== undefined) params.append('limit', limit.toString());

    const queryString = params.toString();
    const endpoint = queryString ? `/api/bills?${queryString}` : '/api/bills';

    return this.request<Bill[]>(endpoint);
  }

  async getBillById(id: number): Promise<ApiResponse<Bill>> {
    return this.request<Bill>(`/api/bills/${id}`);
  }

  async createBill(bill: CreateBill): Promise<ApiResponse<Bill>> {
    return this.request<Bill>('/api/bills', {
      method: 'POST',
      body: JSON.stringify(bill),
    });
  }

  async updateBill(id: number, bill: Partial<CreateBill>): Promise<ApiResponse<Bill>> {
    return this.request<Bill>(`/api/bills/${id}`, {
      method: 'PUT',
      body: JSON.stringify(bill),
    });
  }

  async deleteBill(id: number): Promise<ApiResponse<void>> {
    return this.request<void>(`/api/bills/${id}`, {
      method: 'DELETE',
    });
  }

  async searchBills(query: string): Promise<ApiResponse<Bill[]>> {
    const params = new URLSearchParams({ q: query });
    return this.request<Bill[]>(`/api/bills/search?${params}`);
  }

  async getBillsCount(): Promise<ApiResponse<number>> {
    return this.request<number>('/api/bills/count');
  }
}

export const apiClient = new ApiClient();
