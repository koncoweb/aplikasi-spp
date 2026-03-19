# SwiftPOS Implementation Plan
## Step-by-Step Development Roadmap

---

## Using Neon MCP Server

This project uses the **Neon MCP Server** for direct database operations with NeonDB. The MCP server is configured in `.kilocode/mcp.json`.

**MCP Server Endpoint:** `https://mcp.neon.tech/sse`

**Usage Examples:**
```bash
# Execute a query
npx -y mcp-remote "https://mcp.neon.tech/sse" --query "SELECT * FROM tenants"

# Run migrations via MCP
# (See Phase 1.2 Database Setup for details)
```

**When to use MCP vs sqlx-cli:**
| Method | Use Case |
|--------|----------|
| Neon MCP | Quick queries, testing, migrations during development |
| sqlx-cli | CI/CD pipelines, production migrations |
| Rust code | Application runtime database operations |

---

## Phase 1: Project Foundation (Week 1-2)

### 1.1 Environment Setup
**Duration:** 2 days

**Alternative Paths:**

If you already have the tools installed, verify they meet the minimum requirements:

| Check | Minimum Version | Command to Verify |
|-------|----------------|-------------------|
| Rust | 1.70+ | `rustc --version` |
| Node.js | 18+ | `node --version` |
| npm | 9+ | `npm --version` |
| Visual Studio Build Tools (Windows) | 2022 | Check installed programs |

**Skip Installation - Go Directly To:**

| Already Installed | Skip To Task |
|-----------------|---------------|
| Rust + Node.js | Tauri-004 |
| Tauri CLI | Tauri-004 |
| VS Build Tools | Tauri-004 |
| All tools ready | Tauri-007 |

**Setup Tasks:**

| Task | Description | Dependencies | Alternative |
|------|-------------|--------------|-------------|
| Tauri-001 | Install Rust toolchain (rustup) | None | **Skip if**: `rustc --version` shows 1.70+ |
| Tauri-002 | Install Node.js (v18+) and npm | None | **Skip if**: `node --version` shows 18+ |
| Tauri-003 | Install Visual Studio Build Tools (Windows) | None | **Skip if**: VS 2022 already installed |
| Tauri-004 | Create Tauri v2 project with `npm create tauri-app` | Tauri-001, Tauri-002 | Verify with `npm list -g @tauri-apps/cli` |
| Tauri-005 | Configure Tauri with project name "SwiftPOS" | Tauri-004 | None |
| Tauri-006 | Set up logging (tracing crate) | Tauri-005 | None |
| Tauri-007 | Verify empty shell builds successfully | Tauri-006 | None |

**Verification:** `npm run tauri build` produces executable

### 1.2 Database Setup
**Duration:** 3 days

**Using Neon MCP Server:**

For NeonDB operations, use the configured Neon MCP server (already set up in `.kilocode/mcp.json`). The MCP server provides direct access to NeonDB for executing queries and migrations.

| Task | Description | Dependencies | Neon MCP Usage |
|------|-------------|--------------|----------------|
| DB-001 | Create NeonDB account and project | None | Use Neon console or MCP server to create project |
| DB-002 | Set up database connection string | DB-001 | Get connection string from Neon dashboard |
| DB-003 | Install sqlx-cli for migrations | Tauri-001 | Alternative: Use Neon MCP for migrations |
| DB-004 | Create migration: tenants table | DB-003 | Execute via Neon MCP or sqlx-cli |
| DB-005 | Create migration: branches table | DB-004 | Execute via Neon MCP or sqlx-cli |
| DB-006 | Create migration: users table | DB-005 | Execute via Neon MCP or sqlx-cli |
| DB-007 | Create migration: categories table | DB-006 | Execute via Neon MCP or sqlx-cli |
| DB-008 | Create migration: products table | DB-007 | Execute via Neon MCP or sqlx-cli |
| DB-009 | Create migration: branch_products table | DB-008 | Execute via Neon MCP or sqlx-cli |
| DB-010 | Create migration: transactions table | DB-009 | Execute via Neon MCP or sqlx-cli |
| DB-011 | Create migration: transaction_items table | DB-010 | Execute via Neon MCP or sqlx-cli |
| DB-012 | Create migration: payments table | DB-011 | Execute via Neon MCP or sqlx-cli |
| DB-013 | Create migration: settings table | DB-012 | Execute via Neon MCP or sqlx-cli |
| DB-014 | Create migration: printers table | DB-013 | Execute via Neon MCP or sqlx-cli |
| DB-015 | Run all migrations on NeonDB | DB-014 | Use Neon MCP `query` tool |

