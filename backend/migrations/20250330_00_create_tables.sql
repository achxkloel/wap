create table users
(
    id         serial primary key,
    email      varchar(255) not null unique,
    password   varchar(255) not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
