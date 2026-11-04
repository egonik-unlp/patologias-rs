-- Your SQL goes here
--
-- pub struct Histologia {
--     microflora: String,
--     celulas_pavimentosas: String,
--     celulas_metaplasicas: String,
--     leucocitos: String,
--     hematies: String,
--     celulas_endocervicales: String,
--     compatible_con: String,
--     sugerencia: String,
--     diag_citologias: String,
-- }
--
create table if not exists histopatologias (
    id serial primary key,
  study_id serial not null,
  descripcion_macroscopica text,
  diagnostio_histopatologico text,

  foreign key (study_id) references studies(id)
)
