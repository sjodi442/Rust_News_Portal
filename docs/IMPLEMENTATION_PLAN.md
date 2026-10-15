# Implementation Plan & Engineering Roadmap
## Project: Rust Web App — The Rust Chronicle (Monochrome News Portal)

---

### Phase 1: Toolchain & Project Initialization (SELESAI - COMPLETED)
- [x] Verifikasi dan pasang Rust toolchain stabil (`rustup` / `cargo 1.99.0` aktif).
- [x] Inisialisasi workspace proyek Loco di `Projects/Rust/rust_chronicle`:
  - Framework: Loco 1.2 (Axum + SeaORM + Tera + Async Background Worker).
  - Database: SQLite3 WAL mode (`rust_chronicle_development.sqlite`).
  - Struktur direktori modular: `src/controllers`, `src/models`, `src/views`, `assets/views/`, `migration/`.
- [x] Integrasi template desain Google Stitch ke sistem view Loco:
  - Homepage broadsheet di `assets/views/home/index.html`.
  - Editorial article reader di `assets/views/articles/reader.html`.
- [x] Routing dasar di `src/controllers/page.rs`:
  - `GET /` -> Halaman Depan Broadsheet.
  - `GET /article` -> Halaman Editorial Article Reader.
- [x] Test suite passing: 29 tests, 100% pass rate.

### Phase 2: Database Layer & Migrations (SQLite3 WAL) (SELESAI - COMPLETED)
- [x] Buat file migrasi database sesuai spesifikasi `DATABASE_SCHEMA.md`:
  - Tabel: `users`, `categories`, `articles`, `wire_dispatches`, `market_ledgers`, `reader_dispatches`, `newsletter_subscribers`.
  - Migrator SeaORM terdaftar di `migration/src/lib.rs`.
- [x] Model SeaORM entity & domain:
  - Entities di `src/models/_entities/`: `categories`, `articles`, `wire_dispatches`, `market_ledgers`, `reader_dispatches`, `newsletter_subscribers`.
  - Domain models di `src/models/` dengan relasi SeaORM dan active model hooks.
- [x] Konfigurasi SQLite PRAGMA & WAL mode di `config/development.yaml` dan skema database.
- [x] Seed data awal (*broadsheet newsroom fixtures*):
  - Fixtures di `src/fixtures/`: `categories.yaml`, `articles.yaml`, `wire_dispatches.yaml`, `market_ledgers.yaml`, `reader_dispatches.yaml`, `newsletter_subscribers.yaml`.
  - Terintegrasi pada hook `seed` dan `truncate` di `src/app.rs`.
- [x] Verifikasi migrasi & seed:
  - `cargo run -- db migrate` sukses.
  - `cargo run -- db seed` sukses (5 categories, 5 articles, 5 wire dispatches, 5 market ledgers, 2 reader dispatches, 2 subscribers).
- [x] Test suite passing: 35 tests, 100% pass rate (`cargo test --all`).

### Phase 3: Server Core & Template Engine (SSR) (SELESAI - COMPLETED)
- [x] Implementasi routing Axum / Loco:
  - `GET /` -> Halaman Depan Broadsheet (`assets/views/home/index.html`).
  - `GET /articles/:slug` & `GET /article` -> Halaman Reader Artikel (`assets/views/articles/reader.html`).
  - `GET /category/:slug` -> Filter berita per kategori (`assets/views/categories/show.html`).
  - `GET /search?q=` -> Pencarian arsip instan (`assets/views/articles/search.html`).
- [x] Implementasi template modular Tera:
  - Layout induk `broadsheet_base.html` (Folio, Masthead, Ticker, Navigation, Footer).
  - Partials: `_lead_story.html`, `_bulletins.html`, `_market_ledger.html`, `_reader_dispatches.html`.
- [x] Rendering HTML server-side berkecepatan tinggi dengan integrasi CSS monokrom.
- [x] API endpoints untuk ticker & interaksi pembaca:
  - `GET /api/ticker/wires` -> Feed kawat telegraf aktif (JSON).
  - `GET /api/ticker/markets` -> Feed spot ledger komoditas (JSON).
  - `POST /api/dispatches` -> Kirim surat / tanggapan pembaca (JSON).
  - `POST /api/subscribe` -> Registrasi buletin harian pagi (JSON).

