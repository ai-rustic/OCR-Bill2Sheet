"use client";

import { useState } from "react";
import { BillsTable } from "@/components/bills/BillsTable";
import { Navigation } from "@/components/navigation/Navigation";
import { Bill } from "@/lib/api";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function BillsPage() {
  const [selectedBill, setSelectedBill] = useState<Bill | null>(null);

  const handleViewBill = (bill: Bill) => {
    setSelectedBill(bill);
    // TODO: Implement view bill modal/drawer
    console.log("View bill:", bill);
  };

  const handleEditBill = (bill: Bill) => {
    // TODO: Implement edit bill modal/form
    console.log("Edit bill:", bill);
  };

  const handleDeleteBill = (billId: number) => {
    console.log("Bill deleted:", billId);
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-7xl mx-auto space-y-8">
        {/* Header */}
        <div className="text-center">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">
            Quản lý hóa đơn
          </h1>
          <p className="text-gray-600">
            Xem và quản lý các hóa đơn đã được xử lý bởi hệ thống OCR
          </p>
        </div>

        {/* Navigation */}
        <Navigation />

        {/* Bills Table */}
        <BillsTable
          onViewBill={handleViewBill}
          onEditBill={handleEditBill}
          onDeleteBill={handleDeleteBill}
        />
      </div>
    </div>
  );
}