CREATE TABLE IF NOT EXISTS tag_events
(
    id                            uuid PRIMARY KEY,
    name                          VARCHAR(255) DEFAULT NULL,
    payload                       JSONB DEFAULT NULL,
    version                       INT NOT NULL DEFAULT 0,
    created_at                    TIMESTAMPTZ  NOT NULL   DEFAULT CURRENT_TIMESTAMP
    );