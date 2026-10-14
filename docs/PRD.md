# Product Requirements Document (PRD)
## Project: Rust Web App — The Rust Chronicle (Monochrome News Portal)

**Document Status:** Approved / Ready for Implementation  
**Version:** 1.0.0  
**Target Architecture:** Rust Modern Fullstack Web Architecture  
**Design Reference:** Google Stitch Project (`projects/3603408518424547203`)  

---

### 1. Executive Summary & Product Vision
**The Rust Chronicle** adalah platform portal berita web modern berperforma tinggi yang dibangun di atas ekosistem Rust. Konsep visual dan ergonomi membaca mengadopsi estetika koran cetak klasik (*broadsheet vintage monochrome*) yang disempurnakan dengan interaktivitas web modern:
- **Tampilan Broadsheet Monokrom:** Mengutamakan tipografi tajam (Serif Display & Clean Sans-Serif), tata letak multi-kolom koran, drop caps, pull quotes, dan garis batas tinta (*ink rules*).
- **Zero-Distraction Reading:** Menghilangkan iklan pop-up dan animasi yang tidak relevan. Fokus 100% pada keterbacaan artikel, kenyamanan visual mata (kertas *warm off-white* `#f9f9f7` dengan tinta hitam pekat `#000000`), dan kecepatan render instan (< 50ms TTFB).
- **Integritas Jurnalisme Berbasis Data:** Fitur ticker telegrafik (*breaking wire*), indeks komoditas & pasar (*commercial ledger*), serta folionya edisi cetak (*Vol./No./Date*).

---

### 2. Core Personas
1. **The Daily Reader (Casual & Longform):** Mencari ringkasan berita harian secara cepat lewat masthead dan late bulletins, lalu membaca artikel mendalam tanpa gangguan interupsi visual.
2. **The Field Correspondent / Journalist:** Mengunggah naskah reportase, editorial op-ed, gambar monokromatik terverifikasi, dan data pendukung (*telemetry/matrix*).
3. **The Editor-in-Chief / Admin:** Mengatur tata letak halaman depan (*front page curation*), menyematkan *lead hero story*, mengelola kurasi wire bulletin, dan moderasi diskursus pembaca.

---

### 3. Key Feature Specifications

#### 3.1. Front Page (Halaman Depan / Broadsheet Grid)
- **Folio Bar & Weather/Ticker:** Menampilkan nomor edisi, tanggal, cuaca kota global, dan indikator pasar komoditas.
- **Classic Masthead:** Logo tipografi koran monumental ("The Rust Chronicle") dengan sub-header motto publikasi.
- **Top Navigation Bar:** Navigasi kategori utama (*World, Tech, Business, Culture, Opinion, Science*) + pencarian arsip instan.
- **Breaking Telegraphic Wire:** Pita berita bergulir (*real-time telegraph ticker*) untuk kabar kilat/breaking news.
- **Tri-Fold Broadsheet Layout:**
  - *Left Column (3 cols):* Late Bulletins (urgent updates per jam) dan Commercial Ledger (tabel indeks pasar).
  - *Center Column (6 cols):* Lead Cover Story dengan fotografi monokrom beresolusi tinggi, kicker kategori, subjudul italic, byline koresponden, drop cap, dan pull quote.
  - *Right Column (3 cols):* Voices & Verdicts (Op-Ed kolom opini terkurasi dengan avatar inisial penulis) dan kutipan arsip sejarah.
- **Section B (Applied Science & Longform):** Liputan khusus teknis/industri dilengkapi diagram schematics monokrom dan data martials index.
- **The Saturday Folios (Bento/Card Grid):** Kumpulan esai panjang 4 kolom dengan estimasi waktu baca.
- **Daily Broadsheet Home Delivery / Newsletter Box:** Formulir pendaftaran email langganan edisi cetak digital harian.

#### 3.2. Article Reader View (Halaman Artikel Panjang)
- **Reading Progress Bar:** Indikator progress baca tipis di bagian atas layar.
- **Dynamic Utility Header:** Tombol kembali ke front page, kontrol ukuran font (A- / A+), dan opsi audio dispatch.
- **Editorial Typography Hierarchy:** Judul investigasi besar, sub-headline serif naratif, byline ganda reporter & fotografer, timestamp dispatches.
- **Visual & Tabular Supplements:**
  - Foto dokumenter monokrom berkontras tinggi dengan caption bergaya arsip pers.
  - *Drop Cap* pada paragraf pembuka.
  - *Pull Quotes* berbingkai double line (*double rule*).
  - *Technical Diagnostic Matrix:* Tabel data spesifikasi teknis untuk analisis mendalam.
