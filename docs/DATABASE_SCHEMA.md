# Database Schema Specification
## Project: Rust Web App — The Rust Chronicle (SQLite3 WAL)

---

### 1. Desain Relasional Database

Database dirancang menggunakan SQLite3 dengan relasi relasional penuh, index pada query selektif, dan foreign key cascade handling.

```
┌─────────────────┐        ┌──────────────────┐        ┌─────────────────┐
│   categories    │        │      users       │        │  editions_meta  │
├─────────────────┤        ├──────────────────┤        ├─────────────────┤
│ id (PK)         │◄───┐   │ id (PK)          │◄───┐   │ id (PK)         │
│ name            │    │   │ name             │    │   │ volume_no       │
│ slug (UQ)       │    │   │ email (UQ)       │    │   │ issue_no        │
│ display_order   │    │   │ role             │    │   │ date_published  │
└─────────────────┘    │   │ title / beat     │    │   │ weather_summary │
                       │   │ pgp_fingerprint  │    │   └─────────────────┘
                       │   └──────────────────┘    │
                       │             ▲             │
                       │             │ (M:N)       │
                       │   ┌─────────────────────┐ │
                       │   │   article_authors   │ │
                       │   ├─────────────────────┤ │
                       │   │ article_id (FK) ────┼─┘
                       │   │ user_id (FK) ───────┘
                       │   │ role_descriptor     │
                       │   └─────────────────────┘
                       │             ▲
                       │             │
┌──────────────────────┴─────────────┴─────────────────┐
│                      articles                        │
├──────────────────────────────────────────────────────┤
│ id (PK, UUID/TEXT)                                   │
│ category_id (FK)                                     │
│ slug (UQ)                                            │
│ title                                                │
│ kicker                                               │
│ subhead                                              │
│ body_markdown                                        │
│ body_html                                            │
│ article_type ('lead', 'bulletin', 'oped', 'folio')   │
│ status ('draft', 'published', 'archived')            │
│ dispatch_ref_code                                    │
│ cover_image_url                                      │
│ cover_image_caption                                  │
│ cover_image_credit                                   │
│ reading_time_minutes                                 │
│ view_count                                           │
│ published_at                                         │
│ created_at / updated_at                              │
└──────────────────────┬───────────────────────────────┘
                       │
       ┌───────────────┴───────────────┐
       ▼                               ▼
┌─────────────────────────┐    ┌─────────────────────────┐
│    reader_dispatches    │    │      article_tags       │
├─────────────────────────┤    ├─────────────────────────┤
│ id (PK)                 │    │ article_id (FK)         │
│ article_id (FK)         │    │ tag_name                │
│ author_name             │    └─────────────────────────┘
│ affiliation             │
│ body                    │
│ is_editors_pick         │
│ is_technical_note       │
│ endorsements_count      │
│ created_at              │
└─────────────────────────┘

┌─────────────────────────┐    ┌─────────────────────────┐
│     wire_dispatches     │    │     market_ledgers      │
├─────────────────────────┤    ├─────────────────────────┤
│ id (PK)                 │    │ id (PK)                 │
│ headline                │    │ item_name               │
│ location                │    │ category ('metals',etc) │
│ gmt_timestamp           │    │ spot_value              │
│ is_urgent (BOOLEAN)     │    │ change_pct              │
│ published_at            │    │ updated_at              │
└─────────────────────────┘    └─────────────────────────┘

┌─────────────────────────┐
│ newsletter_subscribers  │
├─────────────────────────┤
│ id (PK)                 │
│ email (UQ)              │
│ is_active (BOOLEAN)     │
│ created_at              │
└─────────────────────────┘
```

---

### 2. DDL SQL (SQLite Schema Definition)

