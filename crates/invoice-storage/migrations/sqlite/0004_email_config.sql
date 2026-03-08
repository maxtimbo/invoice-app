CREATE TABLE IF NOT EXISTS email_config (
    id          INTEGER PRIMARY KEY CHECK (id = 1),
    smtp_server TEXT    NOT NULL,
    port        INTEGER NOT NULL,
    tls         INTEGER NOT NULL DEFAULT 1,
    username    TEXT    NOT NULL,
    password    TEXT    NOT NULL,
    fromname    TEXT    NOT NULL
);

