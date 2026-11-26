use leptos::prelude::*;

/// `< current/total_pages >` links. `base_url` is the page path, e.g. `/reports`.
#[component]
pub fn Pager(current: i64, total_pages: i64, base_url: &'static str) -> impl IntoView {
    let next_url = format!("{base_url}?page={}", current + 1);
    let prev_url = format!("{base_url}?page={}", (current - 1).max(1));
    view! {
        <div class="flex mx-auto text-lg text-slate-700 px-2 py-2 gap-1">
            <a href=prev_url>"<"</a>
            <span>{format!("{}/{}", current, total_pages)}</span>
            <span></span>
            <a href=next_url>">"</a>
        </div>
    }
}
