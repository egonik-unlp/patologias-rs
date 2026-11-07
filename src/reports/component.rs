use leptos::prelude::*;

use crate::reports::server::get_reports;
#[component]
pub fn Reports() -> impl IntoView {
    let resource = OnceResource::new(get_reports());
    view! {
    {move || resource.get()}
    }
}
