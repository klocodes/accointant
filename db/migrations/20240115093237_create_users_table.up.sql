CREATE TABLE IF NOT EXISTS users
(
    id                            uuid PRIMARY KEY,
    name                          VARCHAR(255) DEFAULT NULL,
    email                         VARCHAR(255) NOT NULL,
    password                      VARCHAR(255) NOT NULL,
    confirmation_token            VARCHAR(255) NOT NULL,
    confirmation_token_expires_at TIMESTAMPTZ  NOT NULL,  DEFAULT NULL,
    confirmed_at                  TIMESTAMPTZ             DEFAULT NULL,
    created_at                    TIMESTAMPTZ  NOT NULL   DEFAULT CURRENT_TIMESTAMP,
    updated_at                    TIMESTAMPTZ  NOT NULL   DEFAULT CURRENT_TIMESTAMP,
    deleted_at                    TIMESTAMPTZ             DEFAULT NULL
);