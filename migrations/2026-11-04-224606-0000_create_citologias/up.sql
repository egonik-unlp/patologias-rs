-- Your SQL goes here
create table if not exists citologias (
  id serial PRIMARY KEY,
  study_id serial not null,
  microflora text,
  celulas_pavimentosas text,
  celulas_metaplasicas text,
  leucocitos text,
  hematies text,
  celulas_endocervicales text,
  compatible_con text,
  sugerencia text,
  diag_citologias text,

  FOREIGN KEY (study_id)
    REFERENCES studies(id)
)
