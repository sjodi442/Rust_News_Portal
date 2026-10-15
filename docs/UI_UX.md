# UI/UX & Design System Specification
## Project: Rust Web App — The Rust Chronicle (Monochrome Broadsheet)

---

### 1. Filosofi Desain: "The Digital Broadsheet"

Konsep visual **The Rust Chronicle** memadukan estetika koran fisik abad ke-20 dengan kemudahan navigasi dan ergonomi web modern. Tujuannya adalah menciptakan pengalaman membaca yang khidmat, terpercaya, dan bebas dari distraksi digital.

#### Prinsip Utama:
1. **Ink on Newsprint (Tinta di Atas Kertas):** Kontras tinggi tanpa membuat mata cepat lelah. Menghindari warna putih silau (`#ffffff`) untuk latar belakang utama, melainkan menggunakan warna kertas koran alami (*warm cold-pressed newsprint* `#f9f9f7`).
2. **Kekuatan Hirarki Tipografi:** Struktur informasi ditegaskan melalui ukuran font, bobot (*weights*), dan kontras serif/sans-serif, bukan melalui blok warna neon.
3. **Struktur Multi-Kolom Klasik:** Mengadopsi format *tri-fold broadsheet* (3 kolom: Late Bulletins / Lead Investigation / Voices & Verdicts).
4. **Elemen Khas Percetakan:** Garis pembatas tinta ganda (*double rule*), drop cap editorial, kutipan bersayap (*pull quotes*), penomoran edisi (*folio strip*), dan tanda colophon percetakan (`❖ ❖ ❖`).

---

### 2. Palet Warna (Monochrome Palette)

| Token Warna | Hex Code | Karakteristik & Penggunaan |
| :--- | :--- | :--- |
| `background` | `#f9f9f7` | Warna dasar kanvas (simulasi kertas koran *warm off-white*) |
| `primary` / `ink` | `#000000` | Tinta hitam pekat untuk masthead, headline utama, garis batas ganda |
| `on-surface` / `text-body` | `#1a1c1b` | Hitam arang pekat untuk teks paragraf berita |
| `secondary` | `#5e5e5e` | Abu-abu medium untuk byline, dek subjudul, dan teks keterangan |
| `surface-container` | `#eeeeec` | Abu-abu lembut untuk latar kotak kutipan dan ticker wire |
| `surface-container-lowest` | `#ffffff` | Putih bersih untuk kartu folios dan ledger container |
| `outline` / `border-ink` | `#747878` | Garis tipis pembatas kolom dan divider |
| `outline-variant` | `#c4c7c7` | Garis pemisah hairline antar artikel |
| `error` / `urgent-accent` | `#ba1a1a` | Merah stempel tua hanya untuk label **[URGENT]** bulletin |

---

### 3. Skala Tipografi & Font Pairing

Aplikasi memadukan 3 font Google Fonts yang saling melengkapi:

| Kategori | Font Family | Contoh Penggunaan | Ukuran / Karakteristik |
| :--- | :--- | :--- | :--- |
| **Display & Headline** | `Playfair Display` | Masthead, Judul Hero, Subhead | 72px (Masthead), 48px (Lead Headline), 28px (Section H2), Serif elegan |
| **Body & Reading** | `Inter` | Naskah artikel, caption foto | 15px - 18px, Line-height 1.7, Spacing nyaman untuk membaca panjang |
| **Folio & Byline** | `Space Grotesk` | Kicker, tanggal edisi, ticker, byline | 10px - 12px, Uppercase, Tracking +0.12em, Geometrik modern |

---

### 4. Layout & Pola Komponen Utama

#### A. Broadsheet Masthead & Folio Strip
- Terletak di paling atas dengan garis ganda pembatas atas dan bawah.
- Berisi nomor edisi cetak (*Vol. CXLII — No. 51,902*), tanggal terbit, suhu kota utama dunia, indeks komoditas, serta tombol langganan (*Subscribe*).
- Tombol otentikasi login publik ditiadakan untuk menjaga kemurnian estetika koran broadsheet klasik (portal redaksi diakses via jalur khusus `/admin/login`).

#### B. Tri-Fold Editorial Grid (Halaman Depan)
- **Kolom Kiri (3 cols - Late Bulletins):** Menampilkan kabar mendesak per jam dengan timestamp relatif (`• 14m ago`) dan ringkasan ringkas. Di bawahnya terdapat tabel *Commercial Ledger* (kurs/harga logam industri).
- **Kolom Tengah (6 cols - Lead Story):** Menampilkan berita utama hari ini dengan foto hitam-putih artistik (*grayscale filter*), drop cap besar di paragraf awal, dan kutipan bersayap di tengah naskah.
- **Kolom Kanan (3 cols - Voices & Verdicts):** Op-Ed dan esai sudut pandang dari analis independen dengan inisial avatar klasik.

#### C. Article Reader Mode (Halaman Detail)
- Dilengkapi pita pembaca atas yang menempel (*sticky reading ribbon*) dengan meteran persentase baca.
- Kontrol ukuran teks langsung di header (`A- / A+`).
- Blok data teknis (*Diagnostic Matrix Table*) bergaya laporan audit laboratorium.
- Kolom diskusi tertutup beradab (*Dispatches from Readers*) dengan tanda afiliasi institusional.

---

### 5. File Mockup & Preview Desain (Google Stitch)

Desain interaktif telah dibuat dan disimpan secara lokal di repositori proyek:

1. **Halaman Depan Broadsheet:**  
   Path file: `Projects/Rust/design/homepage.html`  
   Screen ID Stitch: `6229dc69f85048d69bdf455ffacdde71`

2. **Halaman Reader Artikel Investigasi:**  
   Path file: `Projects/Rust/design/article_reader.html`  
   Screen ID Stitch: `d719284795ff4d1b829b58245589da40`

*Catatan: Kedua file dapat langsung dibuka di browser desktop atau mobile untuk menguji interaktivitas, responsivitas grid, dan kesesuaian kontras monokrom.*
