use crate::{
    entities::dto::DoctorFieldsDto,
    frontend::suggestions::SuggestInput,
    reports::{
        dto::{CitoFields, HistoFields, ReportFieldsDto, StudyTypeDto},
        server::read_all_and_stringify,
    },
};

use leptos::prelude::*;
use leptos_router::{components::Form, hooks::use_query_map};

macro_rules! query_map_to_field {
    ( $($param:ident),*   ) => {
        let query = leptos_router::hooks::use_query_map();
        $(
        let $param = move || query.read().get(stringify!($param)).unwrap_or_default();
        )*
    };
}

macro_rules! html_and_suggestions {
    ($(($param:ident, $tp:expr)),* ) => {

        view!{
        <div class= "flex flex-col max-w-lg bg-center border-b-slate-700">
        $(
        <div class="flex-2 justify-between">
            <label for=stringify!($param)>  {stringify!($param) }  </label>
            <SuggestInput field=$tp id=stringify!($param) value = $param/>

        </div>
        )*
        </div>
        }
     };
}

#[component]
pub fn Reports() -> impl IntoView {
    let resource = OnceResource::new(read_all_and_stringify());
    view! {
        <h1>AAAAAAAAAAAAAAAA</h1>
        // Resources must be read inside <Suspense>: otherwise the server renders
        // the fallback while the hydrating client already has the data, and the
        // two HTML trees don't match.
        <Suspense fallback=|| {
            view! { <div>"Floripondio"</div> }
        }>
            {move || {
                resource
                    .get()
                    .map(|rec| match rec {
                        Ok(val) => {
                            view! {
                                <div>
                                    {val
                                        .into_iter()
                                        .rev()
                                        .take(20)
                                        .map(|val| view! { <p>{format!("{:?}", val)}</p> })
                                        .collect_view()}
                                </div>
                            }
                                .into_any()
                        }
                        Err(e) => view! { <div>{e.to_string()}</div> }.into_any(),
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn CreateReport() -> impl IntoView {
    let (rep_type, set_rep_type) = signal(StudyTypeDto::Citologia);
    _ = set_rep_type;
    view! {
        <div class="flex-2">
            <ReportHeader />
            {match rep_type.get() {
                StudyTypeDto::Histopatologia => view! { <Citologias /> }.into_any(),
                StudyTypeDto::Citologia => view! { <Histopatologia /> }.into_any(),
            }}

        </div>
    }
}

#[component]
pub fn ReportHeader() -> impl IntoView {
    // reactive access to URL query
    let query = use_query_map();
    let protocolo = move || query.read().get("number").unwrap_or_default();
    let fecha = move || query.read().get("select").unwrap_or_default();
    let paciente = move || query.read().get("paciente").unwrap_or_default();
    let doctor = move || query.read().get("name").unwrap_or_default();
    let material = move || query.read().get("name").unwrap_or_default();

    view! {
        <h2>"Header"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <div class="flex flex-col max-w-lg">
                <div class = "flex-2 justify-around">
                <label for="protocolo"> Protocolo </label>
                <input type="text" name="protocolo" value=protocolo />
                </div>
                <div class = "flex-2 justify-around">
                    <label for="fecha"> Fecha </label>
                <input type="text" name="fecha" value=fecha />
                </div>
                <div class = "flex-2 justify-around">
                <label for="paciente"> Paciente </label>
                <input type="text" name="paciente" value=paciente />
                </div>
                <div class = "flex-2 justify-around">
                <label for="doctor"> Doctor </label>
                <SuggestInput field=DoctorFieldsDto::Name id="doctor" value=doctor />
                </div>
                <div class = "flex-2 justify-around">
                <label for="material"> Material </label>
                <SuggestInput field=DoctorFieldsDto::Name id="material" value=material/>
                </div>
            </div>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit" />
        </Form>
        <Citologias />
    }
}

#[component]
pub fn Histopatologia() -> impl IntoView {
    // reactive access to URL query
    let query = use_query_map();
    let descripcion_macroscopica = move || {
        query
            .read()
            .get("descripcion_macroscopica")
            .unwrap_or_default()
    };
    let diag_histopatologico = move || query.read().get("diag_histopatologico").unwrap_or_default();

    view! {
        <h2>"Histopatologia Specific fields"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            {html_and_suggestions!(
                (descripcion_macroscopica, ReportFieldsDto::Histopatologia(HistoFields::DescripcionMacroscopica)),
            (diag_histopatologico, ReportFieldsDto::Histopatologia(HistoFields::DiagnosticoHistopatologico))
            )}

            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit" />
        </Form>
    }
}

#[component]
pub fn Citologias() -> impl IntoView {
    // reactive access to URL query
    query_map_to_field!(
        microflora,
        celulas_pavimentosas,
        celulas_metaplasicas,
        leucocitos,
        hematies,
        celulas_endocervicales,
        compatible_con,
        sugerencia,
        diag_citologias
    );
    view! {
        <h2>"Citologia Specific fields"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            {html_and_suggestions!(
                (microflora, ReportFieldsDto::Citologia(CitoFields::Microflora)),
                    (leucocitos, ReportFieldsDto::Citologia(CitoFields::Leucocitos)),
                    (celulas_pavimentosas, ReportFieldsDto::Citologia(CitoFields::CelulasPavimentosas)),
                    (celulas_metaplasicas, ReportFieldsDto::Citologia(CitoFields::CelulasMetaplasicas)),
                    (hematies, ReportFieldsDto::Citologia(CitoFields::Hematies)),
                    (celulas_endocervicales, ReportFieldsDto::Citologia(CitoFields::CelulasEndocervicales)),
                    (compatible_con, ReportFieldsDto::Citologia(CitoFields::CompatibleCon)),
                    (sugerencia, ReportFieldsDto::Citologia(CitoFields::Sugerencia)),
                    (diag_citologias, ReportFieldsDto::Citologia(CitoFields::DiagCitologias))
            )}
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit" />
        </Form>
    }
}
