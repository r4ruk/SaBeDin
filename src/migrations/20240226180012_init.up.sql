-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "users" (
                id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
                name VARCHAR(100) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                verified BOOLEAN NOT NULL DEFAULT FALSE,
                password VARCHAR(100) NOT NULL,
                role VARCHAR(50) NOT NULL DEFAULT 'user',
                created_at TIMESTAMP
                       WITH
                           TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP
                       WITH
                           TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX users_email_idx ON users (email);

-- password of the created user is done by argon2 algorithm and equals "password" (without the ")
INSERT INTO users(name, email, verified, password, role)
VALUES ('validuser','validuser@test.com', true,'$argon2id$v=19$m=19456,t=2,p=1$5Fowt30D7yxpFW0ZyfAlDw$OSJLoBEQpTdj8dRelbUclJo6qq/iWnhK7CV8CcFU3Xk','user');



CREATE TABLE "idempotency" (
   user_id UUID NOT NULL REFERENCES users(id),
   idempotency_key TEXT NOT NULL,
   response_status_code SMALLINT NOT NULL,
   response_body BYTEA NOT NULL,
   created_at TIMESTAMP WITH Time ZONE DEFAULT NOW(),
   PRIMARY KEY(user_id, idempotency_key)
);



CREATE TABLE
    "articles" (
                id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
                programming_key_name VARCHAR(255),
                title VARCHAR,
                contents VARCHAR,
                tags VARCHAR,
                created_at TIMESTAMP
                    WITH TIME ZONE DEFAULT NOW()
);
CREATE UNIQUE INDEX articles_pkn ON articles(programming_key_name);
