//! Brand mark and UI icon set. See `branding/README.md`.
//!
//! Every icon is decorative (`aria-hidden`): it always sits next to text that names it. Stroke icons
//! draw with `currentColor`, so color them with `text-*` and size them with `size-*` on `class`.
//!
//! IconHouse, IconFilePlus, IconFiles, IconUserPlus, IconLogOut, IconSearch, IconPrint, IconEdit,
//! IconTrash, IconCopy and IconArrowLeft use path data from Lucide (lucide-static v0.460.0, https://lucide.dev), under this license:
//!
//! ISC License
//!
//! Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2022 as part of Feather (MIT).
//! All other copyright (c) for Lucide are held by Lucide Contributors 2022.
//!
//! Permission to use, copy, modify, and/or distribute this software for any purpose with or without
//! fee is hereby granted, provided that the above copyright notice and this permission notice appear
//! in all copies.
//!
//! THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
//! SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
//! AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
//! WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
//! NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
//! OF THIS SOFTWARE.
//!
//! IconSpecimenJar, IconCitologia, IconHistopatologia, BrandMark and EmptyField are original to this
//! project.

use leptos::prelude::*;

/// Brand mark: a microscope field of view (hematoxylin disc) holding one squamous cell (eosin
/// cytoplasm, hematoxylin nucleus). Same geometry as `branding/mark.svg`: change both together.
#[component]
pub fn BrandMark(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" aria-hidden="true" class=class>
            <circle cx="16" cy="16" r="15.5" class="fill-hema-900" />
            <path
                d="M9 12.2 L15 7.6 L22 9 L25 15.4 L21.6 22.8 L13.6 24.4 L8 19.2 Z"
                class="fill-eosin stroke-eosin"
                stroke-width="2.4"
                stroke-linejoin="round"
            />
            <circle cx="16.6" cy="16" r="3.1" class="fill-hema-900" />
        </svg>
    }
}

/// 404 illustration: the brand mark's field of view with the cell missing. Only a dashed ghost of
/// the cell is left, plus a few specks and the eyepiece's measuring scale. Colors are fixed (it's
/// art, not an icon); size it with `size-*` or `w-*` on `class`. Same geometry as
/// `branding/empty-field.svg`: change both together.
#[component]
pub fn EmptyField(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 160" aria-hidden="true" class=class>
            <circle cx="80" cy="80" r="78" class="fill-hema-900" />
            <circle cx="80" cy="80" r="66" class="fill-hema-50" />
            <path
                d="M50 128H110M50 122V128M56 125V128M62 125V128M68 125V128M74 125V128M80 122V128M86 125V128M92 125V128M98 125V128M104 125V128M110 122V128"
                fill="none"
                stroke-width="1.5"
                stroke-linecap="round"
                class="stroke-hema-300"
            />
            <g class="fill-hema-300">
                <circle cx="40" cy="104" r="2" />
                <circle cx="124" cy="100" r="1.5" />
                <circle cx="118" cy="42" r="2.5" />
                <circle cx="38" cy="58" r="1.5" />
                <circle cx="58" cy="34" r="1.5" />
            </g>
            <g
                stroke-width="2.5"
                stroke-linejoin="round"
                stroke-linecap="round"
                stroke-dasharray="5 6"
                class="stroke-hema-400"
            >
                <path
                    d="M53.4 65.6 L76.2 48.1 L102.8 53.4 L114.2 77.7 L101.3 105.8 L70.9 111.9 L49.6 92.2 Z"
                    class="fill-hema-100"
                />
                <circle cx="82.3" cy="80" r="11.8" fill="none" />
            </g>
        </svg>
    }
}

/// Generates a 24px-grid, 2px-stroke icon component that draws with `currentColor`.
macro_rules! stroke_icon {
    ($(#[$meta:meta])* $name:ident { $($shapes:tt)* }) => {
        $(#[$meta])*
        #[component]
        pub fn $name(#[prop(into, optional)] class: String) -> impl IntoView {
            view! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                    class=class
                >
                    $($shapes)*
                </svg>
            }
        }
    };
}

stroke_icon! {
    /// Lucide `house`.
    IconHouse {
        <path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8" />
        <path d="M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
    }
}

stroke_icon! {
    /// Lucide `file-plus`.
    IconFilePlus {
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
        <path d="M14 2v4a2 2 0 0 0 2 2h4" />
        <path d="M9 15h6" />
        <path d="M12 18v-6" />
    }
}

stroke_icon! {
    /// Lucide `files`.
    IconFiles {
        <path d="M20 7h-3a2 2 0 0 1-2-2V2" />
        <path d="M9 18a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h7l4 4v10a2 2 0 0 1-2 2Z" />
        <path d="M3 7.6v12.8A1.6 1.6 0 0 0 4.6 22h9.8" />
    }
}

stroke_icon! {
    /// Lucide `user-plus`.
    IconUserPlus {
        <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
        <circle cx="9" cy="7" r="4" />
        <line x1="19" x2="19" y1="8" y2="14" />
        <line x1="22" x2="16" y1="11" y2="11" />
    }
}

stroke_icon! {
    /// Lucide `log-out`.
    IconLogOut {
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
        <polyline points="16 17 21 12 16 7" />
        <line x1="21" x2="9" y1="12" y2="12" />
    }
}

stroke_icon! {
    /// Lucide `search`.
    IconSearch {
        <circle cx="11" cy="11" r="8" />
        <path d="m21 21-4.3-4.3" />
    }
}

stroke_icon! {
    /// Lucide `printer`.
    IconPrint {
        <path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" />
        <path d="M6 9V3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v6" />
        <rect x="6" y="14" width="12" height="8" rx="1" />
    }
}

stroke_icon! {
    /// Lucide `pencil`.
    IconEdit {
        <path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z" />
        <path d="m15 5 4 4" />
    }
}

stroke_icon! {
    /// Lucide `trash-2`.
    IconTrash {
        <path d="M3 6h18" />
        <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
        <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
        <line x1="10" x2="10" y1="11" y2="17" />
        <line x1="14" x2="14" y1="11" y2="17" />
    }
}

stroke_icon! {
    /// Lucide `copy`. Used for "Copia" (duplicate a report).
    IconCopy {
        <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
        <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
    }
}

stroke_icon! {
    /// Lucide `arrow-left`. For "back" links (e.g. the 404 page's way home).
    IconArrowLeft {
        <path d="m12 19-7-7 7-7" />
        <path d="M19 12H5" />
    }
}

stroke_icon! {
    /// Specimen jar: lid, body and fixative level. Used for Entities (doctors and materials).
    IconSpecimenJar {
        <rect x="5" y="3" width="14" height="4" rx="1" />
        <path d="M6 7v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7" />
        <path d="M6 12h12" />
    }
}

stroke_icon! {
    /// Citología: one squamous cell with its nucleus, as in a Pap smear.
    IconCitologia {
        <path d="M6 9.2 11.6 4.6 18.2 6.2 20.6 12.2 17.4 19.2 10 20.2 4.6 15Z" />
        <circle cx="12.6" cy="12.4" r="2.2" />
    }
}
//BUG: Es re feo
stroke_icon! {
    /// Histopatología: a glass slide with its label end and a tissue section.
    IconHistopatologia {
        <rect x="2" y="6" width="20" height="12" rx="2" />
        <path d="M7 6v12" />
        <path d="M12.6 10.2c1.6-1.4 4.6-1.4 5.6.4.9 1.7-.4 3.6-2.6 3.9-2.1.3-3.9-.4-4.1-1.8-.1-1 .4-1.9 1.1-2.5Z" />
    }
}
