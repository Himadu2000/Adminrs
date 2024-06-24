use super::data::Data;
use leptos::{
    ev::{MouseEvent, SubmitEvent},
    *,
};
use leptos_meta::*;

#[component]
pub fn View<E1>(data: Data, events: (E1)) -> impl IntoView
where
    E1: Fn(SubmitEvent),
{
    view! {
        <Title text="Home"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                <form on:submit=events.1>
                <input type="text" node_ref=input_element class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
                <input type="submit" value="Log In" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"/>
            </form>
                </div>
            </div>
        </main>
    }
}
