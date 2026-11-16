-- Your SQL goes here

create type privilegies as enum('reader', 'editor', 'admin');


create table if not exists users (
      id serial primary key,
      username  varchar(255) not null,
      privilegies PRIVILEGIES
)
