use super::data::Data;
use leptos::{ev::MouseEvent, *};
use leptos_meta::*;

#[component]
pub fn View<E1, E2>(data: Data, events: (E1, E2)) -> impl IntoView
where
    E1: Fn(MouseEvent) + 'static,
    E2: Fn(MouseEvent) + 'static,
{
    view! {
        <Title text="Home"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=events.0 class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <p class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {data.value}
                    </p>
                    <button on:click=events.1 class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "-"
                    </button>
                    <Suspense
        fallback=move || view! { <p>"Loading..."</p> }
    >
        {move || {
            data.text.get()
                .map(|text| view! { <p>{text}</p> })
        }}
    </Suspense>
                </div>
            </div>
        </main>
    }
}