### Phase 4: Interaktivitas Modern via HTMX (SELESAI - COMPLETED)
- [x] Integrasi HTMX untuk interaksi tanpa reload halaman:
  - Polling live ticker telegrafik berkala via `hx-get="/api/wire/latest"` `hx-trigger="every 30s"`.
  - Live instant search arsip tanpa flicker via `hx-get="/api/search/fragment"`.
  - Submit tanggapan pembaca (*Reader Dispatches*) secara inline dengan partial swap `_dispatch_card.html`.
  - Form pendaftaran newsletter surat kabar harian secara inline (`POST /api/subscribe/submit`).
- [x] Interaksi ergonomis di sisi klien (Vanilla JS minimal):
  - Scroll progress meter di reader mode (`#reading-progress`).
  - Pengatur ukuran teks membaca (`A- / A+ / Reset`).
  - Shortcut cetak koran fisik (`Print Folio` via `window.print()`).
- [x] Test suite passing: 48 tests, 100% pass rate (`cargo test --all`).

### Phase 5: Admin Editorial CMS & Layout Management Desk (SELESAI - COMPLETED)
- [x] Master Admin Layout & Newsroom Desk Styling (`assets/views/admin/admin_base.html`).
- [x] Newsroom Overview & KPI Metrics Dashboard (`assets/views/admin/index.html`):
  - Metrik sirkulasi: total berita, lead stories, draft, kategori/desk, telegraph wires aktif, surat pembaca, pembaca terdaftar.
- [x] Manajemen Artikel & Berita Lengkap (`assets/views/admin/articles/`):
  - Form tambah/edit artikel lengkap: headline, kicker, dispatch ref code, subhead, category picker, custom/auto URL slug, slot frontpage (lead/standard/bulletin/oped), status (published/draft/archived), reading time, opening dropcap lead, full Markdown body, cover image with credit & caption.
  - CRUD operations: create, update, delete, set-lead toggle.
- [x] Pengkategorian & Departemen Editorial Desk (`assets/views/admin/categories/`):
  - Tabel kategori dengan counter jumlah berita terbit.
  - Form tambah cepat dan edit detail nama, slug, dan prioritas urutan tampil (`display_order`).
- [x] Pengaturan Penataan Halaman Depan / Front Page Desk (`assets/views/admin/frontpage/`):
  - Kurasi visual slot berita utama (*Front Page Lead Story*).
  - Slotting kolom kiri (*Bulletins*) & kolom kanan (*Op-Ed / Verdicts*).
  - Banner breaking news / bulletin wire flash toggle.
- [x] Manajemen Kawat Telegraf (*Telegraph Wires*) (`assets/views/admin/wires/`):
  - Form transmisi kawat baru (urgency level: routine/priority/flash, source station, text).
  - Toggle status tayang ticker dan delete kawat kedaluwarsa.
- [x] Moderasi Surat Pembaca (*Reader Dispatches*) (`assets/views/admin/dispatches/`):
  - Filter dan tinjau kiriman opini pembaca dari form publik.
  - Toggle editorial pick (tampil di halaman depan) dan moderasi/hapus spam.
- [x] Editor Pasar & Komoditas (*Market Ledgers*) (`assets/views/admin/markets/`):
  - Editor harga spot komoditas (Coal, Iron, Grain, Freight, Timber) dan perubahan persentase harian.
- [x] Sirkulasi Pembaca (*Newsletter Subscribers*) (`assets/views/admin/subscribers/`):
  - Daftar pelanggan broadsheet morning edition dan toggle status aktif/unsub.
- [x] Suite Pengujian Admin Integration Test (`tests/requests/admin.rs`):
  - 17/17 integration tests pass rate 100%.

### Phase 6: Production Hardening, Optimasi & Security (SELESAI - COMPLETED)
- [x] Keamanan Autentikasi Admin & Session Guard:
  - Custom Axum extractor `AdminGuard` di `src/controllers/admin.rs`.
  - Proteksi penuh terhadap 27 rute administratif via verifikasi Cookie JWT HTTP-Only (`token`) dan fallback header `Authorization: Bearer`.
  - Penolakan akses otomatis dialihkan (redirect 303) ke form login `/admin/login`.
  - Halaman login editorial khusus broadsheet (`assets/views/admin/login.html`) dan endpoint login (`POST /admin/login`) serta logout (`GET /admin/logout`).
  - Akun administrator default aktif: `admin@example.com` / `password` dengan enkripsi Argon2id bawaan Loco.
