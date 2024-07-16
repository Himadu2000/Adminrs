mod text;

use super::data::Data;
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
pub fn Form(
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

                <TextInput input=form_values input_type="text" placeholder="Enter Product name *" name="name"/>
                <TextInput input=form_values input_type="text" placeholder="Enter Slug" name="slug"/>
                <p>"The “slug” is the URL-friendly version of the name. It is usually all lowercase and contains only letters, numbers, and hyphens."</p>
                <TextInput input=form_values input_type="text" placeholder="Enter Page title" name="meta_title"/>
                <TextInput input=form_values input_type="text" placeholder="Enter Meta Description" name="meta_description"/>
                <TextInput input=form_values input_type="text" placeholder="Enter Description" name="description"/>

                <TextInput input=form_values input_type="number" placeholder="Enter Regular price ($)" name="regular_price"/>
                <TextInput input=form_values input_type="number" placeholder="Enter Stock quantity" name="stock_quantity"/>
                <TextInput input=form_values input_type="number" placeholder="Enter Weight (kg)" name="weight"/>

                <TextInput input=form_values input_type="number" placeholder="Enter Position" name="position"/>

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
    }
}
