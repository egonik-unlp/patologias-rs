use crate::entities::server::{get_all_doctors, get_all_materials};
use leptos::prelude::*;
#[component]
fn Doctors() -> impl IntoView {
    view! {
        <div class="flex flex-col px-4 py-2 gap-4 mt-10 max-w-96 mx-auto border border-slate-500 bg-white rounded-lg shadow">
            <Await future=get_all_doctors() let:data>
                {data
                    .clone()
                    .unwrap()
                    .iter()
                    .take(20)
                    .map(|doctor| view! { <p>{doctor.name.clone()}</p> })
                    .collect_view()}
            </Await>
        </div>
    }
}

#[component]
fn Materials() -> impl IntoView {
    view! {
        <div class="flex flex-col px-4 py-2 gap-4 mt-10 max-w-96 mx-auto border border-slate-500 bg-white rounded-lg shadow">
            <Await future=get_all_materials() let:data>
                {data
                    .clone()
                    .unwrap()
                    .iter()
                    .take(20)
                    .map(|material| view! { <p>{material.name.clone()}</p> })
                    .collect_view()}
            </Await>
        </div>
    }
}

#[component]
pub fn Entities() -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row">
            <Doctors />
            <Materials />
        </div>
    }
}

#[component]
fn CreateDoctor() -> impl IntoView {}

#[component]
fn CreateMaterial() -> impl IntoView {}
