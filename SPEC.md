# SwiftPOS Desktop Application
## Software Requirements Specification (SRS)

**Version:** 1.0  
**Date:** March 17, 2026  
**Status:** Final  

---

## 1. Introduction

### 1.1 Purpose

This document defines the complete requirements for developing SwiftPOS, a desktop Point of Sale (POS) application designed for retail businesses with multi-branch operations. The application enables store owners to manage products, process sales transactions, track inventory across branches, and generate reports—all with offline capability and cloud synchronization.

### 1.2 Scope

SwiftPOS is a desktop application built with Tauri v2 (Rust backend + WebView frontend) that provides:

- Multi-tenant architecture with strict data isolation
- Multi-branch support with per-branch inventory management
- Offline-first operations with automatic cloud synchronization
- Role-based access control for different user types
- Complete POS functionality for retail businesses

### 1.3 Definitions and Acronyms

| Term | Definition |
|------|------------|
| **Tenant** | A store/organization that subscribes to SwiftPOS |
| **Branch** | A physical location belonging to a tenant |
| **Kasir** | Indonesian term for Cashier |
| **HTMX** | HTML extensions for dynamic content without JavaScript |
| **NeonDB** | Serverless PostgreSQL database |
| **RBAC** | Role-Based Access Control |
| **STRUK** | Indonesian term for Receipt |

---

## 2. Overall Description

### 2.1 Product Perspective

SwiftPOS follows a desktop application architecture with the following components:

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                        │
│  ┌─────────────────┐    ┌─────────────────────────────┐   │
│  │   WebView UI    │    │      Rust Backend           │   │
│  │   (HTMX + CSS)  │◄──►│   (Business Logic)          │   │
│  └─────────────────┘    └──────────────┬──────────────┘   │
│                                         │                   │
│                    ┌─────────────────────┼─────────────────┐│
│                    │     Data Layer      │                 ││
│                    │  ┌───────────────┐  │  ┌────────────┐ ││
│                    │  │   NeonDB      │  │  │  SQLite    │ ││
│                    │  │ (Cloud Primary)│  │  │ (Local)    │ ││
│                    │  └───────────────┘  │  └────────────┘ ││
│                    └─────────────────────┴─────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Desktop Runtime | Tauri v2 | Cross-platform desktop executable |
| Frontend | HTML + HTMX | Dynamic UI without complex JavaScript |
| Styling | TailwindCSS | Responsive design |
| Backend | Rust | High-performance business logic |
| Primary Database | NeonDB (PostgreSQL) | Cloud-based data storage |
| Local Database | SQLite | Offline backup and operations |
| Authentication | JWT + Argon2 | Secure login |

### 2.3 User Characteristics

| User Role | Description | Typical User |
|-----------|-------------|--------------|
| **Super Admin** | System-wide administration | Platform owner |
| **Tenant Admin** | Full store management | Store owner/manager |
| **Branch Manager** | Single/multiple branch management | Branch supervisor |
| **Cashier (Kasir)** | Process transactions | Front-line staff |

---

## 3. Functional Requirements

### 3.1 Authentication Module

#### 3.1.1 Login System

| Requirement ID | Description |
|----------------|-------------|
| AUTH-001 | Users shall log in with email and password |
| AUTH-002 | Passwords shall be hashed using Argon2 algorithm |
| AUTH-003 | Sessions shall be managed via JWT tokens |
| AUTH-004 | Failed login attempts shall be tracked and account locked after 5 attempts |
| AUTH-005 | Users shall be able to reset password via email token |

#### 3.1.2 Role-Based Access Control

| Role | Permissions |
|------|-------------|
| **Super Admin** | Manage all tenants, system configuration |
| **Tenant Admin** | Full access within tenant, manage branches |
| **Branch Manager** | Manage assigned branches, view reports |
| **Cashier** | Process transactions only |

### 3.2 Dashboard Module (Admin)

#### 3.2.1 Dashboard Widgets

