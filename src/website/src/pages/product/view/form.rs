use super::text::TextInput;
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn Form(
    values: RwSignal<HashMap<String, String>>,
    on_submit: Action<(RwSignal<HashMap<String, String>>, Option<u8>), ()>,
) -> impl IntoView {
    view! {
                    <form on:submit=move |ev|{
                        ev.prevent_default();
                        on_submit.dispatch((values, None));
                    }>

                <TextInput input=values input_type="text" placeholder="Enter Product name *" name="name"/>
                <TextInput input=values input_type="text" placeholder="Enter Slug" name="slug"/>
                <p>"The “slug” is the URL-friendly version of the name. It is usually all lowercase and contains only letters, numbers, and hyphens."</p>
                <TextInput input=values input_type="text" placeholder="Enter Page title" name="meta_title"/>
                <TextInput input=values input_type="text" placeholder="Enter Meta Description" name="meta_description"/>
                <TextInput input=values input_type="text" placeholder="Enter Description" name="description"/>

                <TextInput input=values input_type="number" placeholder="Enter Regular price ($)" name="regular_price"/>
                <TextInput input=values input_type="number" placeholder="Enter Stock quantity" name="stock_quantity"/>
                <TextInput input=values input_type="number" placeholder="Enter Weight (kg)" name="weight"/>

                <TextInput input=values input_type="number" placeholder="Enter Position" name="position"/>

                <input type="submit" value="Save" class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"/>
            </form>
    }
}
