use crate::{
    entities::server::{get_all_doctors, get_all_materials},
    frontend::{
        icons::{IconCitologia, IconHistopatologia},
        suggestions::SuggestInput,
    },
    reports::{
        dto::{
            CitoFields, CitologiaDto, FullStudyDto, HistoFields, HistopatologiaDto,
            ReportFieldsDto, StudyDetailDto, StudyDto, StudyTypeDto,
        },
        server::{read_all, read_report, CreateStudy},
    },
};
use std::str::FromStr;

use chrono::NaiveDateTime;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[macro_export]
macro_rules! query_map_to_field {
    ( $($param:ident),*   ) => {
        let query = leptos_router::hooks::use_query_map();
        $(
        let $param = move || query.read().get(stringify!($param)).unwrap_or_default();
        )*
    };
}

#[component]
pub fn Report() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    let id = id().parse::<i32>().expect("Blew up");
    view! {
        <Await future=read_report(id) let:data>
            <ReportView report=data.clone().unwrap() />
        </Await>
    }
}

#[component]
fn ReportView(report: FullStudyDto) -> impl IntoView {
    let FullStudyDto { parent, child } = report;
    view! {
        <div class="flex flex-col px-4 py-2 gap-4  max-w-2xl mx-auto border border-slate-500 bg-white rounded-lg shadow">
            <ShowParent parent />
            <ShowChild child />
        </div>
    }
}

#[component]
fn ShowChild(child: StudyDetailDto) -> impl IntoView {
    match child {
        StudyDetailDto::Citologia(citologia_props) => {
            view! { <CitoInforme citologia=citologia_props /> }.into_any()
        }
        StudyDetailDto::Histopatologia(histopatologia) => {
            view! { <HistoInforme histopatologia /> }.into_any()
        }
    }
}

macro_rules! simple_report_html {
    (
        $(straight: $($field_straight:ident),* $(,)? ;)?
        formatted: $($field_formatted:ident),* $(,)?
    ) => {
        ::leptos::view! {
            <div>
                $($(
                    <div>
                        {stringify!($field_straight).to_uppercase()} " : " {$field_straight}

                    </div>
                )*)?
                $(
                    <div>
                        {stringify!($field_formatted).to_uppercase()} " : " {format!("{:?}", $field_formatted)}
                    </div>
                )*
            </div>
        }
    };
}

// both work now:

#[component]
fn CitoInforme(citologia: CitologiaDto) -> impl IntoView {
    let CitologiaDto {
        microflora,
        celulas_pavimentosas,
        celulas_metaplasicas,
        leucocitos,
        hematies,
        celulas_endocervicales,
        ..
    } = citologia;

    view! {
        {simple_report_html!(
            straight: microflora, celulas_pavimentosas, celulas_metaplasicas, leucocitos, hematies; formatted: celulas_endocervicales
        )}
    }
}

#[component]
fn HistoInforme(histopatologia: HistopatologiaDto) -> impl IntoView {
    let HistopatologiaDto {
        descripcion_macroscopica,
        diagnostico_histopatologico,
    } = histopatologia;

    view! {
        {simple_report_html!(
            straight: descripcion_macroscopica; formatted: diagnostico_histopatologico
        )}
    }
}

#[component]
fn ShowParent(parent: StudyDto) -> impl IntoView {
    let StudyDto {
        protocolo,
        fecha,
        paciente,
        doctor_id,
        material_id,
        tipo,
        ..
    } = parent;
    view! {
        <div>"Protocolo": {protocolo}</div>

        <div>"Fecha": {format!("{}", fecha)}</div>

        <div>"Parciente": {paciente}</div>
        <div>"Doctor Id": {doctor_id}</div>
        <div>"Material id": {material_id}</div>
        <div class="flex items-center gap-1.5">
            "Tipo: " <StudyTypeIcon tipo=tipo.clone() class="size-4" /> {format!("{:?}", tipo)}
        </div>
    }
}

/// Icon that tells the two study types apart at a glance (cell vs. slide).
#[component]
fn StudyTypeIcon(tipo: StudyTypeDto, #[prop(into, optional)] class: String) -> impl IntoView {
    match tipo {
        StudyTypeDto::Citologia => view! { <IconCitologia class /> }.into_any(),
        StudyTypeDto::Histopatologia => view! { <IconHistopatologia class /> }.into_any(),
    }
}

#[component]
pub fn Reports() -> impl IntoView {
    let resource = OnceResource::new(read_all());
    view! {
        <h1>Citologia Reports Table</h1>
        // Resources must be read inside <Suspense>: otherwise the server renders
        // the fallback while the hydrating client already has the data, and the
        // two HTML trees don't match.
        <Suspense fallback=|| {
            view! { <div>"Loading..."</div> }
        }>
            {move || {
                resource
                    .get()
                    .map(|rec| match rec {
                        Ok(reports) => view! { <ReportTable reports /> }.into_any(),
                        Err(e) => view! { <div>{e.to_string()}</div> }.into_any(),
                    })
            }}
        </Suspense>
    }
}