- [x] Optimasi Query & Indeks Database (SeaORM Migration):
  - Migrasi `m20261013_040000_optimize_indexes.rs` terpasang di database SQLite.
  - 6 indeks komposit performa tinggi: `idx_articles_status_pub`, `idx_articles_cat_pub`, `idx_articles_type`, `idx_wires_active_urgent`, `idx_dispatches_editors_pick`, `idx_markets_category`.
- [x] Self-Hosting Static Assets & Resiliensi Offline:
  - Memindahkan dependensi CDN eksternal ke penyimpanan lokal mandiri di `assets/static/`.
  - Vendor file HTMX v1.9.12 offline di `assets/static/js/htmx.min.js`.
  - Master stylesheet cetak broadsheet di `assets/static/css/broadsheet.css`.
  - Halaman error 404 bertema koran broadsheet vintage di `assets/static/404.html`.
- [x] Hardening Konfigurasi Production (`config/production.yaml`):
  - Static file server middleware aktif melayani direktori `assets/static` pada prefix `/static`.
  - Fallback error handler 404 aktif mengarah ke `404.html`.
  - Konfigurasi autentikasi JWT cookie `token` aktif.
- [x] Optimasi Profile Release (`Cargo.toml`):
  - `[profile.release]` diatur dengan `opt-level = 3`, `codegen-units = 16`, `strip = true`, `panic = "abort"`.
- [x] Verifikasi Kualitas & Test Suite Menyeluruh:
  - Total 65 integration & unit tests lolos 100% tanpa kegagalan (`cargo test --all` -> 65 passed, 0 failed).

### Phase 7: Realtime Market, Continuous Wire & Syndication (SELESAI - COMPLETED)
- [x] Live Market Commodities Synchronization:
  - Integrasi publik Yahoo Finance chart API untuk 5 komoditas bursa utama tanpa kunci/registrasi.
  - Background worker periodic `MarketSyncInitializer` berjalan otomatis setiap 5 menit via `tokio::spawn`.
  - Endpoint manual sync admin `POST /admin/markets/sync` dan tombol sync di dashboard editorial.
  - Live HTMX polling partial `GET /partials/market_ledger` dengan refresh tiap 60 detik.
- [x] Telegraph Wire Marquee Ticker:
  - Teks kawat telegraf berjalan continuous horizontal (*smooth infinite marquee*) via CSS native murni di `assets/static/css/broadsheet.css`.
  - Fitur auto-pause saat kursor di-hover untuk kenyamanan membaca.
- [x] RSS 2.0 Syndication & Dynamic Sitemap:
  - Endpoint publik `/rss.xml` dan `/feed.xml` menyajikan feed XML standar untuk 30 artikel terbitan terbaru lengkap dengan GUID permalink dan RFC 2822 pubDate.
  - Endpoint publik `/sitemap.xml` menyajikan peta situs XML komprehensif (beranda, arsip pencarian, kategori, dan seluruh artikel dengan lastmod W3C).
- [x] OpenGraph & Twitter Card Social Metadata:
  - Tag meta `og:title`, `og:description`, `og:image`, `og:type`, `og:url`, `og:site_name`, dan `twitter:card` diimplementasikan secara dinamis di `broadsheet_base.html` dan `articles/reader.html`.
  - Tag auto-discovery RSS dan Sitemap pada `<head>`.
- [x] Test Suite Expansion:
  - Penambahan pengujian unit dan integrasi untuk RSS feed, XML sitemap, dan OpenGraph meta tags di `tests/requests/page.rs`.
  - Total 70 integration & unit tests lolos 100% tanpa kegagalan (`cargo test --all` -> 70 passed, 0 failed).

### Phase 8: Realtime Masthead Folio (Date, Meteorology, Commodities) (SELESAI - COMPLETED)
- [x] Realtime Localized Date:
  - Format server UTC fallback dan script client vanilla JavaScript yang memformat tanggal pengguna secara realtime via `toLocaleDateString` dalam format formal broadsheet (`Wednesday, October 14, 2026`).
  - Auto-reevaluasi tanggal setelah swapping HTMX partial.
