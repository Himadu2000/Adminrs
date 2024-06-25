use super::data::Data;
use leptonic::toggle::Toggle;
use leptos::{
    ev::{MouseEvent, SubmitEvent},
    *,
};
use leptos_meta::*;
use thaw::{FileList, Upload, UploadDragger};

#[component]
pub fn View<E1, E2, E3, E4, E5>(data: Data, events: (E1, E2, E3, E4, E5)) -> impl IntoView
where
    E1: Fn(SubmitEvent) + 'static,
    E2: Fn(FileList) + 'static,
    E3: Fn(MouseEvent) + 'static,
    E4: Fn(MouseEvent) + 'static,
    E5: Fn(MouseEvent) + 'static,
{
    view! {
        <Title text="Home"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=events.3 class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <p class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {data.value}
                    </p>
                    <button on:click=events.4 class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "-"
                    </button>
                    <Suspense
        fallback=move || view! { <p>"Loading..."</p> }
    >
        {move || {
            data.product.get()
                .map(|product| view! {
                    <div>
                    <form on:submit=events.1>

                <input type="text" placeholder="Enter Product name *" name="name" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="text" placeholder="Enter Slug" name="slug" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <p>"The “slug” is the URL-friendly version of the name. It is usually all lowercase and contains only letters, numbers, and hyphens."</p>
                <input type="text" placeholder="Enter Page title" name="meta_title" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="text" placeholder="Enter Meta Description" name="meta_description" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="text" placeholder="Enter Description" name="description" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

                <input type="number" placeholder="Enter Regular price ($)" name="regular_price" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="number" placeholder="Enter Stock quantity" name="stock_quantity" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="number" placeholder="Enter Weight (kg)" name="weight" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

                <input type="number" placeholder="Enter Position" name="position" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

                <input type="submit" value="Log In" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"/>
            </form>

            <Toggle state=data.state set_state=events.5/>

            <Upload custom_request=events.2>
        <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
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