#[derive(Debug, Clone)]
struct ReportRowFields {
    pub protocolo: String,
    pub fecha: NaiveDateTime,
    pub paciente: String,
    pub tipo: StudyTypeDto,
}

impl From<FullStudyDto> for ReportRowFields {
    fn from(value: FullStudyDto) -> Self {
        let FullStudyDto {
            parent:
                StudyDto {
                    protocolo,
                    fecha,
                    paciente,
                    tipo,
                    ..
                },
            ..
        } = value;
        ReportRowFields {
            protocolo,
            fecha,
            paciente,
            tipo,
        }
    }
}

#[component]
fn ReportTable(reports: Vec<FullStudyDto>) -> impl IntoView {
    view! {
        <div class="flex flex-col px-4 py-2 gap-4 max-w-2xl mx-auto border border-slate-500 bg-white rounded-lg shadow">
            <table>
                <thead>
                    <tr>
                        <th>Protocolo</th>
                        <th>Fecha</th>
                        <th>Paciente</th>
                        <th>Tipo</th>
                    </tr>
                </thead>
                <tbody>

                    {reports
                        .into_iter()
                        .rev()
                        .take(20)
                        .map(|node| {
                            let local_node = node.clone();
                            let table_node: ReportRowFields = node.into();
                            view! {
                                <tr>
                                    <td>
                                        <a href=format!(
                                            "/reports/{}",
                                            local_node.parent.id,
                                        )>{table_node.protocolo}</a>
                                    </td>
                                    <td>{format!("{:?}", table_node.fecha)}</td>
                                    <td>{table_node.paciente}</td>
                                    <td>
                                        <span class="inline-flex items-center gap-1.5">
                                            <StudyTypeIcon
                                                tipo=table_node.tipo.clone()
                                                class="size-4"
                                            />
                                            {format!("{:?}", table_node.tipo)}
                                        </span>
                                    </td>
                                </tr>
                            }
                        })
                        .collect_view()}

                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn CreateReport() -> impl IntoView {
    let create: ServerAction<CreateStudy> = ServerAction::<CreateStudy>::new();
    let tipo = RwSignal::new(StudyTypeDto::Citologia);
    #[allow(unused_variables)]
    let value: MappedSignal<Option<Result<(), ServerFnError>>> = create.value();
    view! {
        <ActionForm action=create>
            <div class="max-w-4xl mx-auto flex flex-col gap-6">
                // Needed if SuggestInput reads a Resource; harmless if it doesn't.
                <Transition fallback=|| view! { <p>"Cargando…"</p> }>
                    // Header fields on the left, type-specific fields on the right
                    // (one column on phones). Each side is a single grid child.
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <ReportHeader tipo=tipo />

                        // child fields: inputs inside these are named study[child][<Variant>][...]
                        {move || match tipo.get() {
                            StudyTypeDto::Histopatologia => view! { <Histopatologia /> }.into_any(),
                            StudyTypeDto::Citologia => view! { <Citologias /> }.into_any(),
                        }}
                    </div>
                </Transition>
                <button
                    class="max-w-40 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white font-semibold py-2 px-4 rounded"
                    type="submit"
                >
                    "Guardar"
                </button>
            </div>
        </ActionForm>
    }
}

#[component]
pub fn ReportHeader(tipo: RwSignal<StudyTypeDto>) -> impl IntoView {
    // set these when a suggestion is picked (see note below)
    let doctor_id = RwSignal::new(None::<i32>);
    let material_id = RwSignal::new(None::<i32>);
    let doctors = OnceResource::new(get_all_doctors());
    let materials = OnceResource::new(get_all_materials());
    view! {
        <Suspense fallback=|| {
            view! { <p>"Cargando…"</p> }
        }>
            {move || Suspend::new(async move {
                let doctors = doctors.await.unwrap();
                let materials = materials.await.unwrap();
                view! {
                    <div class="flex flex-col gap-4">
                        <label class="flex flex-col gap-1">
                            <span class="text-sm font-semibold text-slate-700">"Protocolo"</span>
                            <input
                                class="border border-slate-400 px-3 py-2 rounded-md w-full"
                                type="text"
                                id="protocolo"
                                name="study[parent][protocolo]"
                            />
                        </label>
                        <label class="flex flex-col gap-1">
                            <span class="text-sm font-semibold text-slate-700">"Fecha"</span>
                            <input
                                class="border border-slate-400 px-3 py-2 rounded-md w-full"
                                type="datetime-local"
                                id="fecha"
                                name="study[parent][fecha]"
                            />
                        </label>
                        <label class="flex flex-col gap-1">
                            <span class="text-sm font-semibold text-slate-700">"Paciente"</span>
                            <input
                                class="border border-slate-400 px-3 py-2 rounded-md w-full"
                                type="text"
                                id="paciente"
                                name="study[parent][paciente]"
                            />
                        </label>
                        <label class="flex flex-col gap-1">
                            <span class="text-sm font-semibold text-slate-700">"Doctor"</span>
                            <select
                                id="doctor-id"
                                class="border border-slate-400 px-3 py-2 rounded-md w-full"
                                name="study[parent][doctor_id]"
                                on:change:target=move |ev| {
                                    let v: i32 = ev
                                        .target()
                                        .value()
                                        .parse()
                                        .expect("Failed parsing doctor-id");
                                    doctor_id.set(Some(v));
                                }
                            >
                                {doctors
                                    .into_iter()
                                    .map(|doctor| {
                                        view! {
                                            <option value=doctor
                                                .clone()
                                                .id>{doctor.clone().name}</option>
                                        }
                                    })
                                    .collect_view()}
                            </select>
                        </label>
                        <label class="flex flex-col gap-1">
                            <span class="text-sm font-semibold text-slate-700">"Material"</span>
                            <select
                                class="border border-slate-400 px-3 py-2 rounded-md w-full"
                                id="material-id"
                                name="study[parent][material_id]"
                                on:change:target=move |ev| {
                                    let v: i32 = ev
                                        .target()
                                        .value()
                                        .parse()
                                        .expect("Failed parsing material-id");
                                    material_id.set(Some(v));
                                }
                            >
                                {materials
                                    .into_iter()
                                    .map(|material| {
                                        view! { <option value=material.id>{material.name}</option> }
                                    })
                                    .collect_view()}
                            </select>
                        </label>
                        <label class="flex flex-col gap-1">
                            <span class="text-sm font-semibold text-slate-700">"Tipo"</span>
                            <div class="flex items-center gap-2">
                                <select
                                    class="border border-slate-400 px-3 py-2 rounded-md w-full"
                                    id="tipo"
                                    name="study[parent][tipo]"
                                    on:change:target=move |ev| {
                                        if let Ok(t) = StudyTypeDto::from_str(
                                            &ev.target().value(),
                                        ) {
                                            tipo.set(t);
                                        }
                                    }
                                >
                                    <option value="Citologia">"Citopatologia"</option>
                                    <option value="Histopatologia">"Histopatologia"</option>
                                </select>
                                {move || {
                                    view! {
                                        <StudyTypeIcon tipo=tipo.get() class="size-5 shrink-0" />
                                    }
                                }}
                            </div>
                        </label>
                    </div>
                }
            })}

        </Suspense>
    }
}

// No <Form> in here: this renders inside CreateReport's <ActionForm>, and
// HTML forms can't be nested.
#[component]
pub fn Histopatologia() -> impl IntoView {
    // optional prefill from the URL query (?descripcion_macroscopica=...)
    query_map_to_field!(descripcion_macroscopica, diagnostico_histopatologico);

    view! {
        <div class="flex flex-col gap-4">
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Descripción macroscópica"</span>
                <SuggestInput
                    field=ReportFieldsDto::Histopatologia(HistoFields::DescripcionMacroscopica)
                    id="descripcion_macroscopica"
                    name="study[child][Histopatologia][descripcion_macroscopica]"
                    value=descripcion_macroscopica
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">
                    "Diagnóstico histopatológico"
                </span>
                <SuggestInput
                    field=ReportFieldsDto::Histopatologia(HistoFields::DiagnosticoHistopatologico)
                    id="diagnostico_histopatologico"
                    name="study[child][Histopatologia][diagnostico_histopatologico]"
                    value=diagnostico_histopatologico
                />
            </label>
        </div>
    }
}

#[component]
pub fn Citologias() -> impl IntoView {
    // optional prefill from the URL query
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
        <div class="flex flex-col gap-4">
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Microflora"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Microflora)
                    id="microflora"
                    name="study[child][Citologia][microflora]"
                    value=microflora
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Leucocitos"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Leucocitos)
                    id="leucocitos"
                    name="study[child][Citologia][leucocitos]"
                    value=leucocitos
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células pavimentosas"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasPavimentosas)
                    id="celulas_pavimentosas"
                    name="study[child][Citologia][celulas_pavimentosas]"
                    value=celulas_pavimentosas
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células metaplásicas"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasMetaplasicas)
                    id="celulas_metaplasicas"
                    name="study[child][Citologia][celulas_metaplasicas]"
                    value=celulas_metaplasicas
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Hematíes"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Hematies)
                    id="hematies"
                    name="study[child][Citologia][hematies]"
                    value=hematies
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células endocervicales"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasEndocervicales)
                    id="celulas_endocervicales"
                    name="study[child][Citologia][celulas_endocervicales]"
                    value=celulas_endocervicales
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Compatible con"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CompatibleCon)
                    id="compatible_con"
                    name="study[child][Citologia][compatible_con]"
                    value=compatible_con
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Sugerencia"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Sugerencia)
                    id="sugerencia"
                    name="study[child][Citologia][sugerencia]"
                    value=sugerencia
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Diagnóstico citológico"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::DiagCitologias)
                    id="diag_citologias"
                    name="study[child][Citologia][diag_citologias]"
                    value=diag_citologias
                />
            </label>
        </div>
    }
}