| Requirement ID | Description | Data Source |
|----------------|-------------|--------------|
| DASH-001 | Display today's revenue | `SUM(transactions.total_amount)` where date = today |
| DASH-002 | Display total transactions count | `COUNT(transactions)` where date = today |
| DASH-003 | Display average transaction value | `AVG(transactions.total_amount)` |
| DASH-004 | Display total products count | `COUNT(products)` per branch |

### 3.3 Products Module

#### 3.3.1 Product Management

| Requirement ID | Description | Data Fields |
|----------------|-------------|-------------|
| PROD-001 | Display product list | Image, Name, Category, Price, Stock, Status |
| PROD-002 | Create new product | SKU, Name, Category, Price, Cost, Image, Tax rate |
| PROD-003 | Edit existing product | All product fields |
| PROD-004 | Delete (deactivate) product | Soft delete via is_active flag |
| PROD-005 | Search products by name/SKU/barcode | Full-text search |
| PROD-006 | Filter products by category | Category relationship |

#### 3.3.2 Category Management

| Requirement ID | Description |
|----------------|-------------|
| CAT-001 | Create/Edit/Delete categories |
| CAT-002 | Display product count per category |
| CAT-003 | Support category hierarchy (parent/child) |
| CAT-004 | Reorder categories via drag-and-drop |

#### 3.3.3 Inventory Management

| Requirement ID | Description |
|----------------|-------------|
| INV-001 | Track stock per branch |
| INV-002 | Adjust stock with reason (restock, damage, correction) |
| INV-003 | Set minimum stock alerts |
| INV-004 | Override product price per branch |
| INV-005 | Branch-specific barcode support |

### 3.4 Transactions Module (Kasir)

#### 3.4.1 POS Interface

| Requirement ID | Description | UI Element |
|----------------|-------------|-------------|
| TXN-001 | Display product grid with images | Product cards |
| TXN-002 | Filter products by category | Category buttons |
| TXN-003 | Add product to cart | Click product |
| TXN-004 | Adjust item quantity | Number input |
| TXN-005 | Remove item from cart | Delete button |
| TXN-006 | Enter customer name | Text input "Nama Pelanggan" |
| TXN-007 | Calculate subtotal, tax, total | Automatic |
| TXN-008 | Generate transaction number | Auto-increment format |

#### 3.4.2 Payment Processing

| Requirement ID | Description | Payment Methods |
|----------------|-------------|-----------------|
| PMT-001 | Process cash payment | Tunai |
| PMT-002 | Process QR Code payment | QR (QRIS) |
| PMT-003 | Process debit card | Kartu Debit |
| PMT-004 | Process credit card | Kartu Kredit |
| PMT-005 | Calculate change | Automatic |

#### 3.4.3 Transaction Operations

| Requirement ID | Description |
|----------------|-------------|
| TXN-009 | Complete transaction |
| TXN-010 | Void transaction (same day, requires reason) |
| TXN-011 | Refund transaction (requires original receipt) |
| TXN-012 | Reprint receipt |

### 3.5 Users Module (Admin)

| Requirement ID | Description | Data Fields |
|----------------|-------------|-------------|
| USER-001 | Display user list | Name, Email, Store, Role, Status |
| USER-002 | Create new user | Name, Email, Password, Role, Branch |
| USER-003 | Edit user | All user fields |
| USER-004 | Deactivate user | is_active flag |
| USER-005 | Assign user to branches | Branch assignment table |

### 3.6 Branches Module (Admin)

| Requirement ID | Description | Data Fields |
|----------------|-------------|-------------|
| BRANCH-001 | Display branch list | Code, Name, Address, Status |
| BRANCH-002 | Create new branch | Code, Name, Address, Phone, Operating hours |
| BRANCH-003 | Edit branch | All branch fields |
| BRANCH-004 | Set main branch | is_main_branch flag |
| BRANCH-005 | Configure operating hours | Open/Close time |

