//! Create / update report forms (`/reports/new`, `/reports/:id/edit`).
use crate::{
    entities::server::{get_all_doctors, get_all_materials},
    reports::{
        dto::{FullStudyDto, StudyDetailDto, StudyDto, StudyTypeDto},
        server::{read_report, CreateStudy, UpdateStudy},
    },
};
use std::str::FromStr;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use super::{
    sections::{Citologias, Histopatologia},
    StudyTypeIcon,
};

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
