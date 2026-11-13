// 'protocolo', 'fecha', 'paciente', 'medico', 'material', 'microflora',
//        'celulas_pavimentosas', 'celulas_metaplasicas', 'leucocitos',
//        'hematies', 'celulas_endocervicales', 'compatible_con', 'sugerencia',
//        'diag_citologias', 'descripcion_macroscopica', 'diagnostico_histopat',
//        'tipo', 'imprimir', 'copia
//
use std::collections::HashMap;

use diesel::prelude::*;
use patologias_rs::core::config::model::Config;
use patologias_rs::core::database::connection::{get_connection_pool, interact, DbPool};
use patologias_rs::core::database::repository::{AsyncLayer, AsyncLayerWithHierarchy};
use patologias_rs::core::database::schema::{doctors, materials, studies};
use patologias_rs::entities::model::{NewDoctor, NewMaterial};
use patologias_rs::entities::repository::{DoctorRepository, MaterialRepository};
use patologias_rs::reports::model::{NewCitologia, NewHistopatologia, NewStudy, StudyType};
use patologias_rs::reports::repository::{NewStudyDetail, StudyRepository};
use serde::Deserialize;

const CSV_PATH: &str = "./data/studies-test.csv";

/// The doctor/material columns, which the CSV holds as names, not ids.
#[derive(Debug, Deserialize)]
struct CsvNames {
    medico: String,
    material: String,
}

#[derive(Debug)]
struct CsvReport {
    names: CsvNames,
    study: NewStudy,
    histopatologia: NewHistopatologia,
    citologia: NewCitologia,
}

fn main() -> anyhow::Result<()> {
    let pool = get_connection_pool(Config::from_env().database_url);
    tokio::runtime::Runtime::new()?.block_on(load(pool))
}

async fn load(pool: DbPool) -> anyhow::Result<()> {
    // protocolo isn't unique, so a second run would duplicate every study.
    let existing: i64 = interact(&pool, |conn| Ok(studies::table.count().get_result(conn)?)).await?;
    anyhow::ensure!(
        existing == 0,
        "studies table already has {existing} rows; refusing to load again"
    );

    let doctors_repo = DoctorRepository::new(pool.clone());
    let materials_repo = MaterialRepository::new(pool.clone());
    let studies_repo = StudyRepository::new(pool.clone());

    // name -> id, so each distinct name is created only once.
    let mut doctor_ids: HashMap<String, i32> = interact(&pool, |conn| {
        Ok(doctors::table.select((doctors::name, doctors::id)).load(conn)?.into_iter().collect())
    })
    .await?;
    let mut material_ids: HashMap<String, i32> = interact(&pool, |conn| {
        Ok(materials::table.select((materials::name, materials::id)).load(conn)?.into_iter().collect())
    })
    .await?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true) // Tell it to look for a header row
        .trim(csv::Trim::All) // Automatically strip whitespace around fields
        .flexible(true)
        .delimiter(b',')
        .from_path(CSV_PATH)?;
    // Deserialize each part from the raw record instead of going through
    // `#[serde(flatten)]`: with flatten, csv infers field types, turning
    // protocolo "07122619" into the integer 7122619.
    let headers = rdr.headers()?.clone();
    let (mut loaded, mut skipped) = (0usize, 0usize);
    for record in rdr.records() {
        let report = record.and_then(|rec| {
            Ok(CsvReport {
                names: rec.deserialize(Some(&headers))?,
                study: rec.deserialize(Some(&headers))?,
                histopatologia: rec.deserialize(Some(&headers))?,
                citologia: rec.deserialize(Some(&headers))?,
            })
        });
        let CsvReport { names, mut study, histopatologia, citologia } = match report {
            Ok(report) => report,
            Err(e) => {
                eprintln!("skipping record: {e}");
                skipped += 1;
                continue;
            }
        };

        study.doctor_id = match doctor_ids.get(&names.medico) {
            Some(id) => *id,
            None => {
                let doctor = doctors_repo.create(NewDoctor::new(names.medico.clone())).await?;
                *doctor_ids.entry(names.medico).or_insert(*doctor.id())
            }
        };
        study.material_id = match material_ids.get(&names.material) {
            Some(id) => *id,
            None => {
                let material = materials_repo.create(NewMaterial::new(names.material.clone())).await?;
                *material_ids.entry(names.material).or_insert(*material.id())
            }
        };

        let detail = match study.tipo {
            StudyType::Citologia => NewStudyDetail::Citologia(citologia),
            StudyType::Histopatologia => NewStudyDetail::Histopatologia(histopatologia),
        };
        let protocolo = study.protocolo.clone();
        match studies_repo.create(study, detail).await {
            Ok(_) => loaded += 1,
            Err(e) => {
                eprintln!("failed to insert protocolo {protocolo:?}: {e:#}");
                skipped += 1;
            }
        }
    }

    println!(
        "loaded {loaded} studies ({} doctors, {} materials), skipped {skipped}",
        doctor_ids.len(),
        material_ids.len()
    );
    Ok(())
}