### 3.7 Reports Module

| Requirement ID | Description | Output |
|----------------|-------------|--------|
| RPT-001 | Daily sales report | Table: Product, Qty Sold, Revenue |
| RPT-002 | Weekly sales report | Summary + comparison |
| RPT-003 | Monthly sales report | Summary + trends |
| RPT-004 | Per-branch report | Branch breakdown |
| RPT-005 | Consolidated report | All branches combined |
| RPT-006 | Export to Excel | .xlsx download |
| RPT-007 | Export to PDF | .pdf download |
| RPT-008 | Date range filter | Start/End date |

### 3.8 Settings Module

| Requirement ID | Description | Fields |
|----------------|-------------|--------|
| SET-001 | General settings | Application name |
| SET-002 | Store profile | Name, Logo, Address, Phone, Email |
| SET-003 | Receipt settings | Header, Footer, Show logo |
| SET-004 | Tax settings | Default tax rate |

### 3.9 Printer Module

#### 3.9.1 Printer Types Supported

| Requirement ID | Description | Printer Type |
|----------------|-------------|--------------|
| PRT-001 | Support dot matrix printers | Dot Matrix |
| PRT-002 | Support inkjet printers | Inkjet |
| PRT-003 | Support Bluetooth printers | Bluetooth |
| PRT-004 | Support thermal printers | Thermal |
| PRT-005 | Support label printers | Label |

#### 3.9.2 Connection Methods

| Requirement ID | Description | Connection |
|----------------|-------------|------------|
| PRT-006 | Support USB connection | USB |
| PRT-007 | Support Bluetooth connection | Bluetooth |
| PRT-008 | Support network/Ethernet connection | Network |
| PRT-009 | Support serial/COM port connection | Serial |
| PRT-010 | Support WiFi connection | WiFi |
| PRT-011 | Support virtual printer (PDF) | Virtual |

#### 3.9.3 Printer Management

| Requirement ID | Description |
|----------------|-------------|
| PRT-012 | Auto-detect connected printers |
| PRT-013 | Save printer configuration per branch |
| PRT-014 | Test print functionality |
| PRT-015 | Set default printer |
| PRT-016 | Configure paper size (58mm, 80mm, A4) |

#### 3.9.4 Receipt Printing

| Requirement ID | Description |
|----------------|-------------|
| PRT-017 | Print receipt automatically after transaction |
| PRT-018 | Support ESC/POS commands for thermal printers |
| PRT-019 | Support plain text for dot matrix |
| PRT-020 | Support graphics for inkjet/laser |
| PRT-021 | Reprint last receipt |
| PRT-022 | Print Kitchen Order Ticket (KOT) |

---

## 4. Data Requirements

### 4.1 Database Schema Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     TENANTS (Multi-tenant)                      │
│  id, name, slug, address, phone, email, logo_url...           │
└─────────────────────┬───────────────────────────────────────────┘
                      │ 1:N
┌─────────────────────┴───────────────────────────────────────────┐
│                         BRANCHES                                 │
│  id, tenant_id, code, name, address, phone, is_main_branch    │
└─────────────────────┬───────────────────────────────────────────┘
                      │ 1:N
        ┌─────────────┴─────────────┐
        │                           │
┌───────┴───────┐         ┌─────────┴────────┐
│ BRANCH_PRODUCTS│         │  TRANSACTIONS    │
│ (inventory)   │         │  id, number...   │
└───────┬───────┘         └─────────┬────────┘
        │                           │ 1:N
┌───────┴───────┐         ┌─────────┴────────┐
│   PRODUCTS    │         │ TRANSACTION_ITEMS│
│  (catalog)    │         └─────────┬────────┘
└───────┬───────┘                   │
        │ 1:N                       │
