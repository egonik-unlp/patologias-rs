use crate::{
    core::database::{
        connection::{interact, interact_with_connection},
        repository::{rank_suggestions, DatabaseRepository},
        schema::{citologias, histopatologias, studies},
    },
    impl_repository,
    reports::{
        dto::{CitoFields, HistoFields, PaginatedResponseDto, ReportFieldsDto, StudyDto},
        model::{
            Citologia, Histopatologia, NewCitologia, NewHistopatologia, NewStudy, Study, StudyType,
        },
    },
    suggestions,
};

impl_repository!(
    async, Study, studies, i32, NewStudy,
    children(study_id) {
        Citologia: citologias, NewCitologia;
        Histopatologia: histopatologias, NewHistopatologia;
    },
    kind = tipo: StudyType,
);
impl StudyRepository {
    pub async fn get_suggestions(
        &self,
        field: ReportFieldsDto,
        limit: usize,
    ) -> anyhow::Result<Vec<String>> {
        let raw_suggestions = interact_with_connection(&self.pool, |conn| match field {
            ReportFieldsDto::Citologia(cito_fields) => match cito_fields {
                CitoFields::Microflora => suggestions!(conn, citologias, citologias::microflora),
                CitoFields::CelulasPavimentosas => {
                    suggestions!(conn, citologias, citologias::celulas_pavimentosas)
                }
                CitoFields::CelulasMetaplasicas => {
                    suggestions!(conn, citologias, citologias::celulas_pavimentosas)
                }
                CitoFields::Leucocitos => {
                    suggestions!(conn, citologias, citologias::celulas_pavimentosas)
                }
                CitoFields::Hematies => {
                    suggestions!(conn, citologias, citologias::celulas_pavimentosas)
                }
                CitoFields::CelulasEndocervicales => {
                    suggestions!(conn, citologias, citologias::celulas_endocervicales)
                }
                CitoFields::CompatibleCon => {
                    suggestions!(conn, citologias, citologias::compatible_con)
                }
                CitoFields::Sugerencia => suggestions!(conn, citologias, citologias::sugerencia),
                CitoFields::DiagCitologias => {
                    suggestions!(conn, citologias, citologias::sugerencia)
                }
            },
            ReportFieldsDto::Histopatologia(histo_fields) => match histo_fields {
                HistoFields::DescripcionMacroscopica => suggestions!(
                    conn,
                    histopatologias,
                    histopatologias::descripcion_macroscopica
                ),
                HistoFields::DiagnosticoHistopatologico => suggestions!(
                    conn,
                    histopatologias,
                    histopatologias::descripcion_macroscopica
                ),
            },
        })
        .await?;
        Ok(rank_suggestions(
            raw_suggestions
                .into_iter()
                .filter_map(|(v, n)| Some((v?, n))),
            limit,
        ))
    }
    pub async fn get_shallow(&self) -> anyhow::Result<Vec<StudyDto>> {
        let repo = self.clone();
        interact(&self.pool, move |conn| {
            repo._read_all(conn)
                .map(|val| val.into_iter().map(Into::into).collect())
        })
        .await
    }
    pub async fn get_shallow_paginated(&self, offset: i32) -> anyhow::Result<PaginatedResponseDto> {
        _ = offset;
        todo!()
    }
}