```sql
-- Pragmas untuk performa tinggi dan integritas data
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

-- 1. Users / Editorial Staff
CREATE TABLE users (
    id TEXT PRIMARY KEY, -- UUID v4
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin', 'editor', 'correspondent')),
    title TEXT, -- e.g. "Senior Mechanical Correspondent"
    bio TEXT,
    pgp_fingerprint TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. Categories
CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    display_order INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 3. Articles
CREATE TABLE articles (
    id TEXT PRIMARY KEY, -- UUID v4
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    kicker TEXT, -- e.g. "Special Investigation • Estuary Region"
    subhead TEXT, -- deck/subjudul artikel
    lead_paragraph TEXT, -- pembuka untuk dropcap
    body_markdown TEXT NOT NULL,
    body_html TEXT NOT NULL,
    article_type TEXT NOT NULL DEFAULT 'standard' 
        CHECK(article_type IN ('lead', 'standard', 'bulletin', 'oped', 'folio')),
    status TEXT NOT NULL DEFAULT 'draft' 
        CHECK(status IN ('draft', 'review', 'published', 'archived')),
    dispatch_ref_code TEXT, -- e.g. "DISPATCH #1094"
    cover_image_url TEXT,
    cover_image_caption TEXT,
    cover_image_credit TEXT,
    reading_time_minutes INTEGER DEFAULT 5,
    view_count INTEGER DEFAULT 0,
    published_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_articles_slug ON articles(slug);
CREATE INDEX idx_articles_status_published ON articles(status, published_at DESC);
CREATE INDEX idx_articles_type ON articles(article_type);

-- 4. Article Authors (Junction Table for Multi-Byline)
CREATE TABLE article_authors (
    article_id TEXT REFERENCES articles(id) ON DELETE CASCADE,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    role_descriptor TEXT DEFAULT 'Author', -- e.g. "Reporting By", "Photography By"
    display_order INTEGER DEFAULT 0,
    PRIMARY KEY (article_id, user_id)
);

-- 5. Telegraphic Wire Dispatches
CREATE TABLE wire_dispatches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    headline TEXT NOT NULL,
    location TEXT,
    gmt_timestamp TEXT NOT NULL, -- e.g. "11:42 GMT"
    is_urgent BOOLEAN DEFAULT 0,
    is_active BOOLEAN DEFAULT 1,
    published_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 6. Commercial Ledger / Market Commodities
CREATE TABLE market_ledgers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_name TEXT NOT NULL, -- e.g. "Bessemer Heavy 50", "Nordic Ore Spot"
    category TEXT NOT NULL DEFAULT 'commodities', -- 'commodities', 'currencies', 'bonds'
    specification TEXT, -- e.g. "London Metal Exchange Spot"
    spot_value TEXT NOT NULL, -- e.g. "4,128.90"
    change_pct REAL NOT NULL, -- e.g. +0.32 or -1.20
    unit TEXT, -- e.g. "/ MT", "pts"
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 7. Reader Dispatches (Komentar Beradab & Diskursus Terverifikasi)
CREATE TABLE reader_dispatches (
    id TEXT PRIMARY KEY, -- UUID v4
    article_id TEXT REFERENCES articles(id) ON DELETE CASCADE,
    author_name TEXT NOT NULL,
    affiliation TEXT, -- e.g. "Fellow, Institute of Structural Materials"
    body TEXT NOT NULL,
    is_editors_pick BOOLEAN DEFAULT 0,
    is_technical_note BOOLEAN DEFAULT 0,
    endorsements_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_reader_dispatches_article ON reader_dispatches(article_id, created_at DESC);

-- 8. Newsletter Subscribers
CREATE TABLE newsletter_subscribers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 9. Production Query Performance Indexes (Migration m20261013_040000_optimize_indexes)
CREATE INDEX idx_articles_status_pub ON articles(status, published_at DESC);
CREATE INDEX idx_articles_cat_pub ON articles(category_id, published_at DESC);
CREATE INDEX idx_articles_type ON articles(article_type);
CREATE INDEX idx_wires_active_urgent ON wire_dispatches(is_active, is_urgent DESC);
CREATE INDEX idx_dispatches_editors_pick ON reader_dispatches(is_editors_pick);
CREATE INDEX idx_markets_category ON market_ledgers(category);
```