┌───────┴───────┐         ┌─────────┴────────┐
│  CATEGORIES   │         │    PAYMENTS      │
└───────────────┘         └──────────────────┘
```

### 4.2 Core Tables

| Table | Purpose | Key Fields |
|-------|---------|------------|
| `tenants` | Store/Organization | name, slug, address, phone, email, logo_url |
| `branches` | Store locations | tenant_id, code, name, address, is_main_branch |
| `categories` | Product categories | tenant_id, name, parent_id, sort_order |
| `products` | Product catalog | tenant_id, category_id, sku, name, price, image_url |
| `branch_products` | Per-branch inventory | branch_id, product_id, current_stock, price_override |
| `users` | User accounts | tenant_id, email, password_hash, role, branch_id |
| `transactions` | Sales transactions | branch_id, user_id, transaction_number, total_amount, status |
| `transaction_items` | Line items | transaction_id, product_id, quantity, unit_price, total |
| `payments` | Payment records | transaction_id, method, amount |
| `settings` | Configuration | tenant_id, branch_id, setting_key, setting_value |
| `printers` | Printer configurations | tenant_id, branch_id, name, type, connection_type, config |

### 4.3 Printer Configuration Table

### 4.3 Printer Configuration Table

| Table | Purpose | Key Fields |
|-------|---------|------------|
| `printers` | Printer configurations | tenant_id, branch_id, name, type, connection_type, config |

```sql
-- PRINTERS TABLE
CREATE TABLE printers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    branch_id UUID REFERENCES branches(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    printer_type VARCHAR(50) NOT NULL CHECK (printer_type IN ('dot_matrix', 'inkjet', 'thermal', 'laser', 'label')),
    connection_type VARCHAR(50) NOT NULL CHECK (connection_type IN ('usb', 'bluetooth', 'network', 'serial', 'wifi', 'virtual')),
    is_default BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    -- Connection configuration
    device_path VARCHAR(255),        -- USB path, COM port, Bluetooth address
    ip_address VARCHAR(45),          -- Network IP
    port INTEGER DEFAULT 9100,       -- Network port
    bluetooth_address VARCHAR(50),  -- Bluetooth MAC address
    -- Printer settings
    paper_width_mm INTEGER DEFAULT 80,
    character_per_line INTEGER DEFAULT 48,
    auto_cut BOOLEAN DEFAULT true,
    config JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_printers_tenant ON printers(tenant_id);
CREATE INDEX idx_printers_branch ON printers(branch_id);
CREATE INDEX idx_printers_type ON printers(printer_type);
CREATE INDEX idx_printers_connection ON printers(connection_type);
```

### 4.4 Offline Support Tables

| Table | Purpose |
|-------|---------|
| `offline_queue` | Pending sync operations |
| `local_transactions` | Cached transactions |
| `local_products` | Cached product catalog |
| `sync_metadata` | Sync status tracking |

---

## 5. Non-Functional Requirements

### 5.1 Performance

| Requirement | Target |
|-------------|--------|
| Application startup time | < 3 seconds |
| Transaction processing | < 500ms |
| Product search response | < 200ms |
| Report generation | < 5 seconds |

### 5.2 Security

| Requirement | Implementation |
|-------------|----------------|
| Password hashing | Argon2 |
| SQL injection prevention | Parameterized queries |
| Data encryption | AES-256 for sensitive data |
| Session management | JWT with expiration |
| Input validation | Server-side validation |

### 5.3 Availability

| Requirement | Implementation |
|-------------|----------------|
| Offline operations | SQLite local database |
| Auto-sync | Background sync when online |
| Conflict resolution | Automatic + manual override |

### 5.4 Cross-Platform

| Platform | Installer |
|----------|-----------|
| Windows | NSIS (.exe), MSI (.msi) |
| macOS | DMG, App bundle |
| Linux | DEB, RPM |

---

## 6. UI/UX Requirements

### 6.1 Admin Interface

| Screen | Components |
|--------|------------|
| Login | Email, Password, Login button |
| Dashboard | Revenue card, Transaction count, Product count |
| Products | Table with image, name, category, price, stock, status |
| Categories | Table with name, product count, status |
| Users | Table with name, email, store, role, status |
| Branches | Table with code, name, address, status |
| Transactions | Table with number, time, customer, total, payment, status |
| Reports | Date filter, Period select, Export buttons, Data table |
| Settings | Form inputs for store info, receipt, tax |

### 6.2 Kasir (Cashier) Interface

| Component | Description |
|-----------|-------------|
| Header | Store name, Cashier name, Date, Navigation menu |
| Product Grid | Product cards with image, name, price |
| Category Filter | Buttons: Semua, Mie, Nasi, Minuman, Extra |
| Transaction Panel | Customer name, Cart items, Qty, Subtotal, PPN, Total |
| Payment Buttons | Tunai, QR, Debit, Kredit |
| Receipt | Store info, Items, Totals, Payment details |

---

## 7. Deployment Requirements

### 7.1 Build Commands

```bash
# Development
npm run tauri dev

# Production (current platform)
npm run tauri build

# Platform-specific
npm run tauri build -- --target x86_64-pc-windows-msvc    # Windows
npm run tauri build -- --target x86_64-apple-darwin      # macOS
npm run tauri build -- --target x86_64-unknown-linux-gnu  # Linux
```

### 7.2 Database Migration

| Step | Command |
|------|---------|
| Install CLI | `cargo install sqlx-cli` |
| Create migration | `sqlx migrate add -r <name>` |
| Run migrations | `sqlx migrate run` |
| Revert | `sqlx migrate revert` |

---

## 8. Acceptance Criteria

### 8.1 Authentication
- [ ] User can log in with valid credentials
- [ ] Invalid credentials show error message
- [ ] Account locks after 5 failed attempts

### 8.2 Dashboard
- [ ] Today's revenue displays correctly
- [ ] Transaction count is accurate
- [ ] Product count reflects inventory

### 8.3 Products
- [ ] Products display in table with all fields
- [ ] Create/Edit/Delete operations work
- [ ] Search and filter function correctly

### 8.4 Kasir (Transactions)
- [ ] Products display in grid
- [ ] Adding to cart works
- [ ] Quantity adjustment works
- [ ] All payment methods process correctly
- [ ] Receipt generates properly

### 8.5 Reports
- [ ] Date filter works
- [ ] Data displays correctly
- [ ] Export to Excel/PDF works

### 8.6 Offline Mode
- [ ] Application works without internet
- [ ] Transactions save locally
- [ ] Sync occurs when connection restored

### 8.7 Printer Module
- [ ] Auto-detect connected printers
- [ ] Support USB printer connection
- [ ] Support Bluetooth printer connection
- [ ] Support network printer connection
- [ ] Support serial/COM port connection
- [ ] Print receipt after transaction
- [ ] Support thermal printer (ESC/POS)
- [ ] Support dot matrix printer (plain text)
- [ ] Configure paper size (58mm, 80mm)
- [ ] Test print functionality

---

## 9. Appendix

### 9.1 Database Connection (NeonDB)

```env
# .env
DATABASE_URL=postgresql://user:password@host.neon.tech/dbname?sslmode=require
```

### 9.2 Tauri Configuration

```json
{
  "productName": "SwiftPOS",
  "version": "1.0.0",
  "identifier": "com.swiftpos.app",
  "build": {
    "devtools": true
  }
}
```

### 9.3 HTMX Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/dashboard/stats` | GET | Dashboard data |
| `/api/products` | GET, POST | Products CRUD |
| `/api/categories` | GET, POST | Categories CRUD |
| `/api/transactions` | GET, POST | Transactions |
| `/api/transactions/:id/void` | POST | Void transaction |
| `/api/reports/sales` | GET | Sales report |
| `/api/settings` | GET, PUT | Settings |

---

**Document Prepared:** March 17, 2026  
**Project:** SwiftPOS Desktop POS Application  
**Technology:** Tauri v2 + HTMX + NeonDB + SQLite
