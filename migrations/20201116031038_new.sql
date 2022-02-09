-- Add migration script here
CREATE TABLE song (
  id integer primary key autoincrement,
  number integer,
  name varchar(255) not null,
  path varchar(255) not null,
  album integer not null,
  artist varchar(255) not null,
  artists varchar(255) not null,
  liked bool,
  duration integer not null,
  last_play datetime,
  plays integer,
  lossless bool,
  genre integer,
  created_at datetime not null,
  updated_at datetime,
  FOREIGN KEY (album) REFERENCES album (id),
  FOREIGN KEY (artists) REFERENCES artist (id)
  
);

CREATE TABLE album (
  id integer primary key autoincrement,
  name varchar(255) not null,
  artist integet not null,
  picture varchar(255),
  year integer,
  created_at datetime not null,
  updated_at datetime,
  FOREIGN KEY (artist) REFERENCES artist (id)
  on delete cascade
);

CREATE TABLE artist (
  id integer primary key autoincrement,
  name varchar(255) not null,
  bio varchar(255),
  picture varchar(255),
  created_at datetime not null,
  updated_at datetime,
  similar varchar(255),
  tags varchar(255)
);

CREATE TABLE genre (
  id integer primary key autoincrement,
  name varchar(255),
  created_at datetime not null,
  updated_at datetime,
  FOREIGN KEY (id) REFERENCES song (genre)
);

CREATE TABLE playlist (
  id integer primary key autoincrement,
  title varchar(255),
  song integer,
  created_at datetime not null,
  updated_at datetime,
  FOREIGN KEY (song) REFERENCES song (id)
);

CREATE TABLE server (
  id integer primary key autoincrement,
  start datetime,
  end datetime,
  last_scan datetime,
  seconds integer,
  albums integer,
  artists integer,
  size integer
);