**Neon MCP Server Commands:**

```bash
# Using Neon MCP for database operations
# The MCP server is configured at: .kilocode/mcp.json

# Execute SQL via MCP server
npx -y mcp-remote "https://mcp.neon.tech/sse" --query "SELECT version()"
```

**Alternative:** Use sqlx-cli directly:
```bash
# Set DATABASE_URL and run migrations
sqlx migrate run
```

**Verification:** All tables created in NeonDB

### 1.3 SQLite Local Database
**Duration:** 2 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| SQL-001 | Design SQLite schema for offline | DB-015 |
| SQL-002 | Implement SQLite connection in Rust | SQL-001 |
| SQL-003 | Create offline_queue table | SQL-002 |
| SQL-004 | Create local_products cache table | SQL-003 |
| SQL-005 | Create local_transactions cache table | SQL-004 |

**Verification:** SQLite database created in app data folder

---

## Phase 2: Core Backend (Week 3-4)

### 2.1 Database Layer
**Duration:** 3 days

**Neon MCP Server for Development:**

Use the Neon MCP server for quick database queries and testing during development:

```bash
# Query NeonDB directly during development
npx -y mcp-remote "https://mcp.neon.tech/sse" --query "SELECT * FROM tenants"
```

| Task | Description | Dependencies |
|------|-------------|--------------|
| CORE-001 | Create db module structure | SQL-005 |
| CORE-002 | Implement connection pool for NeonDB | CORE-001 |
| CORE-003 | Implement SQLite connection | CORE-002 |
| CORE-004 | Create CRUD traits for all tables | CORE-003 |
| CORE-005 | Add tenant_id filtering to all queries | CORE-004 |

**Verification:** All database operations work with tenant isolation

### 2.2 Authentication
**Duration:** 4 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| AUTH-001 | Create User model | CORE-005 |
| AUTH-002 | Implement password hashing (Argon2) | AUTH-001 |
| AUTH-003 | Create JWT token generation | AUTH-002 |
| AUTH-004 | Implement login command | AUTH-003 |
| AUTH-005 | Implement logout command | AUTH-004 |
| AUTH-006 | Create auth middleware | AUTH-005 |
| AUTH-007 | Add role-based permission checks | AUTH-006 |

**Verification:** Users can log in and receive JWT token

### 2.3 Multi-Tenant Middleware
**Duration:** 2 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| MT-001 | Extract tenant_id from JWT | AUTH-007 |
| MT-002 | Add tenant context to requests | MT-001 |
| MT-003 | Implement branch context | MT-002 |

**Verification:** All queries filtered by tenant and branch

---

## Phase 3: Frontend Foundation (Week 5-6)

### 3.1 HTML/HTMX Setup
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| UI-001 | Set up TailwindCSS | None |
| UI-002 | Create base HTML template | UI-001 |
| UI-003 | Configure HTMX | UI-002 |
| UI-004 | Create layout component (header, sidebar) | UI-003 |
| UI-005 | Create login page | UI-004 |

**Verification:** Login page renders correctly

