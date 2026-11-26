//! Per-study-type form sections, rendered inside the report form.
use crate::{
    frontend::suggestions::SuggestInput,
    reports::dto::{CitoFields, CitologiaDto, HistoFields, HistopatologiaDto, ReportFieldsDto},
};
use leptos::prelude::*;

// No <Form> in here: this renders inside CreateReport's <ActionForm>, and
// HTML forms can't be nested.
#[component]
pub fn Histopatologia(#[prop(optional)] initial: Option<HistopatologiaDto>) -> impl IntoView {
    // optional prefill from the URL query (?descripcion_macroscopica=...)
    // query_map_to_field!(descripcion_macroscopica, diagnostico_histopatologico);
    let HistopatologiaDto {
        descripcion_macroscopica,
        diagnostico_histopatologico,
    } = initial.unwrap_or_default();

    view! {
        <div class="flex flex-col gap-4">
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">
                    "Descripción macroscópica"
                </span>
                <SuggestInput
                    field=ReportFieldsDto::Histopatologia(HistoFields::DescripcionMacroscopica)
                    id="descripcion_macroscopica"
                    name="study[child][Histopatologia][descripcion_macroscopica]"
                    value=move || descripcion_macroscopica.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">
                    "Diagnóstico histopatológico"
                </span>
                <SuggestInput
                    field=ReportFieldsDto::Histopatologia(HistoFields::DiagnosticoHistopatologico)
                    id="diagnostico_histopatologico"
                    name="study[child][Histopatologia][diagnostico_histopatologico]"
                    value=move || diagnostico_histopatologico.clone()
                />
            </label>
        </div>
    }
}

#[component]
pub fn Citologias(#[prop(optional)] initial: Option<CitologiaDto>) -> impl IntoView {
    let CitologiaDto {
        microflora,
        celulas_pavimentosas,
        celulas_metaplasicas,
        leucocitos,
        hematies,
        celulas_endocervicales,
        compatible_con,
        sugerencia,
        diag_citologias,
    } = initial.unwrap_or_default();
    view! {
        <div class="flex flex-col gap-4">
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Microflora"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Microflora)
                    id="microflora"
                    name="study[child][Citologia][microflora]"
                    value=move || microflora.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Leucocitos"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Leucocitos)
                    id="leucocitos"
                    name="study[child][Citologia][leucocitos]"
                    value=move || leucocitos.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células pavimentosas"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasPavimentosas)
                    id="celulas_pavimentosas"
                    name="study[child][Citologia][celulas_pavimentosas]"
                    value=move || celulas_pavimentosas.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células metaplásicas"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasMetaplasicas)
                    id="celulas_metaplasicas"
                    name="study[child][Citologia][celulas_metaplasicas]"
                    value=move || celulas_metaplasicas.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Hematíes"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Hematies)
                    id="hematies"
                    name="study[child][Citologia][hematies]"
                    value=move || hematies.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Células endocervicales"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CelulasEndocervicales)
                    id="celulas_endocervicales"
                    name="study[child][Citologia][celulas_endocervicales]"
                    value=move || celulas_endocervicales.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Compatible con"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::CompatibleCon)
                    id="compatible_con"
                    name="study[child][Citologia][compatible_con]"
                    value=move || compatible_con.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Sugerencia"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::Sugerencia)
                    id="sugerencia"
                    name="study[child][Citologia][sugerencia]"
                    value=move || sugerencia.clone()
                />
            </label>
            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-slate-700">"Diagnóstico citológico"</span>
                <SuggestInput
                    field=ReportFieldsDto::Citologia(CitoFields::DiagCitologias)
                    id="diag_citologias"
                    name="study[child][Citologia][diag_citologias]"
                    value=move || diag_citologias.clone()
                />
            </label>
        </div>
    }
}
