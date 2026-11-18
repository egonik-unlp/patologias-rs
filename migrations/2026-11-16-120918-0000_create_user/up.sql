create type roles as enum( 'operator', 'admin');


create table if not exists users (
      id serial primary key,
      username  varchar(255) not null unique,
      full_name varchar(255) not null,
    password_hash TEXT NOT NULL,
      role ROLES not null,
      active boolean not null
);

create table if not exists sessions (
   id serial primary key,
  token varchar(255) not null unique,
    user_id integer not null references users(id) on delete cascade,
   expires_at timestamptz not null
);

create index if not exists sessions_user_id_idx on sessions (user_id);