### 3.2 Navigation & Routing
**Duration:** 2 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| UI-006 | Create admin navigation menu | UI-005 |
| UI-007 | Create kasir navigation menu | UI-006 |
| UI-008 | Implement page routing | UI-007 |

**Verification:** Navigation between pages works

### 3.3 Common Components
**Duration:** 2 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| UI-009 | Create data table component | UI-008 |
| UI-010 | Create form components | UI-009 |
| UI-011 | Create modal component | UI-010 |
| UI-012 | Create toast notification system | UI-011 |

**Verification:** Components work with HTMX

---

## Phase 4: Admin Module (Week 7-10)

### 4.1 Dashboard
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-001 | Create dashboard layout | UI-012 |
| ADMIN-002 | Implement revenue widget | ADMIN-001 |
| ADMIN-003 | Implement transaction count widget | ADMIN-002 |
| ADMIN-004 | Implement product count widget | ADMIN-003 |
| ADMIN-005 | Add auto-refresh (HTMX) | ADMIN-004 |

**Verification:** Dashboard shows live statistics

### 4.2 Categories Management
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-006 | Create categories list page | ADMIN-005 |
| ADMIN-007 | Implement add category form | ADMIN-006 |
| ADMIN-008 | Implement edit category form | ADMIN-007 |
| ADMIN-009 | Implement delete category | ADMIN-008 |

**Verification:** Full CRUD for categories

### 4.3 Products Management
**Duration:** 5 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-010 | Create products list page | ADMIN-009 |
| ADMIN-011 | Implement product fields (HPP, Jual, Supplier, Satuan) | ADMIN-010 |
| ADMIN-012 | Implement add product form with stock tracking | ADMIN-011 |
| ADMIN-013 | Implement edit product form | ADMIN-012 |
| ADMIN-014 | Implement delete product | ADMIN-013 |
| ADMIN-015 | Add search functionality | ADMIN-014 |
| ADMIN-016 | Add unit filter dropdown | ADMIN-015 |

**Verification:** Full CRUD for products with accurate stock metrics (Awal, Tambahan, Terjual, Total)

### 4.4 Inventory Management
**Duration:** 4 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-017 | Create branch products view | ADMIN-016 |
| ADMIN-018 | Implement stock adjustment | ADMIN-017 |
| ADMIN-019 | Add low stock alerts | ADMIN-018 |
| ADMIN-020 | Implement price override per branch | ADMIN-019 |

**Verification:** Inventory management works per branch

### 4.5 Users Management
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-021 | Create users list page | ADMIN-020 |
| ADMIN-022 | Implement add user form | ADMIN-021 |
| ADMIN-023 | Implement edit user form | ADMIN-022 |
| ADMIN-024 | Implement role assignment | ADMIN-023 |

**Verification:** User management with RBAC

### 4.6 Unit Management (Branches)
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-025 | Create unit cards grid view | ADMIN-024 |
| ADMIN-026 | Implement add unit form | ADMIN-025 |
| ADMIN-027 | Implement edit/delete unit actions | ADMIN-026 |

**Verification:** Unit management works showcasing product totals and daily sales

### 4.7 Settings
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-028 | Create settings page | ADMIN-027 |
| ADMIN-029 | Implement store info editing | ADMIN-028 |
| ADMIN-030 | Implement receipt settings | ADMIN-029 |
| ADMIN-031 | Implement logo upload | ADMIN-030 |

**Verification:** Settings save correctly

### 4.8 Transactions Management (Admin)
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-032 | Create transactions list page | ADMIN-031 |
| ADMIN-033 | Implement filters (Type, Status, Date) | ADMIN-032 |
| ADMIN-034 | Implement export data functionality | ADMIN-033 |

**Verification:** Admin can view, filter, and export all transactions

### 4.9 Piutang Management
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| ADMIN-035 | Create piutang list page | ADMIN-034 |
| ADMIN-036 | Implement add piutang form | ADMIN-035 |
| ADMIN-037 | Implement pay installment (cicilan) | ADMIN-036 |
| ADMIN-038 | Implement piutang filters & search | ADMIN-037 |

