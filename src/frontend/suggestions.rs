use std::future::Future;

use crate::reports::dto::ReportFieldsDto;
use leptos::prelude::*;

pub trait Suggestions {
    fn get_suggestions(
        &self,
    ) -> impl Future<Output = Result<Vec<String>, ServerFnError>> + Send + 'static;
}

#[component]
pub fn SuggestInput(
    field: impl Suggestions,
    #[prop(into)] id: String,
    /// Current value (e.g. from the URL query). The form reads the edited
    /// value through `name` on submit.
    value: impl Fn() -> String + Send + Sync + 'static,
) -> impl IntoView {
    let list_id = format!("{id}-options");
    let suggestions = OnceResource::new(field.get_suggestions());

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
