create table users
(
    id         serial primary key,
    email      varchar(255) not null unique,
    password_hash   varchar(255) not null,
    created_at timestamptz  not null default now(),
    updated_at timestamptz  not null default now()
);

create type theme as enum ('dark', 'light');

create table settings
(
    id                    serial primary key,
    user_id               integer     not null references users (id) on delete cascade,
    theme                 theme       not null default 'light',
    notifications_enabled boolean     not null default true,
    radius                integer     not null default 50,
    created_at            timestamptz not null default now(),
    updated_at            timestamptz not null default now()
);


create table natural_phenomenon_locations
(
    id          serial primary key,
    user_id     integer          not null references users (id) on delete cascade,
    name        varchar(100)     not null,
    latitude    double precision not null,
    longitude   double precision not null,
    description text,
    created_at  timestamptz      not null default now(),
    updated_at  timestamptz      not null default now()
);

create table weather_locations
(
    id           serial primary key,
    user_id     integer          not null references users (id) on delete cascade,
    name        varchar(100)     not null,
    latitude    double precision not null,
    longitude   double precision not null,
    is_default  bool not null default false,
    description text,
    created_at  timestamptz      not null default now(),
    updated_at  timestamptz      not null default now()
);

create unique index one_default_weather_location_per_user
on weather_locations(user_id)
where is_default = true;

