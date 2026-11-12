use crate::reports::{
    dto::{FullStudyDto, ReportFieldsDto, StudyDto},
    server::{get_suggestions_for_field, read_all_and_stringify},
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
pub fn SuggestInput(
    field: ReportFieldsDto,
    #[prop(into)] id: String,
    /// Current value (e.g. from the URL query). The form reads the edited
    /// value through `name` on submit.
    value: impl Fn() -> String + Send + Sync + 'static,
) -> impl IntoView {
    let list_id = format!("{id}-options");
    let suggestions = OnceResource::new(get_suggestions_for_field(field));

    view! {
        <input
            type="text"
            id=id.clone()
            name=id
            list=list_id.clone()
            autocomplete="off"
            value=value
        />
        <datalist id=list_id>
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    suggestions
                        .await
                        .unwrap_or_default()
                        .into_iter()
                        .map(|v| view! { <option value=v/> })
                        .collect_view()
                })}
            </Suspense>
        </datalist>
    }
}
#[component]
pub fn FormExample() -> impl IntoView {
    // reactive access to URL query
    let query = use_query_map();
    let microflora = move || query.read().get("name").unwrap_or_default();
    let number = move || query.read().get("number").unwrap_or_default();
    let select = move || query.read().get("select").unwrap_or_default();

    view! {
        // read out the URL query strings
        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{microflora}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number}</td>
            </tr>
            <tr>
                <td><code>"select"</code></td>
                <td>{select}</td>
            </tr>
        </table>
        // <Form/> will navigate whenever submitted
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="microflora" value=microflora/>
            <SuggestInput field=ReportFieldsDto::Citologia(crate::reports::dto::CitoFields::Microflora) id="microflora" value = microflora />
            <input type="text" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
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
