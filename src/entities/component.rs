use crate::entities::server::{get_all_doctors, get_all_materials};
use leptos::prelude::*;
#[component]
fn Doctors() -> impl IntoView {
    view! {
        <div>
            <Await future=get_all_doctors() let:data>
                {data
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|doctor| view! { <p>{doctor.name.clone()}</p> })
                    .collect_view()}
            </Await>
        </div>
    }
}

#[component]
fn Materials() -> impl IntoView {
    view! {
        <div>
            <Await future=get_all_materials() let:data>
                {data
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|material| view! { <p>{material.name.clone()}</p> })
                    .collect_view()}
            </Await>
        </div>
    }
}

#[component]
pub fn Entities() -> impl IntoView {
    view! {
        <div class="flex">
            <Doctors />
            <Materials />
        </div>
    }
}

#[component]
fn CreateDoctor() -> impl IntoView {}

#[component]
fn CreateMaterial() -> impl IntoView {}
