mod form;
mod text;

use super::data::Data;
use form::Form;
use leptos::*;
use leptos_meta::*;
use std::collections::HashMap;
use thaw::{FileList, Upload, UploadDragger};

#[component]
pub fn View(
    data: Data,
    form_values: RwSignal<HashMap<String, String>>,
    create: Action<(), String>,
    on_submit: Action<(RwSignal<HashMap<String, String>>, Option<u8>), ()>,
    upload: Action<(FileList, Option<u8>), ()>,
    delete: Action<(String, Option<u8>), String>,
    set_selected_product: WriteSignal<String>,
    set_store_id: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <Title text="Home"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                <button on:click=move |_| set_store_id.set(String::from("jv6oowpygz74f36jw7fu")) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                    "Fondtees.com"
                </button>
                <button on:click=move |_| set_store_id.set(String::from("9bgm71tm36t4bq2d5n9w")) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                    "Varietydoor.com"
                </button>
                <button on:click=move |_| create.dispatch(()) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                    "Create Product"
                </button>
                    <Suspense
        fallback=move || view! { <p>"Loading..."</p> }
    >
    <For
                    each=move || data.products.with(|value| value.to_owned().unwrap_or_default())
                    key=|product| product.clone()
                    children=move |product| {
                        let producta= product.clone();
                        let productb= product.clone();
                        let productc= product;
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_selected_product.set(producta.clone())
                                    class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                                >
                                    {productb}
                                </button>
                                <button
                                    on:click=move |_| delete.dispatch((productc.clone(), None))
                                    class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
                </Suspense>

                <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
        {move || {
            data.product.get()
                .map(|_product| view! {
                    <div>
                    <Form values=form_values on_submit=on_submit />

            <Upload multiple=true custom_request=move |file_list| {upload.dispatch((file_list,None));}>
        <UploadDragger><p class="text-black">"Click or drag a file to this area to upload"</p></UploadDragger>
    </Upload>
                    </div>
                 })
        }}
    </Suspense>
                </div>
            </div>
        </main>
    }
}
