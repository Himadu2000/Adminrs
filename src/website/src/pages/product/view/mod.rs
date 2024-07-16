use super::data::Data;
use leptonic::toggle::Toggle;
use leptos::{
    ev::{MouseEvent, SubmitEvent},
    *,
};
use leptos_meta::*;
use std::collections::HashMap;
use thaw::{FileList, Upload, UploadDragger};

#[component]
pub fn TextInput(
    input: RwSignal<HashMap<String, String>>,
    input_type: &'static str,
    placeholder: &'static str,
    name: &'static str,
    class: &'static str,
) -> impl IntoView {
    view! {
        <input type="text"
        on:input=move |ev| {
            let value=event_target_value(&ev);
            input.update(|values|{ values.insert(name.to_string(), value); });
        }
        prop:value=input.get().get(&name.to_owned())
        placeholder=placeholder name=name class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
    }
}

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

                <TextInput input=form_values input_type="text" placeholder="Enter Product name *" name="name" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <TextInput input=form_values input_type="text" placeholder="Enter Slug" name="slug" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <p>"The “slug” is the URL-friendly version of the name. It is usually all lowercase and contains only letters, numbers, and hyphens."</p>
                <TextInput input=form_values input_type="text" placeholder="Enter Page title" name="meta_title" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <TextInput input=form_values input_type="text" placeholder="Enter Meta Description" name="meta_description" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <TextInput input=form_values input_type="text" placeholder="Enter Description" name="description" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

                <TextInput input=form_values input_type="number" placeholder="Enter Regular price ($)" name="regular_price" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <TextInput input=form_values input_type="number" placeholder="Enter Stock quantity" name="stock_quantity" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <TextInput input=form_values input_type="number" placeholder="Enter Weight (kg)" name="weight" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

                <TextInput input=form_values input_type="number" placeholder="Enter Position" name="position" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

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