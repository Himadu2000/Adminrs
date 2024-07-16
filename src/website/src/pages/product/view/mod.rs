mod form;
mod text;

use super::data::Data;
use form::Form;
use leptonic::toggle::Toggle;
use leptos::{
    ev::{MouseEvent, SubmitEvent},
    *,
};
use leptos_meta::*;
use std::collections::HashMap;
use text::TextInput;
use thaw::{FileList, Upload, UploadDragger};

#[component]
pub fn View(
    data: Data,
    form_values: RwSignal<HashMap<String, String>>,
    custom_request: E2,
    on_submit: Action<(), ()>,
    upload: Action<(FileList, Option<u8>), ()>,
) -> impl IntoView
where
    E1: Fn(SubmitEvent) + 'static,
    E2: Fn(FileList) + 'static,
{
    view! {
        <Title text="Home"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <p>"Product"</p>
                    <Suspense
        fallback=move || view! { <p>"Loading..."</p> }
    >
        {move || {
            data.product.get()
                .map(|product| view! {
                    <div>
                    <form on:submit=move |ev|{
                        ev.prevent_default();
                        on_submit.dispatch(());
                    }>

                <Form input=form_values />

                <input type="submit" value="Save" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"/>
            </form>

            <Toggle state=data.state set_state=data.set_state/>

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
