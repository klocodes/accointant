CREATE TABLE users
(
    id                            uuid PRIMARY KEY,
    name                          VARCHAR(255) NOT NULL,
    email                         VARCHAR(255) NOT NULL,
    password                      VARCHAR(255) NOT NULL,
    confirmation_token            VARCHAR(255)          DEFAULT NULL,
    confirmation_token_expires_at TIMESTAMP             DEFAULT NULL,
    confirmed_at                  TIMESTAMP             DEFAULT NULL,
    created_at                    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at                    TIMESTAMP             DEFAULT NULL
);