**Verification:** Piutang records created and tracked correctly

---

## Phase 5: Kasir Module (Week 11-13)

### 5.1 POS Interface
**Duration:** 5 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| KASIR-001 | Create main POS layout | ADMIN-031 |
| KASIR-002 | Implement product grid | KASIR-001 |
| KASIR-003 | Add category filter buttons | KASIR-002 |
| KASIR-004 | Implement add to cart | KASIR-003 |
| KASIR-005 | Implement quantity adjustment | KASIR-004 |
| KASIR-006 | Implement remove from cart | KASIR-005 |
| KASIR-007 | Calculate totals (subtotal, tax, total) | KASIR-006 |

**Verification:** Products display and cart works

### 5.2 Transaction Processing
**Duration:** 4 days

| Task | Description | Status | Dependencies |
|------|-------------|--------|--|
| KASIR-008 | Implement Tunai cash payment modal | ✅ UI Done | KASIR-007 |
| KASIR-009 | Implement Transfer payment modal | ✅ UI Done | KASIR-008 |
| KASIR-010 | Implement Kredit payment modal (with DP + Sisa Piutang) | ✅ UI Done | KASIR-009 |
| KASIR-011 | Implement QR payment | ⏳ Pending | KASIR-010 |
| KASIR-012 | Implement debit card payment | ⏳ Pending | KASIR-011 |
| KASIR-013 | Calculate change | ✅ UI Done | KASIR-012 |

**Verification:** All payment methods work

### 5.3 Receipt Generation
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| KASIR-013 | Create receipt template | KASIR-012 |
| KASIR-014 | Auto-print after transaction | KASIR-013 |
| KASIR-015 | Implement reprint receipt | KASIR-014 |

**Verification:** Receipts print correctly

### 5.4 Transaction Management
**Duration:** 2 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| KASIR-016 | Create transactions list | KASIR-015 |
| KASIR-017 | Implement void transaction | KASIR-016 |
| KASIR-018 | Implement refund transaction | KASIR-017 |

**Verification:** Transaction operations work

### 5.5 Kasir Operations (Non-Transaction)
**Duration:** 3 days

| Task | Description | Status | Dependencies |
|------|-------------|--------|--|
| KASIR-019 | Implement Buka Kasir modal (initial capital) | ✅ UI Done | KASIR-018 |
| KASIR-020 | Implement Kas Masuk with summary stats | ✅ UI Done | KASIR-019 |
| KASIR-021 | Implement Pengeluaran with history list | ✅ UI Done | KASIR-020 |
| KASIR-022 | Implement Tambah Stok (Stock add) | ✅ UI Done | KASIR-021 |
| KASIR-023 | Implement Bayar Piutang (customer payment) | ✅ UI Done | KASIR-022 |
| KASIR-024 | Implement Manajemen Piutang (overview modal) | ✅ UI Done | KASIR-023 |
| KASIR-025 | Implement Tambah Piutang Manual (entry form) | ✅ UI Done | KASIR-024 |
| KASIR-026 | Implement Tutup Kasir Dashboard (full breakdown) | ✅ UI Done | KASIR-025 |
| KASIR-027 | Implement Laporan Tutup Kasir page (printable) | ✅ UI Done | KASIR-026 |
| KASIR-028 | Implement Tambah Kas Baru | ⏳ Pending | KASIR-027 |
| KASIR-029 | Implement Pengaturan | ⏳ Pending | KASIR-028 |
| KASIR-030 | Wire Kredit payment DP + Sisa Piutang fields to backend | ⏳ Pending | KASIR-014 |
| KASIR-031 | Wire Tutup Kasir export to PDF | ⏳ Pending | KASIR-026 |
| KASIR-032 | Wire Tutup Kasir export to Google Sheets | ⏳ Pending | KASIR-026 |
| KASIR-033 | Backend: piutang CRUD (create, list, update status) | ⏳ Pending | CORE-005 |

