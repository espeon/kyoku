-- Add migration script here
CREATE TABLE artist (
  id serial primary key,
  name varchar not null,
  bio varchar,
  picture varchar,
  created_at timestamp with time zone not null,
  updated_at timestamp with time zone,
  tags varchar
);

CREATE TABLE album (
  id serial primary key,
  name varchar not null,
  artist integer not null,
  picture varchar,
  year integer,
  created_at timestamp with time zone not null,
  updated_at timestamp with time zone,
  FOREIGN KEY (artist) REFERENCES artist (id)
  on delete cascade
);

CREATE TABLE genre (
  id serial primary key,
  name varchar,
  created_at timestamp with time zone not null,
  updated_at timestamp with time zone
);

CREATE TABLE song (
  id serial primary key,
  number integer,
  name varchar not null,
  path varchar not null,
  album integer not null,
  artist integer not null,
  liked bool,
  duration integer not null,
  last_play timestamp with time zone,
  plays integer,
  lossless bool,
  genre integer,
  created_at timestamp with time zone not null,
  updated_at timestamp with time zone,
  FOREIGN KEY (album) REFERENCES album (id),
  FOREIGN KEY (artist) REFERENCES artist (id),
  FOREIGN KEY (genre) REFERENCES genre (id)
);

CREATE TABLE server (
  id serial primary key,
  scan_start timestamp with time zone,
  scan_end timestamp with time zone,
  last_scan timestamp with time zone,
  seconds integer,
  albums integer,
  artists integer,
  size integer
);