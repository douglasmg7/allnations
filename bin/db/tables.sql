-- drop table if exists entrance;
-- enable foreign keys
-- not working, reset to off when back to db
pragma foreign_keys = on;

-- Parameters.
CREATE TABLE IF NOT EXISTS param (
	name                    TEXT PRIMARY KEY,	-- Name without space.
	value                   TEXT
);

-- Categories.
CREATE TABLE IF NOT EXISTS category (
	name                    TEXT PRIMARY KEY,	-- Name without space.
	text                    TEXT NOT NULL,
	products_qty            INTEGER NOT NULL,
	selected                BOOLEAN NOT NULL
);

-- Products.
CREATE TABLE IF NOT EXISTS product (
    zunka_product_id		TEXT DEFAULT "",
    code                    TEXT NULL UNIQUE,	-- From dealer.
    description             TEXT NOT NULL,
    timestamp               DATE NOT NULL,
    department              TEXT NOT NULL,
    category                TEXT NOT NULL,
    sub_category            TEXT NOT NULL,
    maker                   TEXT NOT NULL,
    technical_description   TEXT NOT NULL,
    url_image               TEXT NOT NULL,
    part_number             TEXT NOT NULL,
    ean                     TEXT NOT NULL,
    ncm                     TEXT NOT NULL,
    price_sale              INTEGER NOT NULL,
    price_without_st        INTEGER NOT NULL,
    icms_st_taxation        BOOLEAN NOT NULL,
    warranty_month          INTEGER,
    length_mm               INTEGER NOT NULL,
    width_mm                INTEGER NOT NULL,
    height_mm               INTEGER NOT NULL,
    weight_g                INTEGER NOT NULL,
    active                  BOOLEAN NOT NULL,
    availability            BOOLEAN NOT NULL,
    origin                  TEXT,
    stock_origin            TEXT,
    stock_qtd               INTEGER NOT NULL,
    created_at              DATE NOT NULL,
    changed_at              DATE NOT NULL,
    checked_at              DATE DEFAULT "0001-01-01T03:00:00-03:00",
    removed_at              DATE DEFAULT "0001-01-01T03:00:00-03:00",
    CHECK(TRIM(code, " ") <> '')
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_product_code ON product(code);

-- Products history.
CREATE TABLE IF NOT EXISTS product_history (
    zunka_product_id		TEXT DEFAULT "",
    code                    TEXT NOT NULL UNIQUE,	-- From dealer.
    description             TEXT NOT NULL,
    timestamp               DATE NOT NULL,
    department              TEXT NOT NULL,
    category                TEXT NOT NULL,
    sub_category            TEXT NOT NULL,
    maker                   TEXT NOT NULL,
    technical_description   TEXT NOT NULL,
    url_image               TEXT NOT NULL,
    part_number             TEXT NOT NULL,
    ean                     TEXT NOT NULL,
    ncm                     TEXT NOT NULL,
    price_sale              INTEGER NOT NULL,
    price_without_st        INTEGER NOT NULL,
    icms_st_taxation        BOOLEAN NOT NULL,
    warranty_month          INTEGER,
    length_mm               INTEGER NOT NULL,
    width_mm                INTEGER NOT NULL,
    height_mm               INTEGER NOT NULL,
    weight_g                INTEGER NOT NULL,
    active                  BOOLEAN NOT NULL,
    availability            BOOLEAN NOT NULL,
    origin                  TEXT,
    stock_origin            TEXT,
    stock_qtd               INTEGER NOT NULL,
    created_at              DATE NOT NULL,
    changed_at              DATE NOT NULL,
    checked_at              DATE DEFAULT "0001-01-01T03:00:00-03:00",
    removed_at              DATE DEFAULT "0001-01-01T03:00:00-03:00",
    CHECK(TRIM(code, " ") <> ''),
    UNIQUE (code, changed_at)
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_product_history_code_changed_at ON product_history(code, changed_at);
