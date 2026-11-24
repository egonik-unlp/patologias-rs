use crate::{
    entities::server::{get_all_doctors, get_all_materials},
    frontend::{
        icons::{IconCitologia, IconCopy, IconEdit, IconHistopatologia, IconTrash},
        suggestions::SuggestInput,
    },
    reports::{
        dto::{
            CitoFields, CitologiaDto, FullStudyDto, HistoFields, HistopatologiaDto,
            ReportFieldsDto, StudyDetailDto, StudyDto, StudyTypeDto,
        },
        server::{read_all, read_report, CreateStudy, DeleteStudy, UpdateStudy},
    },
};
use std::str::FromStr;

use anyhow::Result;
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
    let delete_action: ServerAction<DeleteStudy> = expect_context();
    //TODO: Hacer mostrar el resultado o algo, si no esto vuela
    #[allow(unused_variables)]
    let value: MappedSignal<Option<Result<(), _>>> = delete_action.value();
    let delete_requested = RwSignal::new(false);
    let FullStudyDto { parent, child } = report.clone();
    view! {
        <div class="flex flex-col px-4 py-4 gap-4  max-w-2xl mx-auto border border-slate-500 bg-white rounded-lg shadow">
            <ShowParent parent />
            <ShowChild child />
            <Show
                when=move || delete_requested.get()
                fallback=move || {
                    view! {
                        <button
                            on:click=move |_| { delete_requested.set(true) }

                            class=" flex gap-0.5 rounded-3xl max-w-60 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white  py-2 px-4  "
                        ></button>
                    }
                }
            >

                <div class="flex gap-1">
                    <button
                        on:click=move |_| {
                            delete_action
                                .dispatch(DeleteStudy {
                                    study_id: report.parent.id,
                                });
                        }
                        class=" text-xs rounded-3xl max-w-40 bg-red-700 hover:bg-red-800 active:bg-red-900 active:scale-95 transition text-white  py-1 px-2  "
                    >
                        <span>"Ok"</span>
                    </button>
                    <button
                        on:click=move |_| delete_requested.set(false)
                        class=" text-xs rounded-3xl max-w-40 bg-red-700 hover:bg-red-800 active:bg-red-900 active:scale-95 transition text-white  py-1 px-2  "
                    >
                        <span>"Cancelar"</span>
                    </button>

                </div>

            </Show>
        </div>
    }
}

