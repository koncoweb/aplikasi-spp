# Changelog

All notable changes to SwiftPOS are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased] - 2026-03-20

### Fixed — Backend Stability
- Resolved all remaining Rust compilation errors related to `sqlx` version 0.8.
- **Query Refactoring**: Migrated from `sqlx::query!` and `sqlx::query_scalar!` macros to standard functions (`sqlx::query`, `sqlx::query_as`, `sqlx::query_scalar`) for improved build stability and type safety.
- **Explicit Typing**: Added explicit `sqlx::Postgres` type arguments to all database queries to resolve generic type inference issues.
- **Data Integrity**: Fixed variable naming mismatches (`row` vs `rows`) and removed redundant `unwrap_or` calls on guaranteed non-null scalar results (e.g., `COUNT(*)`).

### Changed — UI/UX & Formatting
- **Form Standardization**: Refactored all forms (login, product, branch, user) to a consistent **Light Theme** with dark text and light field backgrounds to enhance readability.
- **Contrast Improvements**: Fixed low-contrast issues in the Dashboard Period Selector and management tables.
- **Real-time Charts**: Replaced dummy data in dashboard charts with real transaction metrics.
- **Session Persistence**: Improved `verify_session` logic to ensure robust data loading (user & tenant) on application startup.

## [Unreleased / Phase 6] - 2026-03-19

### Added — Tenant Registration

- **Multi-step Registration Modal** (`#register-modal`) — pemilik bisnis bisa daftar mandiri langsung dari halaman login:
  - **Step 1:** Input akun owner (nama lengkap, email, password + konfirmasi)
  - **Step 2:** Input info bisnis (nama toko, no. telepon, alamat — telepon & alamat opsional)
  - **Step 3:** Konfirmasi sukses + auto-login ke dashboard
- Live password-match indicator & toggle visibility pada field password
- Progress bar & step indicator (Langkah 1 dari 2 / Pendaftaran selesai)
- Link navigasi dua-arah: Login ↔ Register

### Changed — Backend Auth

- `RegisterTenantRequest` ditambah field `phone: Option<String>` dan `address: Option<String>`
- `register_tenant` command: auto-generate slug dari nama bisnis via `slugify()`, handle slug collision dengan UUID suffix
- `Tenant::create()` di `db/mod.rs` diperluas untuk INSERT phone + address
- Fungsi `slugify()` ditambahkan di `auth.rs` (konversi nama bisnis ke slug DB-safe)

### Changed — Frontend

- Wiring JS auth sepenuhnya ditulis ulang: login → `invoke('login')`, register → `invoke('register_tenant')`, logout → `invoke('logout')` (tidak ada lagi ketergantungan ke `window.SwiftPOS.Auth`)
- Sidebar footer: hapus teks "Secured with Neon Auth" → ganti ke "🔐 Login aman dengan enkripsi JWT + Argon2"

### Removed

- Stub modal registrasi lama yang tidak berfungsi
- Referensi ke `window.SwiftPOS.Auth.signUp`, `closeModalBtn`, `register-form` ID, `reg-error-text`, `reg-success-text` yang sudah tidak relevan

---

## [Unreleased] - 2026-03-19

### Added — Kasir UI Modals & Pages

- **Modal Buka Kasir** — Shift start with initial capital (modal kasir Buka Kasir) input.
- **Modal Pengeluaran Kasir** — Daily expense recording with description, amount, and history list.
- **Modal Kas Masuk Kasir** — Incoming cash inflow with summary stats at top (Total Kas Hari Ini, Total Penjualan, Kas/Penjualan %).
- **Modal Tambah Stok Kasir** — Add stock to product directly from Kasir interface.
- **Modal Bayar Piutang Kasir** — Customer receivables payment modal (customer search, outstanding balance, payment method, amount paid).
- **Modal Manajemen Piutang** — Piutang (receivables) overview modal displaying active and historical receivables with export buttons.
- **Modal Tambah Piutang Manual** — Detailed entry form for manually creating new credit/receivables records (customer info, transaction date, total, DP, jatuh tempo, notes).
- **Modal Tutup Kasir Dashboard** — Comprehensive end-of-day reconciliation modal with:
  - Tunai Masuk summary (Tunai + DP + Lunas)
  - Total Pengeluaran
  - Kas Seharusnya calculation
  - Export buttons (Google Sheets / PDF Preview)
  - Detail Transaksi section
  - Detail Pengeluaran section
  - Detail Piutang Lunas section
  - Kas Aktual input (physical cash count)
  - Selisih (Difference) display
  - Tutup Kasir action button
- **Page Laporan Tutup Kasir** (`#page-laporan-tutup-kasir`) — Full printable closing report page with transaction summaries, cash details, and historical data.

### Changed — Payment Modal

- Kredit payment method now includes:
  - **DP (Uang Muka)** input field
  - **Sisa Piutang** calculated display
- Payment methods updated from Tunai/QR/Debit/Kredit → **Tunai / Transfer / Kredit**

### Changed — Docs

- `SPEC.md` updated to **v1.1** (March 19, 2026):
  - Expanded KSR requirements (KSR-001 → KSR-014)
  - Complete Kasir UI component table in Section 6.2
  - New HTMX API endpoints for kasir, piutang, and laporan
  - Kasir acceptance criteria updated to match implemented modals
- `IMPLEMENTATION_PLAN.md` updated to **v1.1** (March 19, 2026):
  - Phase 5.2 Transaction Processing: marked Tunai, Transfer, Kredit UI as ✅ Done
  - Phase 5.5 Kasir Operations: expanded task list (KASIR-019 → KASIR-033) with completion status
  - Added backend wiring tasks (KASIR-030 to KASIR-033) for next phase

---

## [0.3.0] - 2026-03-18

### Added — Admin UI

- Admin dashboard pages implemented: Dashboard, Products, Categories, Branches, Users, Transactions, Piutang, Reports, Settings.
- Admin UI matching provided screenshots.

---

## [0.2.0] - 2026-03-17

### Added — App Foundation

- Tauri v2 project setup (`com.swiftpos.koncoweb`).
- Frontend: HTMX + TailwindCSS base template (`index.html`).
- Admin navigation sidebar.
- Kasir navigation header.
- NeonDB (`spring-mode-01735400`) connected via Neon MCP Server.

### Fixed

- Tauri `tauri.conf.json` identifier changed from `com.swiftpos.desktop` to `com.swiftpos.koncoweb`.
- Resolved Tauri build warnings.

---

## [0.1.0] - 2026-03-14

### Added

- Initial Tauri + Next.js project setup experiment (superseded by HTMX approach).
- Initial SRS (`SPEC.md`) and implementation plan (`IMPLEMENTATION_PLAN.md`) created.
