-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP type if exists role_t;

create type role_t as enum ('SUPERADMIN','ADMIN');

create table users(
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    name varchar,
    email varchar unique not null,
    phone varchar unique not null,
    password text not null,
    role role_t not null
);