#[component]
fn ReportFields(#[prop(optional, into)] initial: Option<FullStudyDto>) -> impl IntoView {
    let (parent, child) = initial.map(|val| (val.parent, val.child)).unzip();
    let (cito, histo) = child
        .map(|val| match val {
            StudyDetailDto::Citologia(citologia_dto) => (Some(citologia_dto), None),
            StudyDetailDto::Histopatologia(histopatologia_dto) => (None, Some(histopatologia_dto)),
        })
        .unwrap_or_default();
    let tipo = RwSignal::new(
        parent
            .clone()
            .map_or(StudyTypeDto::Citologia, |val| val.tipo),
    );
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <ReportHeader tipo=tipo initial=parent.clone().unwrap_or_default() />

            {move || match tipo.get() {
                StudyTypeDto::Histopatologia => {
                    view! { <Histopatologia initial=histo.clone().unwrap_or_default() /> }
                        .into_any()
                }
                StudyTypeDto::Citologia => {
                    view! { <Citologias initial=cito.clone().unwrap_or_default() /> }.into_any()
                }
            }}
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
    let delete_action: ServerAction<DeleteStudy> = expect_context();
    let resource = Resource::new(move || delete_action.version().get(), |_| read_all());
    view! {
        <h1 class="font-semibold text-3xl text-slate-700 mx-auto">Citologia Reports Table</h1>
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
fn ReportButtons(report_id: i32, just_selected: RwSignal<i32>) -> impl IntoView {
    let delete_action: ServerAction<DeleteStudy> = expect_context();
    let delete_requested = RwSignal::new(false);
    view! {
        <Show
            when=move || { delete_requested.get() && just_selected.get().eq(&report_id) }
            fallback=move || {
                view! {
                    <div class="border-[0.5] border-slate-500 inline-flex text-xs">
                        <button
                            on:click=move |_| {
                                delete_requested.set(true);
                                just_selected.set(report_id)
                            }
                            class=" flex gap-0.5 rounded-3xl max-w-60 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white  py-2 px-4  "
                        >
                            <span>"Delete"</span>
                            <IconTrash />
                        </button>
                        <a href=format!("/reports/{report_id}/edit")>
                            <button class=" flex gap-0.5 rounded-3xl  max-w-40 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white  py-2 px-4  ">
                                <span>"Edit"</span>
                                <IconEdit />
                            </button>
                        </a>
                        <button class=" flex gap-0.5 rounded-3xl  max-w-40 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white  py-2 px-4   ">
                            <span>"Copy"</span>
                            <IconCopy />
                        </button>
                    </div>
                }
            }
        >
            <div class="flex gap-1">
                <button
                    on:click=move |_| {
                        delete_action.dispatch(DeleteStudy { study_id: report_id });
                    }
                    class=" text-xs rounded-3xl max-w-40 bg-red-700 hover:bg-red-800 active:bg-red-900 active:scale-95 transition text-white  py-1 px-2  "
                >
                    <span>"Ok"</span>
                </button>
                <button
                    on:click=move |_| delete_requested.set(false)
                    class=" text-xs rounded-3xl max-w-40 bg-red-700 hover:bg-red-800 active:bg-red-900 active:scale-95 transition text-white  py-1 px-2  "
                >
                    <span>"Cancelar"</span>
                </button>
            </div>
        </Show>
    }
}

#[component]
fn ReportTable(reports: Vec<FullStudyDto>) -> impl IntoView {
    let just_selected = RwSignal::new(0);
    view! {
        <div class="flex flex-col px-4 gap-4 max-w-3xl mx-auto border border-slate-500 bg-white rounded-lg shadow">
            <table>
                <thead>
                    <tr>
                        <th>Protocolo</th>
                        <th>Fecha</th>
                        <th>Paciente</th>
                        <th>Tipo</th>
                        <th>Acciones</th>
                    </tr>
                </thead>
                <tbody>

                    {reports
                        .into_iter()
                        .rev()
                        .take(20)
                        .map(|report: FullStudyDto| {
                            let local_node = report.clone();
                            let table_node: ReportRowFields = report.into();
                            // TODO: Esto está horrible, ver si se puede resolver mas elegantemente
                            //
                            view! {
                                <tr id=format!("report-row-{}", local_node.parent.id)>
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
                                    <td>
                                        <ReportButtons
                                            report_id=local_node.parent.id
                                            just_selected
                                        />
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

// Hay que asociar los params a la server functon con el name
#[component]
pub fn UpdateReport() -> impl IntoView {
    use leptos::logging::log;
    let params = use_params_map();
    let report_id = move || {
        params
            .read()
            .get("id")
            .and_then(|val| val.parse::<i32>().ok())
    };
    let resource = Resource::new(report_id, |report_id| async move {
        log!("Report id: {:?}", report_id);
        match report_id {
            Some(id) => read_report(id).await,
            None => Err(ServerFnError::new("Malformed report id")),
        }
    });
    let update: ServerAction<UpdateStudy> = ServerAction::new();
    let value: MappedSignal<Option<Result<FullStudyDto, ServerFnError>>> = update.value();
    view! {
        <ActionForm action=update>
            <div class="max-w-4xl mx-auto flex flex-col gap-6">
                <Suspense fallback=|| view! { <p>"Cargando…"</p> }>
                    // TODO: Esto tiene que adoptar el mismo formato que el resto de errores
                    <ErrorBoundary fallback=|errors| {
                        view! {
                            <div class="error">
                                <p>"Not a number! Errors: "</p>
                                <ul>
                                    {move || {
                                        errors
                                            .get()
                                            .into_iter()
                                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                            .collect::<Vec<_>>()
                                    }}
                                </ul>
                            </div>
                        }
                    }>
                        {move || Suspend::new(async move {
                            let initial = resource.await;
                            log!("Initial values rendered: {:?}", initial);
                            view! {
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    {initial.map(|val| view! { <ReportFields initial=val /> })}

                                </div>
                            }
                        })}
                    </ErrorBoundary>
                </Suspense>
                <button
                    class="max-w-40 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white font-semibold py-2 px-4 rounded"
                    type="submit"
                >
                    "Guardar"
                </button>
                {move || match value.get() {
                    None => view! {}.into_any(),
                    Some(Ok(v)) => {
                        view! {
                            <p class=" rounded-md border px-3 py-2 text-sm bg-green-50 border-green-200 text-green-700">
                                {format!("Report {} updated", v.parent.protocolo)}
                            </p>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <p class=" rounded-md border px-3 py-2 text-sm bg-red-50 border-red-200 text-red-700">
                                {format!("Error creating report {}", e)}
                            </p>
                        }
                            .into_any()
                    }
                }}
            </div>
        </ActionForm>
    }
}

#[component]
pub fn CreateReport() -> impl IntoView {
    let create: ServerAction<CreateStudy> = ServerAction::<CreateStudy>::new();
    let value: MappedSignal<Option<Result<FullStudyDto, ServerFnError>>> = create.value();
    view! {
        <ActionForm action=create>
            <div class="max-w-4xl mx-auto flex flex-col gap-6">
                <Transition fallback=|| view! { <p>"Cargando…"</p> }>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <ReportFields />
                    </div>
                </Transition>
                <button
                    class="max-w-40 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white font-semibold py-2 px-4 rounded"
                    type="submit"
                >
                    "Guardar"
                </button>
                {move || match value.get() {
                    None => view! {}.into_any(),
                    Some(Ok(v)) => {
                        view! {
                            <p class=" rounded-md border px-3 py-2 text-sm bg-green-50 border-green-200 text-green-700">
                                {format!("Report {} created", v.parent.protocolo)}
                            </p>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <p class=" rounded-md border px-3 py-2 text-sm bg-red-50 border-red-200 text-red-700">
                                {format!("Error creating report {}", e)}
                            </p>
                        }
                            .into_any()
                    }
                }}
            </div>
        </ActionForm>
    }
}

#[component]
pub fn ReportHeader(
    tipo: RwSignal<StudyTypeDto>,
    #[prop(optional)] initial: Option<StudyDto>,
) -> impl IntoView {
    // set these when a suggestion is picked (see note below)
    let StudyDto {
        protocolo,
        fecha,
        paciente,
        id,
        ..
    } = initial.unwrap_or_default();
    let doctor_id = RwSignal::new(None::<i32>);
    let material_id = RwSignal::new(None::<i32>);
    let doctors = OnceResource::new(get_all_doctors());
    let materials = OnceResource::new(get_all_materials());
    view! {
        <Suspense fallback=|| {
            view! { <p>"Cargando…"</p> }
        }>
            {move || {
                let protocolo = protocolo.clone();
                let paciente = paciente.clone();
                Suspend::new(async move {
                    let doctors = doctors.await.unwrap();
                    let materials = materials.await.unwrap();
                    leptos::logging::log!(
                        "protocolo: {}, fecha: {} paciente: {} ", protocolo, fecha, paciente
                    );
                    view! {
                        <div class="flex flex-col gap-4">
                            <label class="flex flex-col gap-1">
                                <span class="text-sm font-semibold text-slate-700">
                                    "Protocolo"
                                </span>
                                <input
                                    class="border border-slate-400 px-3 py-2 rounded-md w-full"
                                    type="text"
                                    id="protocolo"
                                    value=protocolo
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
                                    value=fecha.format("%Y-%m-%dT%H:%M").to_string()
                                />
                            </label>
                            <label class="flex flex-col gap-1">
                                <span class="text-sm font-semibold text-slate-700">"Paciente"</span>
                                <input
                                    class="border border-slate-400 px-3 py-2 rounded-md w-full"
                                    type="text"
                                    id="paciente"
                                    value=paciente
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
                            <input type="hidden" name="study_id" value=id />
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
                })
            }}

        </Suspense>
    }
}

// No <Form> in here: this renders inside CreateReport's <ActionForm>, and
// HTML forms can't be nested.
#[component]
pub fn Histopatologia(#[prop(optional)] initial: Option<HistopatologiaDto>) -> impl IntoView {
    // optional prefill from the URL query (?descripcion_macroscopica=...)
    // query_map_to_field!(descripcion_macroscopica, diagnostico_histopatologico);
    let HistopatologiaDto {
        descripcion_macroscopica,
        diagnostico_histopatologico,
    } = initial.unwrap_or_default();

    view! {
        <div class="flex flex-col gap-4">
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">
                    "Descripción macroscópica"
                </span>
                <SuggestInput
                    field=ReportFieldsDto::Histopatologia(HistoFields::DescripcionMacroscopica)
                    id="descripcion_macroscopica"
                    name="study[child][Histopatologia][descripcion_macroscopica]"
                    value=move || descripcion_macroscopica.clone()
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
                    value=move || diagnostico_histopatologico.clone()
                />
            </label>
        </div>
    }
}

#[component]
pub fn Citologias(#[prop(optional)] initial: Option<CitologiaDto>) -> impl IntoView {
    let CitologiaDto {
        microflora,
        celulas_pavimentosas,
        celulas_metaplasicas,
        leucocitos,
        hematies,
        celulas_endocervicales,
        compatible_con,
        sugerencia,
        diag_citologias,
    } = initial.unwrap_or_default();
    view! {
        <div class="flex flex-col gap-4">
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Microflora"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Microflora)
                    id="microflora"
                    name="study[child][Citologia][microflora]"
                    value=move || microflora.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Leucocitos"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Leucocitos)
                    id="leucocitos"
                    name="study[child][Citologia][leucocitos]"
                    value=move || leucocitos.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células pavimentosas"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasPavimentosas)
                    id="celulas_pavimentosas"
                    name="study[child][Citologia][celulas_pavimentosas]"
                    value=move || celulas_pavimentosas.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células metaplásicas"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasMetaplasicas)
                    id="celulas_metaplasicas"
                    name="study[child][Citologia][celulas_metaplasicas]"
                    value=move || celulas_metaplasicas.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Hematíes"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Hematies)
                    id="hematies"
                    name="study[child][Citologia][hematies]"
                    value=move || hematies.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células endocervicales"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasEndocervicales)
                    id="celulas_endocervicales"
                    name="study[child][Citologia][celulas_endocervicales]"
                    value=move || celulas_endocervicales.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Compatible con"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CompatibleCon)
                    id="compatible_con"
                    name="study[child][Citologia][compatible_con]"
                    value=move || compatible_con.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Sugerencia"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Sugerencia)
                    id="sugerencia"
                    name="study[child][Citologia][sugerencia]"
                    value=move || sugerencia.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Diagnóstico citológico"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::DiagCitologias)
                    id="diag_citologias"
                    name="study[child][Citologia][diag_citologias]"
                    value=move || diag_citologias.clone()
                />
            </label>
        </div>
    }
}
