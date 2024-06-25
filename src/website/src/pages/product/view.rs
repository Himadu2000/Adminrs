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
    E5: WriteSignal<bool> + 'static,
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

                <input type="text" placeholder="Enter Username" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="text" placeholder="Enter Username" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="text" placeholder="Enter Username" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="text" placeholder="Enter Username" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="text" placeholder="Enter Username" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

                <input type="number" placeholder="Enter Username" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="number" placeholder="Enter Username" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="number" placeholder="Enter Username" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

                <input type="number" placeholder="Enter Position" name="uname" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>

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
