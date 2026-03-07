-- Initial schema

CREATE TABLE IF NOT EXISTS company (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    logo            BLOB,
    phone           TEXT,
    email           TEXT,
    addr1           TEXT,
    addr2           TEXT,
    city            TEXT,
    state           TEXT,
    zip             TEXT
);

CREATE TABLE IF NOT EXISTS client (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    phone           TEXT,
    email           TEXT,
    addr1           TEXT,
    addr2           TEXT,
    city            TEXT,
    state           TEXT,
    zip             TEXT
);

CREATE TABLE IF NOT EXISTS items (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    rate            INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS terms (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    due             INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS methods (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    link            TEXT,
    qr              BLOB
);  

CREATE TABLE IF NOT EXISTS templates (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    company_id      INTEGER NOT NULL REFERENCES company(id),
    client_id       INTEGER NOT NULL REFERENCES client(id),
    terms_id        INTEGER NOT NULL REFERENCES terms(id),
    methods_json    TEXT    NOT NULL DEFAULT '[]'
);

CREATE TABLE IF NOT EXISTS invoices (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    template_id     INTEGER NOT NULL REFERENCES templates(id),
    date            TEXT    NOT NULL,
    show_methods    INTEGER NOT NULL DEFAULT 1,
    show_notes      INTEGER NOT NULL DEFAULT 1,
    stage           TEXT    NOT NULL DEFAULT 'Invoice',
    status          TEXT    NOT NULL DEFAULT 'Waiting',
    status_date     TEXT,
    status_check    TEXT,
    notes           TEXT,
    items_json      TEXT    NOT NULL DEFAULT '[]'
);

CREATE TABLE IF NOT EXISTS email_config (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    smtp_server     TEXT    NOT NULL,
    port            INTEGER NOT NULL,
    tls             INTEGER NOT NULL DEFAULT 1,
    username        TEXT    NOT NULL,
    password        TEXT    NOT NULL,
    fromname        TEXT    NOT NULL
);
