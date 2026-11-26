//! Report list page (`/reports`).
use crate::{
    frontend::{
        icons::{IconCopy, IconEdit, IconTrash},
        pager::Pager,
    },
    reports::{
        dto::{FullStudyDto, PaginatedResponseDto, StudyDto, StudyTypeDto},
        server::{read_report_page, DeleteStudy},
        LIMIT,
    },
};
use chrono::NaiveDateTime;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use super::StudyTypeIcon;

#[component]
pub fn Reports() -> impl IntoView {
    let delete_action: ServerAction<DeleteStudy> = expect_context();
    let params = use_query_map();
    let page = move || {
        params
            .read()
            .get("page")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(1)
    };
    let resource: Resource<Result<PaginatedResponseDto, _>> = Resource::new(
        move || (page(), delete_action.version().get()),
        |(page, _)| read_report_page(page),
    );
    view! {
        <div class="flex flex-col items-center gap-4">
            <h1 class="font-semibold text-3xl text-slate-700 mx-auto">Citologia Reports Table</h1>
            <Suspense fallback=|| {
                view! { <div>"Loading..."</div> }
            }>
                {move || {
                    resource
                        .get()
                        .map(|rec| match rec {
                            Ok(reports) => {
                                view! {
                                    <ReportTable reports=reports.reports />
                                    <Pager
                                        current=page()
                                        total_pages=reports.total.div_ceil(LIMIT)
                                        base_url="/reports"
                                    />
                                }
                                    .into_any()
                            }
                            Err(e) => view! { <div>{e.to_string()}</div> }.into_any(),
                        })
                }}

            </Suspense>
        </div>
    }
}

#[component]
fn ReportTable(reports: Vec<StudyDto>) -> impl IntoView {
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
                        .map(|report: StudyDto| {
                            let local_node = report.clone();
                            let table_node: ReportRowFields = report.into();
                            // TODO: Esto está horrible, ver si se puede resolver mas elegantemente
                            //
                            view! {
                                <tr>
                                    <td>
                                        <a href=format!(
                                            "/reports/{}",
                                            local_node.id,
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
                                        <ReportButtons report_id=local_node.id just_selected />
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

#[derive(Debug, Clone)]
struct ReportRowFields {
    pub protocolo: String,
    pub fecha: NaiveDateTime,
    pub paciente: String,
    pub tipo: StudyTypeDto,
}
impl From<StudyDto> for ReportRowFields {
    fn from(value: StudyDto) -> Self {
        let StudyDto {
            protocolo,
            fecha,
            paciente,
            tipo,
            ..
        } = value;
        Self {
            protocolo,
            fecha,
            paciente,
            tipo,
        }
    }
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
