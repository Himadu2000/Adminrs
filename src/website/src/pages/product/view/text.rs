use leptos::*;
use std::collections::HashMap;

#[component]
pub fn TextInput(
    input: RwSignal<HashMap<String, String>>,
    input_type: &'static str,
    placeholder: &'static str,
    name: &'static str,
    // class: &'static str,
) -> impl IntoView {
    view! {
        <input type="text"
        on:input=move |ev| {
            let value=event_target_value(&ev);
            input.update(|values|{ values.insert(name.to_string(), value); });
        }
        prop:value=input.get_untracked().get(&name.to_owned()).unwrap_or_default()
        placeholder=placeholder name=name class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white"/>
    }
}
