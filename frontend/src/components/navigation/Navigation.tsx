"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Upload, FileText, Home } from "lucide-react";

export function Navigation() {
  const pathname = usePathname();

  const navItems = [
    {
      href: "/",
      label: "Upload",
      icon: Upload,
      description: "Tải lên và xử lý hóa đơn"
    },
    {
      href: "/bills",
      label: "Quản lý hóa đơn",
      icon: FileText,
      description: "Xem danh sách hóa đơn đã xử lý"
    }
  ];

  return (
    <Card className="p-4 mb-8">
      <nav className="flex flex-col sm:flex-row gap-4">
        {navItems.map((item) => {
          const isActive = pathname === item.href;
          const Icon = item.icon;

          return (
            <Link key={item.href} href={item.href} className="flex-1">
              <Button
                variant={isActive ? "default" : "outline"}
                className="w-full h-auto flex-col sm:flex-row gap-2 p-4"
              >
                <Icon className="h-5 w-5" />
                <div className="text-center sm:text-left">
                  <div className="font-medium">{item.label}</div>
                  <div className="text-xs opacity-75 hidden sm:block">
                    {item.description}
                  </div>
                </div>
              </Button>
            </Link>
          );
        })}
      </nav>
    </Card>
  );
}