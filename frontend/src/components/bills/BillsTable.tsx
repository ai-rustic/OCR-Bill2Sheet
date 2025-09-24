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
import { Trash2, Edit, Eye, RefreshCw, ChevronLeft, ChevronRight, Download } from "lucide-react";

interface BillsTableProps {
  onEditBill?: (bill: Bill) => void;
  onViewBill?: (bill: Bill) => void;
  onDeleteBill?: (billId: number) => void;
}

export function BillsTable({ onEditBill, onViewBill, onDeleteBill }: BillsTableProps) {
  const [bills, setBills] = useState<Bill[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(1);
  const [totalBills, setTotalBills] = useState(0);
  const [hasMore, setHasMore] = useState(false);
  const [exporting, setExporting] = useState(false);
  const itemsPerPage = 20;

  const fetchBills = async (page: number = 1) => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getAllBills(page, itemsPerPage);

      if (response.success && response.data) {
        setBills(response.data);
        setCurrentPage(page);
        setHasMore(response.data.length === itemsPerPage);
      } else {
        setError(response.error || 'Failed to fetch bills');
        setHasMore(false);
      }
    } catch (err) {
      setError('Network error occurred');
      setHasMore(false);
    } finally {
      setLoading(false);
    }
  };

  const fetchTotalCount = async () => {
    try {
      const response = await apiClient.getBillsCount();
      if (response.success) {
        const count = typeof response.data === 'number'
          ? response.data
          : Number((response.data as { count?: number } | undefined)?.count ?? 0);

        setTotalBills(Number.isFinite(count) ? count : 0);
      }
    } catch (err) {
      console.error('Failed to fetch total count:', err);
    }
  };

  useEffect(() => {
    fetchBills();
    fetchTotalCount();
  }, []);

  const handleDelete = async (billId: number) => {
    if (window.confirm('Bạn có chắc chắn muốn xóa hóa đơn này?')) {
      try {
        const response = await apiClient.deleteBill(billId);

        if (response.success) {
          setBills(bills.filter(bill => bill.id !== billId));
          onDeleteBill?.(billId);

          // Refresh total count and adjust page if needed
          await fetchTotalCount();
          const newTotalPages = Math.ceil((totalBills - 1) / itemsPerPage);
          if (currentPage > newTotalPages && newTotalPages > 0) {
            fetchBills(newTotalPages);
          } else if (bills.length === 1 && currentPage > 1) {
            fetchBills(currentPage - 1);
          }
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

  const totalPagesFromCount = totalBills > 0 ? Math.ceil(totalBills / itemsPerPage) : 0;
  const inferredPages = hasMore ? currentPage + 1 : currentPage;
  const totalPages = Math.max(1, totalPagesFromCount || inferredPages);
  const totalPagesDisplay = totalBills > 0 ? `${totalPages}` : hasMore ? `${inferredPages}+` : `${totalPages}`;
  const totalLabel = totalBills > 0 ? totalBills : 'không xác định';

  const handlePrevPage = () => {
    if (currentPage > 1) {
      fetchBills(currentPage - 1);
    }
  };

  const handleNextPage = () => {
    const canGoNext = totalBills > 0 ? currentPage < totalPages : hasMore;
    if (canGoNext) {
      fetchBills(currentPage + 1);
    }
  };

  const handlePageClick = (page: number) => {
    if (page !== currentPage) {
      fetchBills(page);
    }
  };

  const handleExport = async () => {
    try {
      setExporting(true);
      const blob = await apiClient.exportBills('xlsx');
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      const timestamp = new Date().toISOString().slice(0, 10);
      link.href = url;
      link.download = `bills_${timestamp}.xlsx`;
      document.body.appendChild(link);
      link.click();
      link.remove();
      window.URL.revokeObjectURL(url);
    } catch (err) {
      console.error('Failed to export bills:', err);
      alert('Export danh sach hoa don that bai. Vui long thu lai.');
    } finally {
      setExporting(false);
    }
  };

  const renderPageNumbers = () => {
    if (totalBills === 0) {
      const pages = [
        <Button
          key={currentPage}
          variant="default"
          size="sm"
          onClick={() => handlePageClick(currentPage)}
          className="min-w-[2.5rem]"
        >
          {currentPage}
        </Button>
      ];

      if (hasMore) {
        const nextPage = currentPage + 1;
        pages.push(
          <Button
            key={nextPage}
            variant="outline"
            size="sm"
            onClick={() => handlePageClick(nextPage)}
            className="min-w-[2.5rem]"
          >
            {nextPage}
          </Button>
        );
      }

      return pages;
    }

    const pages = [];
    const maxVisiblePages = 5;

    let startPage = Math.max(1, currentPage - Math.floor(maxVisiblePages / 2));
    let endPage = Math.min(totalPages, startPage + maxVisiblePages - 1);

    // Adjust startPage if we're near the end
    if (endPage - startPage + 1 < maxVisiblePages) {
      startPage = Math.max(1, endPage - maxVisiblePages + 1);
    }

    if (startPage > 1) {
      pages.push(
        <Button
          key={1}
          variant={1 === currentPage ? "default" : "outline"}
          size="sm"
          onClick={() => handlePageClick(1)}
          className="min-w-[2.5rem]"
        >
          1
        </Button>
      );
      if (startPage > 2) {
        pages.push(<span key="ellipsis1" className="px-2">...</span>);
      }
    }

    for (let i = startPage; i <= endPage; i++) {
      pages.push(
        <Button
          key={i}
          variant={i === currentPage ? "default" : "outline"}
          size="sm"
          onClick={() => handlePageClick(i)}
          className="min-w-[2.5rem]"
        >
          {i}
        </Button>
      );
    }

    if (endPage < totalPages) {
      if (endPage < totalPages - 1) {
        pages.push(<span key="ellipsis2" className="px-2">...</span>);
      }
      pages.push(
        <Button
          key={totalPages}
          variant={totalPages === currentPage ? "default" : "outline"}
          size="sm"
          onClick={() => handlePageClick(totalPages)}
          className="min-w-[2.5rem]"
        >
          {totalPages}
        </Button>
      );
    }

    return pages;
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
          <Button onClick={() => fetchBills(1)} variant="outline">
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
        <CardTitle>
          {`Danh sách hóa đơn (${totalLabel} tổng cộng)`}
          {bills.length > 0 && (
            <span className="text-sm font-normal text-gray-500 ml-2">
              - Trang {currentPage} / {totalPagesDisplay} (hiển thị {bills.length} hóa đơn)
            </span>
          )}
        </CardTitle>
        <div className="flex items-center gap-2">
          <Button onClick={handleExport} variant="outline" size="sm" disabled={exporting || bills.length === 0}>
            <Download className="h-4 w-4 mr-2" />
            {exporting ? 'Dang xuat...' : 'Export XLSX'}
          </Button>
          <Button onClick={() => fetchBills(currentPage)} variant="outline" size="sm">
            <RefreshCw className="h-4 w-4 mr-2" />
            Lam moi
          </Button>
        </div>
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

        {/* Pagination Controls */}
        {bills.length > 0 && (
          <div className="flex items-center justify-between mt-6">
            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={handlePrevPage}
                disabled={currentPage === 1}
              >
                <ChevronLeft className="h-4 w-4" />
                Trước
              </Button>

              <div className="flex items-center gap-1">
                {renderPageNumbers()}
              </div>

              <Button
                variant="outline"
                size="sm"
                onClick={handleNextPage}
                disabled={totalBills > 0 ? currentPage === totalPages : !hasMore}
              >
                Sau
                <ChevronRight className="h-4 w-4" />
              </Button>
            </div>

            <div className="text-sm text-gray-500">
              {(() => {
                const rangeStart = ((currentPage - 1) * itemsPerPage) + 1;
                const inferredTotal = totalBills > 0
                  ? totalBills
                  : currentPage * itemsPerPage + (hasMore ? itemsPerPage : 0);
                const rangeEnd = Math.min(rangeStart + bills.length - 1, inferredTotal);

                return (
                  <span>
                    Hiển thị {rangeStart}-{rangeEnd} của {totalLabel} hóa đơn
                  </span>
                );
              })()}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