- [x] Global Realtime Meteorology (Open-Meteo API):
  - Integrasi API publik gratis Open-Meteo (`api.open-meteo.com`) tanpa API key untuk 3 kota keuangan dunia: London, New York, dan Tokyo.
  - Modul `src/models/meteorology.rs` dengan parser WMO weather codes dan caching in-memory thread-safe (`RwLock<Option<(Instant, String)>>`) ber-TTL 10 menit untuk efisiensi beban.
  - Background periodic sync di `MarketSyncInitializer` yang menyegarkan cuaca setiap 10 menit.
- [x] Realtime Commodities Masthead Summary:
  - Fungsi agregator `get_commodities_summary` di `src/models/market_ledgers.rs` yang merangkum 3 pergerakan bursa teratas (Gold, Crude, Gas) langsung dari ledger SeaORM.
- [x] HTMX Dynamic Folio Strip Polling:
  - Partial view independen `assets/views/partials/_folio_strip.html` dengan endpoint `GET /partials/folio`.
  - Polling otomatis tiap 60 detik (`hx-get="/partials/folio" hx-trigger="every 60s" hx-swap="outerHTML"`) tanpa reload peramban.
- [x] Test Suite & Verification:
  - Penambahan integration test `can_get_folio_partial_with_realtime_metrics` di `tests/requests/page.rs`.
  - Total 71 integration & unit tests lolos 100% tanpa kegagalan (`cargo test --all` -> 71 passed, 0 failed).

### Phase 9: SQLite FTS5 Full-Text Lexical Search (SELESAI - COMPLETED)
- [x] Virtual Table `articles_fts` (Migration `m20261014_050000_articles_fts`):
  - Pembuatan tabel virtual SQLite FTS5 terintegrasi dengan tabel `articles` (`title`, `subhead`, `lead_paragraph`, `body_markdown`, `content='articles'`, `content_rowid='id'`).
  - Injeksi trio trigger database otomatis (`articles_ai`, `articles_ad`, `articles_au`) untuk sinkronisasi seketika saat create, update, dan delete.
- [x] Query Method & Probabilistic Relevance Ranking (`src/models/articles.rs`):
  - Metode `search_fts` dengan query sanitization, token splitting, dan prefix wildcard match (`*`).
  - Pemeringkatan relevansi dengan algoritma BM25 bawaan SQLite (`bm25(articles_fts) ASC`).
  - Resilient fallback ke SQL `LIKE` multibagian bila terjadi anomali query FTS.
- [x] Pengkabelan Controller:
  - Integrasi `search_fts` pada endpoint publik arsip `/search?q=...` di `src/controllers/page.rs`.
  - Integrasi `search_fts` pada endpoint live HTMX instant search `/api/search/fragment?q=...` di `src/controllers/api.rs`.
- [x] Test Suite Expansion & Verifikasi:
  - Unit test model `can_search_articles_via_fts5` di `tests/models/articles.rs` untuk memvalidasi exact match, prefix search, dan empty query guard.
  - Total 72 integration & unit tests lolos 100% tanpa kegagalan (`cargo test --all` -> 72 passed, 0 failed).

### Phase 10: Mobile UI/UX Optimization & Clean Masthead (SELESAI - COMPLETED)
- [x] Telegraph Wire Marquee Slowdown:
  - Durasi animasi diperlambat dari 35 detik ke 85 detik di `assets/static/css/broadsheet.css` agar nyaman dibaca.
- [x] Mobile Layout & Content Hierarchy:
  - Penataan urutan konten mobile: Headline Berita Utama (*Lead Story*) tampil paling atas (`order-1` di mobile, `md:order-2` di desktop) sebelum tabel bursa saham dan arsip bulletin.
  - Skala responsif dropcap (`2.75rem` mobile, `3.8rem` desktop) untuk mencegah pemotongan kata berlebih di layar kecil.
  - Input pencarian responsif (`w-24 sm:w-36 lg:w-48`) agar tab rubrik tidak terdesak.
  - Reader toolbar touch-friendly dengan ukuran target sentuh nyaman (`min-w-[32px] h-7`) dan tombol print responsif.
- [x] Streamlining Header & Penghapusan Tombol Login:
  - Penghapusan tombol *Sign In* di header masthead publik baik tampilan desktop maupun mobile untuk memurnikan estetika broadsheet koran cetak. Akses administrasi redaksi tetap aman di jalur `/admin/login`.
- [x] Verification:
  - Seluruh 72 tests lolos 100% (`cargo test --all`).
