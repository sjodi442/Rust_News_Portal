# 🗞️ The Rust Chronicle — Modern Broadsheet Web Portal & CMS

[![Rust](https://img.shields.io/badge/Rust-1.99%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Loco.rs](https://img.shields.io/badge/Loco.rs-1.2.0-blue)](https://loco.rs/)
[![Tests](https://img.shields.io/badge/Tests-70%20passed%20(100%25)-brightgreen)](https://github.com/sjodi442/Rust_News_Portal)
[![SQLite](https://img.shields.io/badge/SQLite-WAL%20Mode-003B57?logo=sqlite&logoColor=white)](https://www.sqlite.org/)
[![HTMX](https://img.shields.io/badge/HTMX-1.9.12-3366CC)](https://htmx.org/)
[![Tailwind CSS](https://img.shields.io/badge/TailwindCSS-v3%2Fv4-38B2AC?logo=tailwindcss&logoColor=white)](https://tailwindcss.com/)
[![License](https://img.shields.io/badge/License-MIT-lightgrey)](LICENSE)

**The Rust Chronicle** is a high-performance editorial web portal and broadsheet news CMS built entirely with Rust. It marries the timeless elegance of classic broadsheet newspapers (*monochrome ink & newsprint*) with sub-millisecond Server-Side Rendering (SSR) and seamless modern interactivity.

Repository: **[https://github.com/sjodi442/Rust_News_Portal](https://github.com/sjodi442/Rust_News_Portal)**

---

## ✨ Fitur Utama (Key Features)

### 1. 📰 Authentic Broadsheet Aesthetic
- **Monochrome Ink & Newsprint:** Tipografi tajam terinspirasi koran abad industri (*Playfair Display*, *Inter*, dan *Space Grotesk*) dengan aksen palet natural parchment (`#f9f9f7`).
- **Classic Frontpage Layout:** Grid editorial berkolom dengan Lead Headline besar, Sidebar Analysis, Columnist Notes, dan Op-Ed columns.
- **Telegraph Wire Ticker:** Continuous horizontal ticker animasi berjalan (*smooth infinite marquee*) dengan fitur auto-pause on hover.
- **Reader Tools:** Font-size scaler, night-shift parchment toggle, progress bar baca, dan print/export layout khusus koran cetak.

### 2. ⚡ Realtime Market Ledger & Live Feeds
- **Zero-Config Realtime Commodities:** Integrasi otomatis dengan public exchange feed (Gold Spot, Silver Ingot, WTI Crude Oil, High Grade Copper, Henry Hub Natural Gas).
- **Background Synchronization Worker:** Background interval worker memperbarui data bursa setiap 5 menit secara non-blocking.
- **HTMX Dynamic Polling:** Bagian bursa di halaman depan melakukan auto-refresh berkala (60s) tanpa perlu reload halaman.
- **Admin Instant Sync:** Tombol sinkronisasi langsung di panel admin untuk pembaruan manual kapan saja.

### 3. 📡 Sindikasi RSS, SEO & Social Cards
- **RSS 2.0 Syndication (`/rss.xml` & `/feed.xml`):** Dukungan penuh untuk RSS reader (Feedly, Miniflux, NetNewsWire) dengan 30 artikel terbitan terbaru, RFC 2822 timestamps, dan GUID permalinks.
- **Dynamic XML Sitemap (`/sitemap.xml`):** Peta situs XML standar yang mengindeks beranda, arsip pencarian, kategori rubrik, dan seluruh artikel terbitan berserta atribut `<lastmod>` dan `<changefreq>`.
- **OpenGraph & Twitter Cards:** Tag meta `og:title`, `og:description`, `og:image`, `og:type`, `og:url`, serta `twitter:card` (summary_large_image) terpasang di seluruh halaman dan artikel reader.
- **Feed Auto-Discovery:** Header `<link rel="alternate" type="application/rss+xml">` dan `<link rel="sitemap">` untuk deteksi otomatis oleh crawler dan reader.

### 4. 🛡️ Editorial CMS & Modern Security
- **AdminGuard Axum Extractor:** Proteksi 28 rute editorial berbasis JWT token dengan dukungan HTTP-Only Cookie dan fallback Bearer Header.
- **Editorial Desk Control:** Kelola artikel, kurasi lead slot halaman utama, moderasi opini publik (*reader dispatches*), kawat telegrafik (*telegraph wires*), dan daftar pelanggan newsletter.
- **Argon2id Hashing:** Standar enkripsi kata sandi resmi Loco/Argon2id.
- **Zero-External Dependencies (Offline-Ready):** Seluruh file JavaScript (HTMX v1.9.12), stylesheet custom, dan font telah di-cache secara mandiri di `assets/static/`.

### 4. 🚀 Arsitektur & Performa Rust
- **Loco.rs Fullstack Framework:** Axum + SeaORM + Tera template engine.
- **SQLite WAL Mode:** Concurrency tinggi dan latensi rendah dengan performa query teroptimasi (6 indeks komposit database).
- **Resource Footprint Minimal:** Konsumsi RAM dev/idle hanya ~25–40MB, cocok untuk server spesifikasi hemat.

---

## 📁 Struktur Direktori Repositori

```text
Rust_News_Portal/
├── README.md                      # Dokumentasi utama repositori GitHub
├── design/                        # Mockup HTML awal (Broadsheet Grid & Article Reader)
│   ├── homepage.html
│   └── article_reader.html
├── docs/                          # Dokumentasi spesifikasi & rekayasa perangkat lunak
│   ├── PRD.md                     # Product Requirements Document
│   ├── ARCHITECTURE.md            # Arsitektur sistem Axum + SeaORM + Loco
│   ├── DATABASE_SCHEMA.md         # Skema SQLite3 DDL & 6 composite performance indexes
│   ├── UI_UX.md                   # Panduan desain, tipografi, dan palet monokrom
│   └── IMPLEMENTATION_PLAN.md     # Rencana implementasi teknis (Phase 1 s/d Phase 6)
└── rust_chronicle/                # Aplikasi utama (Loco.rs CMS Engine)
    ├── assets/                    # Static files (HTMX, CSS) & Tera SSR Views
    │   ├── static/                # Offline static assets (/static/*)
    │   └── views/                 # Template frontpage, article reader, admin CMS
    ├── config/                    # Environment settings (development.yaml, production.yaml)
    ├── migration/                 # SeaORM database migrations
    ├── src/                       # Controllers, models, initializers, admin guard
    └── tests/                     # 67 request & model test suite (100% pass)
```

---

## 🛠️ Prasyarat Sistem (Prerequisites)

- **Rust:** Versi 1.80+ (direkomendasikan versi 1.85+)
- **Cargo:** Paket manager bawaan Rust
- **SQLite3:** Versi 3.35+ dengan dukungan JSON1

---

## 🚀 Panduan Memulai Cepat (Quick Start)

### 1. Clone Repositori
```bash
git clone https://github.com/sjodi442/Rust_News_Portal.git
cd Rust_News_Portal/rust_chronicle
```

### 2. Jalankan Migrasi Database
```bash
cargo run -- db migrate
```

### 3. Isi Data Awal (Newsroom Seed Fixtures)
```bash
cargo run -- db seed
```

### 4. Jalankan Seluruh Test Suite (67 Tests)
```bash
cargo test --all
```

### 5. Jalankan Web Server
```bash
cargo run -- start
```
Web server akan aktif melayani di: **`http://localhost:5150`**

---

## 🔐 Kredensial Administrator Default

Setelah menjalankan `cargo run -- db seed`, akun redaksi administrator berikut siap digunakan:

| Kredensial | Nilai Default |
|---|---|
| **URL Login** | `http://localhost:5150/admin/login` |
| **Email** | `admin@example.com` |
| **Password** | `password` |
| **Role** | Editor in Chief / Bureau Administrator |

> *Catatan:* Untuk membuat akun administrator baru melalui CLI:
> ```bash
> cargo run task user:create email:editor@example.com name:"Senior Editor" password:"YourSecurePassword"
> ```

---

## 🧪 Pengujian & Kualitas Kode (Test Suite)

Semua fungsionalitas diuji secara ketat mencakup 67 skenario uji otomatis:
- **Model Tests:** Relasi entitas SeaORM, hashing Argon2id, token verifikasi, validasi formulir.
- **Admin Request Tests:** Flow autentikasi session cookie, mitigasi unauthenticated access, kurasi frontpage lead slot, sinkronisasi market realtime, moderasi telegraf & surat pembaca.
- **Public Reader Tests:** Dynamic article slug lookup, search query, telegraph ticker fragment, newsletter subscription, dan live market ticker.

Untuk menjalankan pengujian:
```bash
cargo test --all
```

---

## 📜 Lisensi (License)

Proyek ini dilisensikan di bawah lisensi **MIT** — silakan gunakan, pelajari, dan kembangkan secara bebas.
