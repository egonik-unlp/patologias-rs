//! Single report page (`/reports/:id`).
use crate::{
    frontend::{
        icons::{IconEdit, IconTrash},
        notfound::NotFound,
    },
    reports::{
        dto::{CitologiaDto, FullStudyDto, HistopatologiaDto, StudyDetailDto, StudyDto},
        server::{read_report, DeleteStudy},
    },
};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use super::StudyTypeIcon;

#[component]
pub fn Report() -> impl IntoView {
    let params = use_params_map();
    // Reactive: the router reuses this component when going /reports/1 -> /reports/2,
    // so re-running on id change is what gives each report a fresh ReportView (and fresh signals).
    let id = move || params.read().get("id").and_then(|s| s.parse::<i32>().ok());
    move || {
        id().map(|id| {
            view! {
                <Await future=read_report(id) let:data>
                    // TODO: swap the error text for the NotFound page once it exists.
                    {match data.clone() {
                        Ok(report) => view! { <ReportView report /> }.into_any(),
                        Err(_) => view! { <NotFound /> }.into_any(),
                    }}
                </Await>
            }
        })
    }
}

#[component]
fn ReportView(report: FullStudyDto) -> impl IntoView {
    let delete_action: ServerAction<DeleteStudy> = expect_context();
    let value: MappedSignal<Option<Result<(), _>>> = delete_action.value();
    let delete_requested = RwSignal::new(false);
    // The action lives in app context, so `value` still holds the previous delete's result
    // when another report is opened. Only trust it if *this* view sent a delete.
    let delete_sent = RwSignal::new(false);
    let finished = move || delete_sent.get() && !delete_action.pending().get();
    let deleted = move || finished() && matches!(value.get(), Some(Ok(())));
    let delete_error = move || {
        if !finished() {
            return None;
        }
        match value.get() {
            Some(Err(e)) => Some(e.to_string()),
            _ => None,
        }
    };
    let FullStudyDto { parent, child } = report.clone();
    view! {
        <div class="flex flex-col px-4 py-4 gap-4  max-w-2xl mx-auto border border-slate-500 bg-white rounded-lg shadow">
            <ShowParent parent />
            <ShowChild child />
            <Show
                when=deleted
                fallback=move || {
                    view! {
                        {move || {
                            delete_error()
                                .map(|e| view! { <div>{format!("No se pudo borrar: {e}")}</div> })
                        }}
                        <Show
                            when=move || delete_requested.get()
                            fallback=move || {
                                view! {
                                    <div class="flex gap-0.5">
                                        <button
                                            on:click=move |_| delete_requested.set(true)
                                            class="  rounded-3xl max-w-60 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white  py-2 px-4  "
                                        >
                                            <span>"Delete"</span>
                                            <IconTrash />
                                        </button>
                                        <a href=format!("/reports/{}/edit", report.parent.id)>
                                            <button
                                                on:click=move |_| delete_requested.set(true)
                                                class=" flex gap-0.5 rounded-3xl max-w-60 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white  py-2 px-4  "
                                            >
                                                <span>"Edit"</span>
                                                <IconEdit />
                                            </button>
                                        </a>
                                    </div>
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
                                        delete_sent.set(true);
                                        delete_requested.set(false);
                                    }
                                    class=" text-xs rounded-3xl max-w-40 bg-red-700 hover:bg-red-800 active:bg-red-900 active:scale-95 transition text-white  py-1 px-2  "
                                >
                                    <span>"Ok"</span>
                                </button>
                                <button
                                    on:click=move |_| delete_requested.set(false)
                                    class=" text-xs rounded-3xl max-w-40 bg-red-700 hover:bg-red-800 active:bg-red-900 active:scale-95 transition text-white  py-4 px-2 min-h-3"
                                >
                                    <span>"Cancelar"</span>
                                </button>
                            </div>
                        </Show>
                    }
                }
            >
                <div>"Borradooooo"</div>
            </Show>
        </div>
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
