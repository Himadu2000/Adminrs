mod form;
mod text;

use super::data::Data;
use form::Form;
use leptonic::toggle::Toggle;
use leptos::{ev::SubmitEvent, *};
use leptos_meta::*;
use std::collections::HashMap;
use thaw::{FileList, Upload, UploadDragger};

#[component]
pub fn View(
    data: Data,
    form_values: RwSignal<HashMap<String, String>>,
    custom_request: E2,
    on_submit: Action<(RwSignal<HashMap<String, String>>, Option<u8>), ()>,
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
                    <Form values=form_values on_submit=on_submit />

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