**Verification:** All Non-Transaction operations work perfectly and reflect in reports and cash logs.

---

## Phase 6: Reports Module (Week 14-15)

### 6.1 Sales Reports
**Duration:** 4 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| RPT-001 | Create sales report page | KASIR-018 |
| RPT-002 | Implement date filter | RPT-001 |
| RPT-003 | Implement period selection | RPT-002 |
| RPT-004 | Add branch filter | RPT-003 |
| RPT-005 | Generate report data | RPT-004 |

**Verification:** Sales reports display correctly

### 6.2 Export Functionality
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| RPT-006 | Implement Excel export | RPT-005 |
| RPT-007 | Implement PDF export | RPT-006 |

**Verification:** Files download correctly

### 6.3 Additional Reports
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| RPT-008 | Create inventory report | RPT-007 |
| RPT-009 | Create financial summary | RPT-008 |

**Verification:** All reports work

---

## Phase 7: Printer Support (Week 16-17)

### 7.1 Printer Detection
**Duration:** 4 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| PRT-001 | Implement USB printer detection | RPT-009 |
| PRT-002 | Implement Bluetooth detection | PRT-001 |
| PRT-003 | Implement network printer detection | PRT-002 |
| PRT-004 | Implement serial port detection | PRT-003 |

**Verification:** Printers auto-detected

### 7.2 Printer Configuration
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| PRT-005 | Create printer settings page | PRT-004 |
| PRT-006 | Implement connection setup | PRT-005 |
| PRT-007 | Add test print functionality | PRT-006 |

**Verification:** Printers configured successfully

### 7.3 Receipt Printing
**Duration:** 4 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| PRT-008 | Implement ESC/POS for thermal | PRT-007 |
| PRT-009 | Implement text mode for dot matrix | PRT-008 |
| PRT-010 | Add paper size configuration | PRT-009 |

**Verification:** Receipts print on all printer types

---

## Phase 8: Offline Support (Week 18-19)

### 8.1 Sync Engine
**Duration:** 5 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| SYNC-001 | Implement network detection | PRT-010 |
| SYNC-002 | Create offline queue manager | SYNC-001 |
| SYNC-003 | Implement sync worker | SYNC-002 |
| SYNC-004 | Add retry logic | SYNC-003 |

**Verification:** Offline operations queue correctly

### 8.2 Conflict Resolution
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| SYNC-005 | Implement automatic conflict resolution | SYNC-004 |
| SYNC-006 | Add manual conflict override | SYNC-005 |
| SYNC-007 | Create conflict log | SYNC-006 |

**Verification:** Conflicts handled gracefully

---

## Phase 9: Testing & Polish (Week 20-21)

### 9.1 Integration Testing
**Duration:** 5 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| TEST-001 | Test all CRUD operations | SYNC-007 |
| TEST-002 | Test authentication flow | TEST-001 |
| TEST-003 | Test transaction flow | TEST-002 |
| TEST-004 | Test offline mode | TEST-003 |
| TEST-005 | Test printer functionality | TEST-004 |

### 9.2 Bug Fixes
**Duration:** 4 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| TEST-006 | Fix critical bugs | TEST-005 |
| TEST-007 | Fix UI issues | TEST-006 |

---

## Phase 10: Deployment (Week 22)

### 10.1 Build Configuration
**Duration:** 3 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| DEPLOY-001 | Configure production build | TEST-007 |
| DEPLOY-002 | Set up code signing (optional) | DEPLOY-001 |
| DEPLOY-003 | Build Windows executable | DEPLOY-002 |

### 10.2 Release
**Duration:** 2 days

