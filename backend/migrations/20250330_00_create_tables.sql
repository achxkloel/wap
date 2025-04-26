create table users
(
    id            serial primary key,
    email         varchar(255) not null unique,
    password_hash varchar(255) not null,
    first_name    varchar(100)          default null,
    last_name     varchar(100)          default null,
    image_url     varchar(255)          default null,
    provider      varchar(255)          default null,
    google_id     varchar(255) unique   default null,
    created_at    timestamptz  not null default now(),
    updated_at    timestamptz  not null default now()
);

create type theme as enum ('dark', 'light');

create table settings
(
    id                    serial primary key,
    user_id               integer references users (id) on delete set null default null, -- Can be null when user deletes account
    theme                 theme          not null                                  default 'light',
    notifications_enabled boolean                                          default true,
    radius                integer                                          default 50,
    created_at            timestamptz                                      default now(),
    updated_at            timestamptz                                      default now()
);


create table natural_phenomenon_locations
(
    id          serial primary key,
    user_id     integer          not null references users (id) on delete set null,
    name        varchar(100)     not null,
    latitude    double precision not null,
    longitude   double precision not null,
    image_path  varchar(255)     not null default '',
    radius      integer          not null,
    description text             not null default '',
    created_at  timestamptz      not null default now(),
    updated_at  timestamptz      not null default now()
);

create table weather_locations
(
    id          serial primary key,
    user_id     integer          not null references users (id) on delete set null,
    name        varchar(100)     not null,
    latitude    double precision not null,
    longitude   double precision not null,
    is_default  bool             not null default false,
    description text             not null default '',
    created_at  timestamptz      not null default now(),
    updated_at  timestamptz      not null default now()
);

create unique index one_default_weather_location_per_user
    on weather_locations (user_id)
    where is_default = true;

