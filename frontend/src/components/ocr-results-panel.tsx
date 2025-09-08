'use client';

import { Loader2, Download, FileText, Calendar, DollarSign, Building, User, Hash, MapPin, CreditCard, FileSpreadsheet, Copy, Check } from 'lucide-react';
import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Pill } from '@/components/ui/pill';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';

interface BillMeta {
  bill_number: string;
  seller: string;
  buyer: string;
  seller_tax_code: string;
  buyer_tax_code: string;
  bill_date: string;
  total_amount: string;
  vat_amount: string;
  payment_method: string;
  address: string;
}

interface LineItem {
  no: number;
  product_name: string;
  quantity: string;
  unit: string;
  unit_price: string;
  subtotal: string;
}

interface BillData {
  bill_meta: BillMeta;
  line_items: LineItem[];
  notes: string;
}

interface OCRResult {
  success: boolean;
  bill_data?: BillData;
  message?: string;
  processing_timestamp?: string;
  error?: string;
}

interface OCRResultsPanelProps {
  showResults: boolean;
  isUploading: boolean;
  ocrResults: OCRResult | null;
}

export function OCRResultsPanel({ showResults, isUploading, ocrResults }: OCRResultsPanelProps) {
  const [copyNotification, setCopyNotification] = useState<string | null>(null);

  if (!showResults) {
    return null;
  }

  const handleDownloadExcel = (billData: BillData) => {
    // Create CSV content that Excel can read
    const headers = ['No.', 'Product Name', 'Quantity', 'Unit', 'Unit Price', 'Subtotal'];
    const csvHeaders = headers.join(',');
    
    const csvRows = billData.line_items.map(item => [
      item.no.toString(),
      `"${item.product_name.replace(/"/g, '""')}"`, // Escape quotes in product name
      item.quantity,
      item.unit,
      item.unit_price,
      item.subtotal
    ].join(','));
    
    const csvContent = [csvHeaders, ...csvRows].join('\n');
    
    // Add BOM for proper UTF-8 encoding in Excel
    const BOM = '\uFEFF';
    const csvWithBOM = BOM + csvContent;
    
    // Create and download file
    const blob = new Blob([csvWithBOM], { type: 'text/csv;charset=utf-8' });
    const url = window.URL.createObjectURL(blob);
    const link = document.createElement('a');
    
    // Generate filename with bill number and date
    const billNumber = billData.bill_meta.bill_number || 'bill';
    const date = new Date().toISOString().split('T')[0];
    const filename = `${billNumber}_line_items_${date}.csv`;
    
    link.href = url;
    link.download = filename;
    link.style.display = 'none';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    window.URL.revokeObjectURL(url);
    
    // Show success notification
    setCopyNotification('Excel file downloaded successfully!');
    setTimeout(() => {
      setCopyNotification(null);
    }, 3000);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const copyLineItemsTable = async (lineItems: LineItem[]) => {
    // Create header row
    const headers = ['No.', 'Product Name', 'Quantity', 'Unit', 'Unit Price', 'Subtotal'];
    const headerRow = headers.join('\t');
    
    // Create data rows
    const dataRows = lineItems.map(item => [
      item.no.toString(),
      item.product_name,
      item.quantity,
      item.unit,
      item.unit_price,
      item.subtotal
    ].join('\t'));
    
    // Combine header and data rows
    const tableData = [headerRow, ...dataRows].join('\n');
    
    try {
      await navigator.clipboard.writeText(tableData);
      setCopyNotification('Table copied successfully!');
      
      // Clear notification after 3 seconds
      setTimeout(() => {
        setCopyNotification(null);
      }, 3000);
    } catch {
      setCopyNotification('Failed to copy table');
      setTimeout(() => {
        setCopyNotification(null);
      }, 3000);
    }
  };

  return (
    <div className="space-y-4 animate-slide-in-right lg:w-1/2 lg:flex-shrink-0 lg:overflow-hidden lg:px-2 lg:box-border">
      {/* Copy Notification */}
      {copyNotification && (
        <div className="fixed top-4 right-4 z-50 bg-green-600 text-white px-4 py-2 rounded-lg shadow-lg flex items-center space-x-2 animate-slide-in-right">
          <Check size={16} />
          <span className="text-sm font-medium">{copyNotification}</span>
        </div>
      )}
      
      <div className="text-center space-y-2">
        <h2 className="text-2xl font-bold text-gray-900">OCR Results</h2>
        <p className="text-gray-600">Extracted data from your bill images</p>
      </div>

      <div className="border border-gray-200 rounded-lg p-6 space-y-4 shadow-lg backdrop-blur-sm bg-white/95 max-h-[70vh] overflow-y-auto">
        {isUploading ? (
          <div className="flex flex-col items-center justify-center py-8 space-y-4">
            <div className="relative">
              <Loader2 size={40} className="animate-spin text-blue-600" />
              <div className="absolute inset-0 animate-ping">
                <div className="w-10 h-10 border-2 border-blue-300 rounded-full opacity-30"></div>
              </div>
            </div>
            <div className="text-center">
              <p className="font-medium text-gray-900">Processing your bill images...</p>
              <p className="text-sm text-gray-500">This may take a few moments</p>
            </div>
          </div>
        ) : ocrResults ? (
          <div className="space-y-4 animate-scale-in">
            {ocrResults.success && ocrResults.bill_data ? (
              <div className="space-y-6">
                {/* Status Header */}
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                  <span className="text-sm font-medium text-green-700">Processing completed successfully</span>
                </div>

                {/* Bill Metadata Section */}
                <div className="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-lg p-4 border border-blue-200">
                  <div className="flex items-center space-x-2 mb-4">
                    <FileText size={18} className="text-blue-600" />
                    <h3 className="font-semibold text-gray-900">Bill Information</h3>
                  </div>
                  
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {/* Bill Number */}
                    <div className="space-y-1">
                      <div className="flex items-center space-x-1 text-sm text-gray-600">
                        <Hash size={14} />
                        <span className="font-medium">Bill Number</span>
                      </div>
                      <div className="flex items-center space-x-2">
                        <span className="text-sm text-gray-900">{ocrResults.bill_data.bill_meta.bill_number || 'N/A'}</span>
                        <Button variant="ghost" size="sm" className="h-6 w-6 p-0" onClick={() => copyToClipboard(ocrResults.bill_data?.bill_meta.bill_number || '')}>
                          <Copy size={12} />
                        </Button>
                      </div>
                    </div>

                    {/* Date */}
                    <div className="space-y-1">
                      <div className="flex items-center space-x-1 text-sm text-gray-600">
                        <Calendar size={14} />
                        <span className="font-medium">Date</span>
                      </div>
                      <span className="text-sm text-gray-900">{ocrResults.bill_data.bill_meta.bill_date || 'N/A'}</span>
                    </div>

                    {/* Total Amount */}
                    <div className="space-y-1">
                      <div className="flex items-center space-x-1 text-sm text-gray-600">
                        <DollarSign size={14} />
                        <span className="font-medium">Total Amount</span>
                      </div>
                      <span className="text-sm font-semibold text-gray-900">{ocrResults.bill_data.bill_meta.total_amount || 'N/A'}</span>
                    </div>

                    {/* VAT Amount */}
                    <div className="space-y-1">
                      <div className="flex items-center space-x-1 text-sm text-gray-600">
                        <DollarSign size={14} />
                        <span className="font-medium">VAT Amount</span>
                      </div>
                      <span className="text-sm text-gray-900">{ocrResults.bill_data.bill_meta.vat_amount || 'N/A'}</span>
                    </div>

                    {/* Seller */}
                    <div className="space-y-1">
                      <div className="flex items-center space-x-1 text-sm text-gray-600">
                        <Building size={14} />
                        <span className="font-medium">Seller</span>
                      </div>
                      <span className="text-sm text-gray-900">{ocrResults.bill_data.bill_meta.seller || 'N/A'}</span>
                    </div>

                    {/* Buyer */}
                    <div className="space-y-1">
                      <div className="flex items-center space-x-1 text-sm text-gray-600">
                        <User size={14} />
                        <span className="font-medium">Buyer</span>
                      </div>
                      <span className="text-sm text-gray-900">{ocrResults.bill_data.bill_meta.buyer || 'N/A'}</span>
                    </div>

                    {/* Payment Method */}
                    <div className="space-y-1">
                      <div className="flex items-center space-x-1 text-sm text-gray-600">
                        <CreditCard size={14} />
                        <span className="font-medium">Payment Method</span>
                      </div>
                      <span className="text-sm text-gray-900">{ocrResults.bill_data.bill_meta.payment_method || 'N/A'}</span>
                    </div>

                    {/* Address */}
                    <div className="space-y-1">
                      <div className="flex items-center space-x-1 text-sm text-gray-600">
                        <MapPin size={14} />
                        <span className="font-medium">Address</span>
                      </div>
                      <span className="text-sm text-gray-900 break-words">{ocrResults.bill_data.bill_meta.address || 'N/A'}</span>
                    </div>
                  </div>

                  {/* Tax Codes */}
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
                    <div className="space-y-1">
                      <span className="text-sm font-medium text-gray-600">Seller Tax Code</span>
                      <div className="flex items-center space-x-2">
                        <Pill variant="secondary" className="text-xs">{ocrResults.bill_data.bill_meta.seller_tax_code || 'N/A'}</Pill>
                        <Button variant="ghost" size="sm" className="h-6 w-6 p-0" onClick={() => copyToClipboard(ocrResults.bill_data?.bill_meta.seller_tax_code || '')}>
                          <Copy size={12} />
                        </Button>
                      </div>
                    </div>
                    <div className="space-y-1">
                      <span className="text-sm font-medium text-gray-600">Buyer Tax Code</span>
                      <div className="flex items-center space-x-2">
                        <Pill variant="secondary" className="text-xs">{ocrResults.bill_data.bill_meta.buyer_tax_code || 'N/A'}</Pill>
                        <Button variant="ghost" size="sm" className="h-6 w-6 p-0" onClick={() => copyToClipboard(ocrResults.bill_data?.bill_meta.buyer_tax_code || '')}>
                          <Copy size={12} />
                        </Button>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Line Items Section with Simple Overflow */}
                {ocrResults.bill_data.line_items && ocrResults.bill_data.line_items.length > 0 && (
                  <div className="bg-gradient-to-br from-orange-50 to-yellow-50 rounded-lg p-4 border border-orange-200">
                    <div className="flex items-center justify-between mb-4">
                      <div className="flex items-center space-x-2">
                        <FileSpreadsheet size={18} className="text-orange-600" />
                        <h3 className="font-semibold text-gray-900">Line Items ({ocrResults.bill_data.line_items.length})</h3>
                      </div>
                      <div className="flex items-center space-x-2">
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => copyLineItemsTable(ocrResults.bill_data!.line_items)}
                          className="bg-white hover:bg-orange-50 border-orange-300 text-orange-700 hover:border-orange-400"
                        >
                          <Copy size={16} className="mr-2" />
                          Copy Table
                        </Button>
                        <Button
                          size="sm"
                          onClick={() => handleDownloadExcel(ocrResults.bill_data!)}
                          className="bg-green-600 hover:bg-green-700"
                        >
                          <Download size={16} className="mr-2" />
                          Download Excel
                        </Button>
                      </div>
                    </div>
                    
                    <div className="overflow-x-auto">
                      <div className="bg-white rounded-lg border border-gray-200 overflow-hidden">
                        <div className="max-h-96 overflow-y-auto">
                          <Table>
                            <TableHeader className="bg-gray-50 sticky top-0 z-10">
                              <TableRow className="hover:bg-gray-50">
                                <TableHead className="w-16 text-center text-xs font-medium text-gray-700">No.</TableHead>
                                <TableHead className="min-w-48 text-xs font-medium text-gray-700">Product Name</TableHead>
                                <TableHead className="w-20 text-center text-xs font-medium text-gray-700">Quantity</TableHead>
                                <TableHead className="w-16 text-center text-xs font-medium text-gray-700">Unit</TableHead>
                                <TableHead className="w-24 text-right text-xs font-medium text-gray-700">Unit Price</TableHead>
                                <TableHead className="w-24 text-right text-xs font-medium text-gray-700">Subtotal</TableHead>
                              </TableRow>
                            </TableHeader>
                            <TableBody>
                              {ocrResults.bill_data.line_items.map((item, index) => (
                                <TableRow key={index} className={index % 2 === 0 ? 'bg-white hover:bg-gray-50' : 'bg-gray-50/50 hover:bg-gray-100'}>
                                  <TableCell className="text-center font-medium text-gray-900 text-sm">{item.no}</TableCell>
                                  <TableCell className="text-gray-900 text-sm break-words">{item.product_name}</TableCell>
                                  <TableCell className="text-center text-gray-700 text-sm">{item.quantity}</TableCell>
                                  <TableCell className="text-center text-gray-700 text-sm">{item.unit}</TableCell>
                                  <TableCell className="text-right text-gray-700 text-sm">{item.unit_price}</TableCell>
                                  <TableCell className="text-right font-medium text-gray-900 text-sm">{item.subtotal}</TableCell>
                                </TableRow>
                              ))}
                            </TableBody>
                          </Table>
                        </div>
                      </div>
                    </div>
                  </div>
                )}

                {/* Notes Section */}
                {ocrResults.bill_data.notes && (
                  <div className="bg-gradient-to-br from-purple-50 to-pink-50 rounded-lg p-4 border border-purple-200">
                    <h3 className="font-semibold text-gray-900 mb-2">Additional Notes</h3>
                    <p className="text-sm text-gray-700 whitespace-pre-wrap">{ocrResults.bill_data.notes}</p>
                  </div>
                )}

                {/* Processing Info */}
                {ocrResults.processing_timestamp && (
                  <div className="text-center pt-2 border-t">
                    <p className="text-xs text-gray-500">Processed at {new Date(ocrResults.processing_timestamp).toLocaleString()}</p>
                  </div>
                )}
              </div>
            ) : (
              <div className="space-y-4">
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
                  <span className="text-sm font-medium text-red-700">Processing failed</span>
                </div>
                <div className="bg-red-50 rounded-lg p-4 border border-red-200">
                  <p className="text-sm text-red-700">{ocrResults.error || 'Unknown error occurred'}</p>
                </div>
              </div>
            )}
          </div>
        ) : (
          <div className="text-center py-8">
            <p className="text-gray-500">Click &quot;Process Images&quot; to see results here</p>
          </div>
        )}
      </div>
    </div>
  );
}