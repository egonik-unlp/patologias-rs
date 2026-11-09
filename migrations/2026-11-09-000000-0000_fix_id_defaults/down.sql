CREATE SEQUENCE histopatologias_study_id_seq OWNED BY histopatologias.study_id;
ALTER TABLE histopatologias ALTER COLUMN study_id SET DEFAULT nextval('histopatologias_study_id_seq');
CREATE SEQUENCE citologias_study_id_seq OWNED BY citologias.study_id;
ALTER TABLE citologias ALTER COLUMN study_id SET DEFAULT nextval('citologias_study_id_seq');

CREATE SEQUENCE studies_material_id_seq OWNED BY studies.material_id;
ALTER TABLE studies ALTER COLUMN material_id SET DEFAULT nextval('studies_material_id_seq');
CREATE SEQUENCE studies_doctor_id_seq OWNED BY studies.doctor_id;
ALTER TABLE studies ALTER COLUMN doctor_id SET DEFAULT nextval('studies_doctor_id_seq');

ALTER TABLE materials ALTER COLUMN id DROP IDENTITY;
ALTER TABLE doctors ALTER COLUMN id DROP IDENTITY;
