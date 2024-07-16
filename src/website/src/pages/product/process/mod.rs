// mod components;
mod data;
mod query;
mod view;

use crate::pages::client::{client, upload_client, MutationBuilder, QueryBuilder};
use data::Data;
use js_sys::Uint8Array;
use leptos::*;
use leptos_router::{use_params, Params};
use log::info;
use query::{
    crud::{
        CreateProduct, CreateProductVariables, DeleteProduct, DeleteProductVariables,
        UpdateProduct, UpdateProductVariables,
    },
    input::ProductInput,
    UnnamedQuery, Variables,
};
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;
use thaw::FileList;
use view::View;

#[derive(Params, PartialEq)]
struct ProductParams {
    id: Option<String>,
}

#[island]
pub fn Product() -> impl IntoView {
    let (value, set_value) = create_signal::<i8>(0);
    let (selected_product, set_selected_product) =
        create_signal::<String>(String::from("jduohkgejwv7xfdpyh7e"));
    let form_values = create_rw_signal::<HashMap<String, String>>(HashMap::new());
    let (state, set_state) = create_signal(false);

    let id = move || {
        use_params::<ProductParams>().with(|params| {
            params
                .as_ref()
                .map(|params| params.id.clone())
                .unwrap_or_default()
        })
    };

    let response = create_local_resource(
        || (),
        move |_| async move {
            let variables = Variables {
                id: selected_product.get(),
            };

            let data = client::<UnnamedQuery>(UnnamedQuery::build(variables))
                .await
                .unwrap()
                .get_product;

            let data_product = data.clone();
            form_values.update(|values| {
                values.insert(String::from("name"), data_product.name.unwrap_or_default());
                values.insert(String::from("slug"), data_product.slug.unwrap_or_default());
                values.insert(
                    String::from("meta_title"),
                    data_product.meta_title.unwrap_or_default(),
                );
                values.insert(
                    String::from("meta_description"),
                    data_product.meta_description.unwrap_or_default(),
                );
                values.insert(
                    String::from("description"),
                    data_product.description.unwrap_or_default(),
                );
            });

            data
        },
    );

    let create = create_action(move |product: &ProductInput| {
        let product = product.to_owned();

        async move {
            let variables = CreateProductVariables { data: product };

            let token = client::<CreateProduct>(CreateProduct::build(variables))
                .await
                .unwrap()
                .create_product
                .first()
                .unwrap()
                .to_owned()
                .id
                .clone();
        }
    });

    let update = create_action(move |input: &(String, ProductInput)| {
        let (id, data) = input.to_owned();

        async move {
            let variables = UpdateProductVariables { id, data };

            let token = client::<UpdateProduct>(UpdateProduct::build(variables))
                .await
                .unwrap()
                .update_product
                .unwrap()
                .id;
        }
    });

    let delete = create_action(move |product: &ProductInput| {
        let product = product.to_owned();

        async move {
            let variables = DeleteProductVariables { id: String::new() };

            let token = client::<DeleteProduct>(DeleteProduct::build(variables))
                .await
                .unwrap()
                .delete_product
                .unwrap()
                .id;
        }
    });

    let form: NodeRef<html::Form> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
    };

    let custom_request = move |file_list: FileList| {};

    let data = Data {
        value,
        state,
        set_state,
        product: response,
        products: response,
    };

    let update_action = create_action(move |_input: &()| async move {
        let data = ProductInput {
            name: form_values.get().get(&String::from("name")).cloned(),
            ..Default::default()
        };

        update.dispatch((selected_product.get(), data));
    });

    let upload = create_action(move |input: &(FileList, Option<u8>)| {
        let files = input.to_owned().0;

        async move {
            let form = Form::new()
            .text("operations", "{ 'query': 'mutation ($file: Upload!) { upload(file: $file) }', 'variables': { 'file': null }}".replace('\'', "\""))
            .text("map", "{ '0': ['variables.file'] }".replace('\'', "\""));

            let list = (0..files.length())
                .collect::<Vec<u32>>()
                .iter()
                .map(|index| {
                    let file = files.item(*index).expect("File");

                    let mut bytes = Vec::new();
                    Uint8Array::new(&file).copy_to(&mut bytes);

                    let file_name = file.name();
                    let mime = file_name.split('.').last().unwrap_or_default();
                    let mime = format!("image/{}", mime);

                    let part = Part::bytes(bytes)
                        .file_name(file_name)
                        .mime_str(&mime)
                        .expect("Part");

                    (index.to_string(), part)
                })
                .fold(form, |accumulator, (index, part)| {
                    accumulator.part(index, part)
                });

            let res = upload_client(list).await;

            info!("{:?}", res);
        }
    });

    view! {
        <View data=data form_values=form_values on_submit=update_action upload=upload />
    }
}