- **Sticky Tools Ribbon:** Shortcut bookmark, share, dan format cetak (*Print Folio* yang dioptimalkan dengan CSS `@media print`).
- **Marginalia & Sidebar:** Daftar isi laporan (*In This Report*), kotak transmisi dokumen terenkripsi (*Reader Dispatch Inquiry*), artikel arsip terkait, dan status telemetri/cuaca.
- **Dispatches from the Readers (Komentar & Diskusi):** Forum diskursus bertingkat dengan filter (Editor's Picks, Technical Notes, All Inquiries) serta validasi identitas institusional kontributor.

#### 3.3. Content Management & Editorial Engine (Backoffice)
- **Article Lifecycle:** Draft → Under Review → Scheduled → Published → Archived.
- **Frontpage Slot Manager:** Pengaturan posisi slot berita (Lead Hero, Section B Lead, Late Bulletins, Op-Ed).
- **Media Asset Pipeline:** Konversi otomatis gambar yang diunggah menjadi format WebP/AVIF dengan filter monokromatik grayscale optimal.

#### 3.4. Syndication Feeds & SEO Metadata
- **RSS 2.0 Syndication (`/rss.xml` & `/feed.xml`):** Standar RSS feed artikel terbitan publikasi untuk pembaca feed (Feedly, Miniflux, dsb.) lengkap dengan GUID permalink, RFC 2822 pubDate, dan deskripsi lead paragraph.
- **Dynamic XML Sitemap (`/sitemap.xml`):** Protokol XML Sitemap standar Google/Search Engine mencakup rute beranda, arsip pencarian, seluruh kategori dinamis, dan artikel terbitan dengan tag `<lastmod>` (W3C Datetime format).
- **OpenGraph & Twitter Card Protocols:** Injeksi dinamis tag meta `og:title`, `og:description`, `og:type`, `og:url`, `og:image`, `og:site_name`, serta `twitter:card` (summary_large_image) pada template dasar dan pembaca artikel.
- **Auto-Discovery Headers:** Deklarasi `<link rel="alternate" type="application/rss+xml">` dan `<link rel="sitemap" type="application/xml">` di header HTML.

#### 3.5. Realtime Folio, Meteorology & Financial Commodities
- **Live Masthead Strip:** Tanggal hari ini terlokalisasi secara realtime mengikuti zona waktu lokal pengguna melalui JavaScript client (`toLocaleDateString`) dengan layout broadsheet formal.
- **Global Meteorology Integration:** Menampilkan data suhu dan kondisi cuaca kota finansial global (London, New York, Tokyo) via Open-Meteo API publik gratis, didukung in-memory thread-safe cache 10 menit.
- **Commodities Ticker Summary:** Ringkasan harga dan persentase perubahan 3 komoditas bursa teratas (Gold, Crude, Gas) langsung dari ledger SeaORM.
- **HTMX Dynamic Polling (`/partials/folio`):** Auto-refresh berkala per 60 detik tanpa reload halaman penuh.

#### 3.6. Full-Text Search Engine (SQLite FTS5)
- **High-Performance Lexical Search:** Virtual table `articles_fts` berbasis modul SQLite FTS5 bawaan untuk pencarian multi-kolom (`title`, `subhead`, `lead_paragraph`, `body_markdown`).
- **Probabilistic Ranking (BM25):** Hasil pencarian diurutkan berdasarkan relevansi frekuensi kata kunci menggunakan algoritma BM25 bawaan SQLite.
- **Triggers-Driven Auto Synchronization:** Otomatisasi indeks melalui trigger database (`AFTER INSERT`, `AFTER UPDATE`, `AFTER DELETE`) tanpa memerlukan cron indexing terpisah.
- **Prefix Wildcard & Token Cleaning:** Pemrosesan string kueri untuk prefix token matching (`word*`) dan sanitasi karakter pembatas sintaks FTS5.
- **Graceful Search Fallback:** Transisi instan ke query `LIKE` SQL standar jika kueri FTS5 mengalami kendala.

---

### 4. Non-Functional Requirements (NFR)
- **Performa & Latensi:** Server-Side Rendering (SSR) penuh; First Contentful Paint (FCP) < 0.6s; Total Blocking Time (TBT) < 50ms.
- **Efisiensi Memori Server:** Konsumsi RAM runtime Rust web daemon < 35MB pada beban normal (sangat aman untuk sistem dengan RAM 3.8GB).
- **Database Resilience:** SQLite WAL mode (Write-Ahead Logging) dengan proteksi transaksi concurrency tinggi dan zero maintenance overhead.
- **Aksesibilitas (WCAG 2.1 AA):** Rasio kontras teks monokrom mencapai minimum 7:1 (teks `#1a1c1b` di atas kanvas `#f9f9f7`). Dukungan penuh pembaca layar (*screen reader*) dan navigasi keyboard.
- **Cetak Sempurna (Print CSS):** Tombol *Print Folio* menghasilkan dokumen PDF/cetak bergaya koran kertas fisik tanpa elemen navigasi dan tombol interaktif.
