# The Rust Chronicle 📰
**Modern Monochrome Broadsheet News Portal & Editorial Bureau**

Web portal berita bergaya koran cetak klasik (*broadsheet vintage*) yang dibangun dengan **Rust** berkinerja tinggi, mengusung arsitektur Server-Side Rendering (SSR) modular, reaktivitas modern menggunakan **HTMX**, dan database **SQLite3** dalam mode Write-Ahead Logging (WAL).

---

## 🏛️ Informasi Aplikasi & Arsitektur

- **Framework:** [Loco v1.2](https://loco.rs) (Axum, SeaORM, Tera Template Engine, Tokio Async Runtime).
- **Database:** SQLite3 WAL Mode (`rust_chronicle_development.sqlite`).
- **Desain & Tema:** *Modern Broadsheet Newspaper* — tipografi serif monumental (*Playfair Display*), sans-serif bersih (*Inter*), monospaced/kicker (*Space Grotesk*), dan palet monokrom tinta hitam (`#000000`) di atas kertas koran gading (`#f9f9f7`).
- **Interaktivitas:** HTMX 1.9.12 untuk live telegraph ticker polling, live instant search, submit opini pembaca tanpa reload, dan pendaftaran newsletter inline.

### Fitur Publik
1. **Front Page Broadsheet (`/`)**: Headline monumental, lead story dengan dropcap, feed kawat telegrafik berjalan (infinite marquee), kolom bulletin kiri, editorial op-ed kanan, ledger komoditas spot, strip folio realtime (cuaca & tanggal), dan surat pembaca.
2. **Realtime Masthead Folio & Meteorology (`/partials/folio`)**: Tanggal dinamis client-localized, prakiraan cuaca kota-kota finansial global (London, New York, Tokyo) via Open-Meteo API publik gratis (cache TTL 10 menit), dan ringkasan bursa komoditas auto-polling per 60 detik.
3. **Editorial Reader Mode & Broadsheet Print Engine (`/articles/{slug}`)**: Format baca fokus, indikator estimasi waktu membaca, live reading progress bar, kontrol ukuran font (A- / A+ / Reset), penanda tanggal publikasi asli naskah terikat ke folio koran, dan shortcut cetak koran fisik (*Print Folio*) dengan pengecualian total kolom komentar dan formulir pembaca.
4. **Desk Taxonomy Archive (`/category/{slug}`)**: Filter arsip berita berdasarkan rubrik editorial (Dispatches, Industry, Infrastructure, Op-Ed, Folios).
5. **Instant Archive Search (`/search?q=`)**: Pencarian berita leksikal berkecepatan tinggi bertenaga **SQLite FTS5 (Full-Text Search)** dengan algoritma pemeringkatan relevansi BM25, prefix wildcard matching, dan fallback resilien.
6. **Syndicated Feeds & SEO (`/rss.xml`, `/sitemap.xml`)**: Feed XML standar RSS 2.0 untuk pembaca berita dan XML sitemap komprehensif untuk mesin pencari, disertai dynamic OpenGraph & Twitter Card social metadata.
7. **Reader Dispatches (`POST /api/dispatches`)**: Pengiriman opini & surat pembaca langsung dari halaman depan.
8. **Morning Dispatch Circulation (`POST /api/subscribe`)**: Pendaftaran buletin harian pagi pembaca.

### Fitur Admin Editorial Bureau CMS
1. **Autentikasi & Session Guard (`AdminGuard`)**:
   - Seluruh 27 rute administratif diproteksi oleh custom Axum extractor `AdminGuard`.
   - Menggunakan token JWT yang disimpan dalam Cookie HTTP-Only (`token`) dengan masa berlaku 7 hari, serta mendukung fallback header `Authorization: Bearer <token>`.
   - Permintaan tanpa token otomatis dialihkan via HTTP 303 ke `/admin/login`.
   - Endpoint login (`GET /admin/login`, `POST /admin/login`) dan logout (`GET /admin/logout`).
2. **Newsroom Overview (`/admin`)**: Dashboard KPI metrik operasional editorial (total berita, lead stories, draft, kategori aktif, kawat telegraf, surat masuk, sirkulasi pembaca).
3. **Manajemen Berita & Artikel (`/admin/articles`)**:
   - Pembuatan dan pengeditan artikel lengkap (`/admin/articles/new` & `/admin/articles/{id}/edit`).
   - Field: Headline, kicker, ref code, subhead, rubrik desk, custom slug/auto-slug, slotting tipe halaman depan, status terbit, estimasi reading time, dropcap lead paragraph, Markdown body editor, dan cover photo credit/caption.
   - Aksi: Publish, update, hapus, dan switch headline lead.
4. **Penataan Halaman Depan / Front Page Desk (`/admin/frontpage`)**:
   - Kurasi visual penataan koran: menentukan Main Lead Story, slotting kolom kiri (Bulletins), dan slotting kolom kanan (Op-Ed).
   - Pengaturan banner kawat kilat (*Breaking Wire Flash*).
5. **Taksonomi & Rubrik (`/admin/categories`)**:
   - Manajemen rubrik departemen editorial, custom slug, dan urutan prioritas tampil di navigasi (`display_order`).
6. **Feed Kawat Telegraf (`/admin/wires`)**:
   - Transmisi berita kawat berjalan (urgency: Routine, Priority, Flash) dan kontrol toggle aktif/arsip ticker.
7. **Moderasi Surat Pembaca (`/admin/dispatches`)**:
   - Verifikasi dan kurasi surat tanggapan masyarakat, toggle *Editorial Pick* untuk dimuat di halaman depan koran, serta moderasi spam.
8. **Buku Komoditas Pasar (`/admin/markets`)**:
   - Pembaruan harga spot komoditas fisik (Batu Bara, Bijih Besi, Gandum, Freight Baltic, Kayu) dan fluktuasi harian.
9. **Sirkulasi Pelanggan (`/admin/subscribers`)**:
   - Audit daftar pembaca morning edition dan toggle status langganan.

---

## 🔑 Akun Admin Default

Akun administrator bawaan telah disediakan di database seed:

- **Email:** `admin@example.com`
- **Password:** `password`
- **Role:** Editor in Chief / Bureau Administrator

> **Catatan Pengaturan Akun Admin Baru:**  
> Untuk membuat atau mereset akun administrator kustom secara aman via CLI, jalankan Loco task:
> ```bash
> cargo run task user:create email:admin@example.com name:"Editor in Chief" password:"password"
> ```

---

## 🚀 Panduan Instalasi & Penggunaan

### 1. Prasyarat Sistem
- **Rust Toolchain:** v1.80+ (Disarankan `v1.99.0` stabil).
- **SQLite3:** v3.35+ dengan dukungan JSON1.
- **Cargo & C Compiler:** Tersedia di sistem.

### 2. Migrasi & Seed Database
Inisialisasi skema tabel dan masukkan data awal editorial broadsheet:

```bash
# Pindah ke direktori proyek
cd ~/Projects/Rust/rust_chronicle

# Jalankan migrasi tabel SeaORM
cargo run -- db migrate

# Muat data awal fixtures broadsheet newsroom
cargo run -- db seed
```

### 3. Menjalankan Seluruh Test Suite
Pastikan seluruh 72 integration & unit tests lolos:

```bash
cargo test --all
```

### 4. Menjalankan Server Aplikasi
Jalankan server pengembangan Loco (default port `5150` pada `0.0.0.0`):

```bash
cargo run -- start
```
*Atau jalankan binary yang sudah dikompilasi:*
```bash
./target/debug/rust_chronicle-cli start
```

### 5. Akses Endpoint Utama

| Halaman / Fitur | URL Akses | Proteksi |
|---|---|---|
| **Halaman Depan Broadsheet** | `http://localhost:5150/` | Publik |
| **Folio Realtime Partial** | `http://localhost:5150/partials/folio` | Publik |
| **Sindikasi RSS 2.0 Feed** | `http://localhost:5150/rss.xml` | Publik |
| **XML Sitemap** | `http://localhost:5150/sitemap.xml` | Publik |
| **Editorial Reader Mode** | `http://localhost:5150/articles/{slug}` | Publik |
| **Arsip Per Rubrik** | `http://localhost:5150/category/{slug}` | Publik |
| **Pencarian Berita Instan** | `http://localhost:5150/search?q={query}` | Publik (SQLite FTS5) |
| **Admin Login Page** | `http://localhost:5150/admin/login` | Publik |
| **Admin Logout** | `http://localhost:5150/admin/logout` | Publik |
| **Admin Bureau Dashboard** | `http://localhost:5150/admin` | `AdminGuard` (Auth Required) |
| **Admin Manajemen Artikel** | `http://localhost:5150/admin/articles` | `AdminGuard` (Auth Required) |
| **Admin Penataan Front Page** | `http://localhost:5150/admin/frontpage` | `AdminGuard` (Auth Required) |
| **Admin Rubrik / Kategori** | `http://localhost:5150/admin/categories` | `AdminGuard` (Auth Required) |
| **Admin Kawat Telegraf** | `http://localhost:5150/admin/wires` | `AdminGuard` (Auth Required) |
| **Admin Moderasi Dispatches** | `http://localhost:5150/admin/dispatches` | `AdminGuard` (Auth Required) |
| **Admin Pasar Komoditas** | `http://localhost:5150/admin/markets` | `AdminGuard` (Auth Required) |
| **Admin Sirkulasi Pembaca** | `http://localhost:5150/admin/subscribers` | `AdminGuard` (Auth Required) |
| **Static Assets (HTMX, CSS, 404)** | `http://localhost:5150/static/*` | Publik (Cached) |
| **Health Check Server** | `http://localhost:5150/_health` | Publik |

---

## 🛠️ Status Kesiapan Produksi (Production Readiness)

Berikut status implementasi hardening yang telah diselesaikan pada lapisan aplikasi serta rekomendasi infrastruktur pendukung:

### ✅ Selesai Diimplementasikan di Tingkat Aplikasi:
1. **Authentication & Session Middleware Guard (`AdminGuard`):**
   - Rute `/admin/*` terlindungi secara ketat oleh Axum extractor dengan validasi Cookie JWT HTTP-Only (`token`). Sesi yang belum terautentikasi otomatis dialihkan ke `/admin/login`.
2. **Offline-Ready & Self-Hosted Static Assets:**
   - Script HTMX v1.9.12 (`htmx.min.js`), stylesheet broadsheet (`broadsheet.css`), dan halaman fallback custom 404 (`404.html`) disimpan mandiri di `assets/static/` dan dilayani langsung oleh server via routing `/static`.
3. **Database Performance Indexing (SeaORM):**
   - 6 indeks komposit telah ditambahkan pada tabel `articles`, `wire_dispatches`, `reader_dispatches`, dan `market_ledgers` untuk menjamin kecepatan query selektif skala besar.
4. **Pengaturan Profile Release (`Cargo.toml`):**
   - Profil release dikonfigurasi dengan `opt-level = 3`, `codegen-units = 16`, `strip = true`, dan `panic = "abort"` demi efisiensi memori RAM sistem.
5. **Cakupan Pengujian Lengkap:**
   - 72/72 unit & request tests lulus 100% (`cargo test --all`).

---

### 🌐 Rekomendasi Deployment Infrastruktur

#### 1. Reverse Proxy & TLS Termination (Caddy / Nginx)
Gunakan Caddy untuk kemudahan otomatisasi sertifikat SSL (HTTPS) dan kompresi:

```caddyfile
chronicle.example.com {
    encode gzip zstd

    # Reverse proxy ke aplikasi Loco
    reverse_proxy 127.0.0.1:5150 {
        header_up X-Forwarded-Proto {scheme}
        header_up X-Real-IP {remote_host}
    }
}
```

#### 2. Systemd Service (`/etc/systemd/system/rust_chronicle.service`)
```ini
[Unit]
Description=The Rust Chronicle Newsroom Server
After=network.target

[Service]
Type=simple
User=jodi442
WorkingDirectory=/home/jodi442/Projects/Rust/rust_chronicle
Environment=LOCO_ENV=production
Environment=PORT=5150
ExecStart=/home/jodi442/Projects/Rust/rust_chronicle/target/release/rust_chronicle-cli start
Restart=always
RestartSec=5
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

#### 3. Backup Otomatis SQLite WAL
Jalankan cron job harian untuk backup aman tanpa menghentikan database yang sedang berjalan:
```bash
sqlite3 /home/jodi442/Projects/Rust/rust_chronicle/rust_chronicle_production.sqlite "VACUUM INTO '/home/jodi442/backups/chronicle_backup_$(date +\%Y\%m\%d).sqlite';"
```
