// mod components;
mod data;
mod process;
mod query;
mod view;

use crate::pages::get_products;
use data::Data;
use leptos::*;
use leptos_router::{use_params, Params};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use process::{create_product, delete_product, get_product, update_product, upload_files};
use query::input::ProductInput;
use std::collections::HashMap;
use thaw::FileList;
use view::View;

#[derive(Params, PartialEq)]
struct ProductParams {
    id: Option<String>,
}

#[island]
pub fn Product() -> impl IntoView {
    let (selected_product, set_selected_product) = create_signal::<String>(String::new());
    let form_values = create_rw_signal::<HashMap<String, String>>(HashMap::new());
    let (state, set_state) = create_signal(false);
    let (_, set_store_id, _) = use_local_storage::<String, FromToStringCodec>("store_id");

    let _id = move || {
        use_params::<ProductParams>().with(|params| {
            params
                .as_ref()
                .map(|params| params.id.clone())
                .unwrap_or_default()
        })
    };

    let product_response = create_local_resource(
        move || selected_product.get(),
        move |id| async move { get_product(id, form_values).await },
    );

    let products_response =
        create_local_resource(|| (), move |_| async move { get_products().await });

    let create =
        create_action(
            move |_input: &()| async move { create_product(ProductInput::default()).await },
        );

    let update = create_action(move |input: &(String, ProductInput)| {
        let (id, data) = input.to_owned();

        async move { update_product(id, data).await }
    });

    let delete = create_action(move |product: &(String, Option<u8>)| {
        let (product, _) = product.to_owned();

        async move { delete_product(product).await }
    });

    let update_action = create_action(
        move |_input: &(RwSignal<HashMap<String, String>>, Option<u8>)| async move {
            let data = ProductInput {
                name: form_values
                    .get()
                    .get(&String::from("name"))
                    .unwrap()
                    .to_owned(),
                ..Default::default()
            };

            update.dispatch((selected_product.get(), data));
        },
    );

    let upload = create_action(move |input: &(FileList, Option<u8>)| {
        let files = input.to_owned().0;

        async move { upload_files(files).await }
    });

    let _form: NodeRef<html::Form> = create_node_ref();

    let _on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
    };

    let _custom_request = move |_file_list: FileList| {};

    let data = Data {
        state,
        set_state,
        product: product_response,
        products: products_response,
    };

    view! {
        <View data=data form_values=form_values _create=create on_submit=update_action upload=upload delete=delete set_selected_product=set_selected_product set_store_id=set_store_id />
    }
}
