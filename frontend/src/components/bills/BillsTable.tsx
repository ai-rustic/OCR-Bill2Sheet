"use client";

import { useEffect, useState } from "react";
import { apiClient, Bill } from "@/lib/api";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Trash2, Edit, Eye, RefreshCw } from "lucide-react";

interface BillsTableProps {
  onEditBill?: (bill: Bill) => void;
  onViewBill?: (bill: Bill) => void;
  onDeleteBill?: (billId: number) => void;
}

export function BillsTable({ onEditBill, onViewBill, onDeleteBill }: BillsTableProps) {
  const [bills, setBills] = useState<Bill[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchBills = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getAllBills();

      if (response.success && response.data) {
        setBills(response.data);
      } else {
        setError(response.error || 'Failed to fetch bills');
      }
    } catch (err) {
      setError('Network error occurred');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchBills();
  }, []);

  const handleDelete = async (billId: number) => {
    if (window.confirm('Bạn có chắc chắn muốn xóa hóa đơn này?')) {
      try {
        const response = await apiClient.deleteBill(billId);

        if (response.success) {
          setBills(bills.filter(bill => bill.id !== billId));
          onDeleteBill?.(billId);
        } else {
          alert(`Lỗi xóa hóa đơn: ${response.error}`);
        }
      } catch (err) {
        alert('Lỗi mạng khi xóa hóa đơn');
      }
    }
  };

  const formatCurrency = (amount?: number) => {
    if (!amount || amount === 0) return '-';
    return new Intl.NumberFormat('vi-VN', {
      style: 'currency',
      currency: 'VND'
    }).format(amount);
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    try {
      return new Date(dateString).toLocaleDateString('vi-VN');
    } catch {
      return dateString; // Return as-is if parsing fails
    }
  };

  const formatNumber = (num?: number) => {
    if (!num || num === 0) return '-';
    return new Intl.NumberFormat('vi-VN').format(num);
  };

  const formatPercentage = (rate?: number) => {
    if (!rate || rate === 0) return '-';
    return `${rate}%`;
  };

  if (loading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Danh sách hóa đơn</CardTitle>
        </CardHeader>
        <CardContent className="flex justify-center items-center py-8">
          <div className="flex items-center space-x-2">
            <RefreshCw className="h-4 w-4 animate-spin" />
            <span>Đang tải...</span>
          </div>
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Danh sách hóa đơn</CardTitle>
        </CardHeader>
        <CardContent className="text-center py-8">
          <div className="text-red-600 mb-4">{error}</div>
          <Button onClick={fetchBills} variant="outline">
            <RefreshCw className="h-4 w-4 mr-2" />
            Thử lại
          </Button>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle>Danh sách hóa đơn ({bills.length})</CardTitle>
        <Button onClick={fetchBills} variant="outline" size="sm">
          <RefreshCw className="h-4 w-4 mr-2" />
          Làm mới
        </Button>
      </CardHeader>
      <CardContent>
        {bills.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            Chưa có hóa đơn nào
          </div>
        ) : (
          <div className="overflow-auto">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>ID</TableHead>
                  <TableHead>Form No</TableHead>
                  <TableHead>Serial No</TableHead>
                  <TableHead>Invoice No</TableHead>
                  <TableHead>Issued Date</TableHead>
                  <TableHead>Seller Name</TableHead>
                  <TableHead>Seller Tax Code</TableHead>
                  <TableHead>Item Name</TableHead>
                  <TableHead>Unit</TableHead>
                  <TableHead>Quantity</TableHead>
                  <TableHead>Unit Price</TableHead>
                  <TableHead>Total Amount</TableHead>
                  <TableHead>VAT Rate</TableHead>
                  <TableHead>VAT Amount</TableHead>
                  <TableHead className="text-right">Thao tác</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {bills.map((bill) => (
                  <TableRow key={bill.id}>
                    <TableCell className="font-mono text-sm">{bill.id}</TableCell>
                    <TableCell>
                      <div className="max-w-xs truncate" title={bill.form_no}>
                        {bill.form_no || '-'}
                      </div>
                    </TableCell>
                    <TableCell>{bill.serial_no || '-'}</TableCell>
                    <TableCell className="font-medium">{bill.invoice_no || '-'}</TableCell>
                    <TableCell>{formatDate(bill.issued_date)}</TableCell>
                    <TableCell>
                      <div className="max-w-xs truncate" title={bill.seller_name}>
                        {bill.seller_name || '-'}
                      </div>
                    </TableCell>
                    <TableCell className="font-mono text-sm">{bill.seller_tax_code || '-'}</TableCell>
                    <TableCell>
                      <div className="max-w-xs truncate" title={bill.item_name}>
                        {bill.item_name || '-'}
                      </div>
                    </TableCell>
                    <TableCell>{bill.unit || '-'}</TableCell>
                    <TableCell className="text-right">{formatNumber(bill.quantity)}</TableCell>
                    <TableCell className="text-right">{formatCurrency(bill.unit_price)}</TableCell>
                    <TableCell className="text-right font-medium">{formatCurrency(bill.total_amount)}</TableCell>
                    <TableCell className="text-right">{formatPercentage(bill.vat_rate)}</TableCell>
                    <TableCell className="text-right">{formatCurrency(bill.vat_amount)}</TableCell>
                    <TableCell className="text-right">
                      <div className="flex justify-end space-x-1">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => onViewBill?.(bill)}
                          title="Xem chi tiết"
                        >
                          <Eye className="h-4 w-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => onEditBill?.(bill)}
                          title="Chỉnh sửa"
                        >
                          <Edit className="h-4 w-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => handleDelete(bill.id)}
                          className="text-red-600 hover:text-red-700"
                          title="Xóa"
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        )}
      </CardContent>
    </Card>
  );
}