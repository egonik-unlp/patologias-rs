// 'protocolo', 'fecha', 'paciente', 'medico', 'material', 'microflora',
//        'celulas_pavimentosas', 'celulas_metaplasicas', 'leucocitos',
//        'hematies', 'celulas_endocervicales', 'compatible_con', 'sugerencia',
//        'diag_citologias', 'descripcion_macroscopica', 'diagnostico_histopat',
//        'tipo', 'imprimir', 'copia
//
use patologias_rs::reports::model::{NewCitologia, NewHistopatologia, NewStudy};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CsvReport {
    #[serde(flatten)]
    study: NewStudy,
    #[serde(flatten)]
    histopatologia: NewHistopatologia,
    #[serde(flatten)]
    citologia: NewCitologia,
}

fn main() -> anyhow::Result<()> {
    let study = NewStudy::default();
    let histopatologia = NewHistopatologia::default();
    let citologia = NewCitologia::default();
    let studio = CsvReport {
        study,
        citologia,
        histopatologia,
    };

    let dumpino = serde_json::to_string(&studio).unwrap();
    println!("{}", dumpino);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true) // Tell it to look for a header row
        .trim(csv::Trim::All) // Automatically strip whitespace around fields
        .flexible(true)
        .delimiter(b',')
        .from_path("./data/studies-test.csv")?;
    let mut rdrh = csv::Reader::from_path("./data/studies-test.csv")?;
    for (n, (result, raw_record)) in rdr
        .deserialize::<CsvReport>()
        .zip(rdrh.records())
        .enumerate()
    {
        if n > 6 {
            break;
        }
        if let Ok(report) = result {
            println!("{:?}", report);
        } else {
            let rr = raw_record.unwrap();
            println!("Error with line: {:?}\n{:?}\n\n", result, rr)
        }
    }
    Ok(())
}
