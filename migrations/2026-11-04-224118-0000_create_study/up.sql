-- Your SQL goes here
Create type study_type as enum('histopatologia', 'citologia');


CREATE TABLE IF NOT EXISTS studies (
  id serial primary key,
  protocolo varchar(255) not null,
  fecha timestamp not null,
  tipo study_type not null,
  paciente varchar(255) not null,
  doctor_id serial not null,
  material_id serial not null,
  created_at  TIMESTAMP NOT NULL DEFAULT now(),
  updated_at  TIMESTAMP NOT NULL DEFAULT now(),
  foreign key (doctor_id) references doctors(id),
  foreign key (material_id) references materials(id)
);
