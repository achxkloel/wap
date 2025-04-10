CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    email      VARCHAR(255) NOT NULL UNIQUE,
    password   VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TYPE theme AS ENUM ('dark', 'light');

CREATE TABLE settings
(
    id                    SERIAL PRIMARY KEY,
    user_id               INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    theme                 theme   NOT NULL DEFAULT 'light',
    notifications_enabled BOOLEAN NOT NULL DEFAULT true,
    radius                INTEGER NOT NULL DEFAULT 50,
    created_at            TIMESTAMPTZ      DEFAULT NOW(),
    updated_at            TIMESTAMPTZ      DEFAULT NOW()
);