| Task | Description | Dependencies |
|------|-------------|--------------|
| DEPLOY-004 | Create installer (NSIS) | DEPLOY-003 |
| DEPLOY-005 | Test installer | DEPLOY-004 |
| DEPLOY-006 | Release v1.0.0 | DEPLOY-005 |

---

## Implementation Timeline Summary

| Phase | Duration | Week | Key Deliverables |
|-------|----------|------|------------------|
| 1. Foundation | 2 weeks | 1-2 | Project setup, Database |
| 2. Core Backend | 2 weeks | 3-4 | Auth, DB layer |
| 3. Frontend Foundation | 2 weeks | 5-6 | HTML/HTMX base |
| 4. Admin Module | 4 weeks | 7-10 | Full admin UI |
| 5. Kasir Module | 3 weeks | 11-13 | POS interface |
| 6. Reports | 2 weeks | 14-15 | Reports & export |
| 7. Printer Support | 2 weeks | 16-17 | All printer types |
| 8. Offline Support | 2 weeks | 18-19 | Sync engine |
| 9. Testing | 2 weeks | 20-21 | QA & fixes |
| 10. Deployment | 1 week | 22 | Release |

**Total Duration:** ~22 weeks (5-6 months)

---

## Development Milestones

| Milestone | Target | Description |
|-----------|--------|-------------|
| M1 | Week 2 | Empty shell builds, DB ready |
| M2 | Week 4 | Authentication working |
| M3 | Week 6 | Frontend foundation complete |
| M4 | Week 10 | Admin module complete |
| M5 | Week 13 | Kasir module complete |
| M6 | Week 15 | Reports module complete |
| M7 | Week 17 | Printer support complete |
| M8 | Week 19 | Offline sync complete |
| M9 | Week 21 | All testing passed |
| M10 | Week 22 | v1.0.0 Release |

---

## Priority Order for MVP

If releasing in phases, prioritize:

### MVP Release (Week 13)
1. Authentication
2. Products (basic CRUD)
3. Categories
4. Kasir POS (cash only)
5. Basic reports

### Phase 2 (Week 17)
1. All payment methods
2. Printer support
3. Branch management

### Phase 3 (Week 19)
1. Offline mode
2. Advanced reports

---

## Technical Dependencies Map

```
Phase 1 (Foundation)
├── 1.1 Environment
│   └── All tasks sequential
└── 1.2 Database
    └── 1.3 SQLite (parallel)

Phase 2 (Backend)
├── 2.1 DB Layer → 2.2 Auth → 2.3 Middleware
└── 1.3 SQLite (from Phase 1)

Phase 3 (Frontend)
├── 3.1 HTML/HTMX
├── 3.2 Navigation
└── 3.3 Components

Phase 4 (Admin) → Requires Phase 2 + 3
Phase 5 (Kasir) → Requires Phase 2 + 3 + 4
Phase 6 (Reports) → Requires Phase 5
Phase 7 (Printer) → Requires Phase 5
Phase 8 (Offline) → Requires Phase 2 + 3
Phase 9-10 → All previous phases
```

## [COMPLETED] Phase 5.5: Backend Stability & SQLx 0.8 Migration
- [x] Resolve `sqlx::query!` and `sqlx::query_scalar!` compilation errors by migrating to standard query functions.
- [x] Add explicit `sqlx::Postgres` type arguments to all database queries.
- [x] Fix duplicate `impl` blocks and missing `From` implementations in `db/mod.rs`.
- [x] Standardize variable naming (`row` vs `rows`) for single vs multiple record fetches.

## [IN PROGRESS] Phase 6: UI Refinement & Polish
- [x] Standardize all forms (Login, Add/Edit) to **Light Theme** with high-contrast text.
- [x] Fix UI contrast issues in Period Selector and Management Tables.
- [ ] Implement robust error boundary handling for frontend HTMX responses.
- [ ] Add loading states/spinners for long-running database operations.

---

**Document Version:** 1.2  
**Last Updated:** March 20, 2026
