
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "fence" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ts TIMESTAMP[2] NOT NULL,
    lat REAL[2] NOT NULL,
    lng REAL[2] NOT NULL,
    tcount INT NOT NULL DEFAULT 0,
    fcount INT NOT NULL DEFAULT 0
);

CREATE TABLE "proof" (
    id TEXT PRIMARY KEY NOT NULL,
    fid UUID REFERENCES fence(id) NOT NULL,
    result BOOL NOT NULL
)
