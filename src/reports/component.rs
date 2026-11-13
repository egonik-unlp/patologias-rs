use crate::{
    entities::dto::DoctorFieldsDto,
    frontend::suggestions::SuggestInput,
    reports::{
        dto::{FullStudyDto, ReportFieldsDto, StudyDto},
        server::read_all_and_stringify,
    },
};
use leptos::prelude::*;
use leptos_router::{components::Form, hooks::use_query_map};

#[component]
pub fn Reports() -> impl IntoView {
    let resource = OnceResource::new(read_all_and_stringify());
    view! {
        <h1>
            AAAAAAAAAAAAAAAA
        </h1>
        // Resources must be read inside <Suspense>: otherwise the server renders
        // the fallback while the hydrating client already has the data, and the
        // two HTML trees don't match.
        <Suspense fallback=|| view! { <div> "Floripondio" </div> }>
            {move || resource.get().map(|rec| match rec {
                Ok(val) => view! {
                    <div> {val.into_iter().rev().take(20).map(|val| view! { <p> { format!("{:?}", val)} </p> }).collect_view()} </div>
                }.into_any(),
                Err(e) => view! { <div> {e.to_string()} </div> }.into_any(),
            })}
        </Suspense>
    }
}

#[component]
pub fn FormExample() -> impl IntoView {
    // reactive access to URL query
    let query = use_query_map();
    let protocolo = move || query.read().get("number").unwrap_or_default();
    let fecha = move || query.read().get("select").unwrap_or_default();
    let paciente = move || query.read().get("paciente").unwrap_or_default();
    let doctor = move || query.read().get("name").unwrap_or_default();
    let material = move || query.read().get("name").unwrap_or_default();

    view! {
        // read out the URL query strings
        <table>
            <tr>
                <td><code>"Protocolo"</code></td>
                <td>{protocolo}</td>
            </tr>
            <tr>
                <td><code>"Fecha"</code></td>
                <td>{fecha}</td>
            </tr>
            <tr>
                <td><code>"Paciente"</code></td>
                <td>{paciente}</td>
            </tr>
            <tr>
                <td><code>"Doctor"</code></td>
                <td>{doctor}</td>
            </tr>
            <tr>
                <td><code>"Material"</code></td>
                <td>{material}</td>
            </tr>
        </table>
        // <Form/> will navigate whenever submitted
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="protocolo" value=protocolo/>
            <input type="text" name="fecha" value=fecha/>
            <input type="text" name="paciente" value=paciente/>
            <input type="text" name="doctor" value=doctor/>
            <SuggestInput field=DoctorFieldsDto::Name id="doctor" value = doctor />

            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
        // This <Form/> uses some JavaScript to submit
        // on every input
    }
}
#[component]
pub fn NewReport() -> impl IntoView {}
