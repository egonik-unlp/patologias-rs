use crate::reports::{
    dto::{FullStudyDto, StudyDto},
    server::read_all_and_stringify,
};
use leptos::prelude::*;

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
pub fn NewReport() -> impl IntoView {}
