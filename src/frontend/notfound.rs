use crate::frontend::icons::EmptyField;
use leptos::prelude::*;
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 px-2 py-4">
            <h1 class="font-semibold mx-auto text-3xl text-slate-700">"Pagina no encontrada"</h1>
            <EmptyField class="size-80 opacity-35 mx-auto px-4" />
            <a href="/">
                <button class=" text-xs rounded-3xl max-w-40 bg-hema-700 hover:bg-hema-800 active:bg-hema-900 active:scale-95 transition text-white py-4 px-2 min-h-3">
                    <span>"Volver al inicio"</span>
                </button>
            </a>
        </div>
    }
}
