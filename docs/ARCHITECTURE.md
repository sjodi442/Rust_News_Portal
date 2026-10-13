# System Architecture & Framework Selection
## Project: Rust Web App — The Rust Chronicle

---

### 1. Evaluasi & Perbandingan Framework Rust Fullstack

Berdasarkan profil sistem pengguna (RAM fisik 3.8 GB, CPU multi-core, preferensi arsitektur bersih tanpa *bloatware*, dan pengalaman Rails yang kuat), berikut evaluasi opsi framework:

| Kriteria | **Loco (loco.rs)** *(Rekomendasi Utama)* | **Axum + Askama + HTMX** *(Opsi Minimalis)* | **Leptos (SSR + WASM)** |
| :--- | :--- | :--- | :--- |
| **Filosofi** | *The Rails of Rust* (Batteries-included) | *Pragmatic Unix-philosophy* | *Modern Reactive WASM (Next.js-like)* |
| **Fondasi Inti** | Axum + SeaORM + Tokio | Axum + SQLx + Tokio | Axum/Actix + Reactive Signals + WASM |
| **Template Engine** | Tera (Jinja2/Liquid-like) + HTMX | Askama (Compile-time HTML, zero runtime overhead) | RSX (JSX-like dalam Rust) |
| **Kebutuhan RAM Kompilasi** | Sedang (~1.2 - 1.6 GB saat build) | Ringan (~900 MB - 1.2 GB) | Berat (~2.5 - 3.5 GB saat compile WASM) |
| **Konsumsi RAM Runtime** | ~25 MB - 45 MB | ~15 MB - 25 MB | ~35 MB - 60 MB |
| **Kecepatan Development** | Sangat Cepat (ada CLI scaffold, migration, model, auth) | Cepat (tulis route dan handler sendiri) | Sedang (harus sinkronisasi server/client WASM) |
| **Kesesuaian Pengguna** | **Tertinggi** (sangat intuitif bagi developer Rails) | **Tinggi** (kontrol penuh tiap baris kode) | Cukup (risiko OOM saat build di RAM 3.8 GB) |

#### Keputusan Arsitektur:
1. **Pilihan Utama: Loco (loco.rs)**  
   Memberikan developer experience setara Ruby on Rails dengan kecepatan dan keamanan memori Rust. Menangani routing, database migration (SeaORM), authentication, background jobs, dan rendering template Tera + HTMX secara *out-of-the-box*.
2. **Pilihan Alternatif / Zero-Boilerplate: Axum + Askama + HTMX**  
   Jika pengguna menginginkan binary sekecil dan seringan mungkin tanpa layer ORM berat, stack ini memberikan performa absolut dengan kompilasi super hemat memori.

---

### 2. Diagram Alur Sistem (High-Level Architecture)

```
[ Client Browser ]
        │
        ▼ (HTTP/HTTPS Requests)
┌────────────────────────────────────────────────────────┐
│ Nginx Reverse Proxy (SSL Termination, Static Assets)   │
└───────────────────────┬────────────────────────────────┘
                        │ (Local Reverse Proxy :3000)
                        ▼
┌────────────────────────────────────────────────────────┐
│ Rust Application Core (Loco / Axum Engine)             │
│                                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Middlewares: Tracing, Compression, Security-Hdrs │  │
│  └────────────────────────┬─────────────────────────┘  │
│                           ▼                            │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Router & Controllers:                            │  │
│  │  • FrontpageController (Lead, Bulletins, Ledger) │  │
│  │  • ArticleController (Reader, Dropcaps, Audio)   │  │
│  │  • DispatchWireController (Telegraph ticker)     │  │
│  │  • OpEdController (Voices & Verdicts)            │  │
│  │  • SubscriptionController (Newsletter)           │  │
│  └────────────────────────┬─────────────────────────┘  │
│                           ▼                            │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Views & Rendering Engine:                        │  │
│  │  • Tera / Askama Templates (Broadsheet layout)   │  │
│  │  • Tailwind CSS (Monochrome ink & paper theme)   │  │
│  │  • HTMX (Live search, wire ticker auto-refresh)  │  │
│  └────────────────────────┬─────────────────────────┘  │
│                           ▼                            │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Data Access Layer (SeaORM / SQLx)                │  │
│  └────────────────────────┬─────────────────────────┘  │
└───────────────────────────┼────────────────────────────┘
                            ▼
             ┌──────────────────────────────┐
             │ SQLite3 Database (WAL Mode)  │
             │  • Zero server daemon        │
             │  • Single-file storage       │
             │  • Ultra-fast read latencies │
             └──────────────────────────────┘
```

---

### 3. Komponen Utama Arsitektur

1. **Routing & Handler Layer:**
   - Memproses request HTTP masuk.
   - Endpoint SSR untuk merender dokumen HTML utuh (SEO ramah dan kecepatan render cepat).
   - Partial endpoints (`/partials/...`) yang merespons potongan HTML via HTMX untuk pembaruan komponen dinamis (misal: live wire updates, tab switching, form subscription tanpa reload).

2. **Template & Presentation Layer:**
   - Menggunakan sistem komponen modular:
     - `layouts/broadsheet.html`: Kerangka induk (Folio bar, Masthead, Navigation, Footer).
     - `components/masthead.html`: Logo monokrom koran dan tanggal edisi.
     - `components/wire_ticker.html`: Ticker berita telegrafik berjalan.
     - `components/market_ledger.html`: Tabel indeks pasar dan komoditas.
     - `articles/show.html`: Halaman reader mode artikel investigasi.

3. **Data & Storage Layer (SQLite3 WAL):**
   - Mode `PRAGMA journal_mode=WAL;` diaktifkan untuk memastikan konkurensi pembacaan paralel tak terhingga tanpa lock blocking.
   - `PRAGMA synchronous=NORMAL;` dan `PRAGMA cache_size=-64000;` (alokasi cache 64MB di RAM) untuk performa baca sub-milidetik.

4. **Security, Session Guard & Content Policy:**
   - **`AdminGuard` (Axum Extractor):** Mengamankan seluruh 27 rute administratif `/admin/*` via verifikasi Cookie JWT HTTP-Only (`token`) dan fallback header `Authorization: Bearer`. Request tanpa hak akses otomatis diredirect (HTTP 303) ke `/admin/login`.
   - **Self-Hosted Offline Assets:** Static assets (HTMX v1.9.12, broadsheet CSS, custom 404 page) dilayani mandiri dari `assets/static/` melalui endpoint `/static/*`.
   - Strict Content Security Policy (CSP) & HTTP-Only secure cookies.
   - Sanitasi konten editorial HTML via `ammonia` crate untuk mencegah serangan XSS pada naskah artikel atau komentar pembaca.
   - Rate limiting bawaan pada endpoint submit formulir dan komentar.

5. **Query Optimization & Composite Indexing:**
   - Penambahan 6 indeks komposit SeaORM pada status publikasi artikel, rubrik, urgency kawat telegraf, dan editorial pick komentar pembaca untuk menjamin kecepatan kueri baca tinggi pada volume data besar.
