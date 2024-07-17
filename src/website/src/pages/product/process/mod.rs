use super::query::{
    crud::{
        CreateProduct, CreateProductVariables, DeleteProduct, DeleteProductVariables,
        UpdateProduct, UpdateProductVariables,
    },
    input::ProductInput,
    ProductRecord, UnnamedQuery, Variables,
};
use crate::pages::client::{client, upload_client, MutationBuilder, QueryBuilder};
use js_sys::Uint8Array;
use leptos::*;
use log::info;
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;
use thaw::FileList;

pub async fn get_product(
    selected_product: String,
    form_values: RwSignal<HashMap<String, String>>,
) -> Option<ProductRecord> {
    let variables = Variables {
        id: selected_product.into(),
    };

    let data = client::<UnnamedQuery>(UnnamedQuery::build(variables)).await;

    if let Some(with_product) = data {
        let with_product = with_product.get_product;
        let data_product = with_product.clone();

        form_values.update(|values| {
            values.insert(String::from("name"), data_product.name);
            values.insert(String::from("slug"), data_product.slug);
            values.insert(String::from("meta_title"), data_product.meta_title);
            values.insert(
                String::from("meta_description"),
                data_product.meta_description,
            );
            values.insert(String::from("description"), data_product.description);
        });

        return Some(with_product);
    }

    None
}

pub async fn create_product(product: ProductInput) -> String {
    let variables = CreateProductVariables { data: product };

    client::<CreateProduct>(CreateProduct::build(variables))
        .await
        .unwrap()
        .create_product
        .first()
        .unwrap()
        .to_owned()
        .id
        .inner()
        .to_owned()
}

pub async fn update_product(id: String, data: ProductInput) {
    let variables = UpdateProductVariables { id, data };

    let token = client::<UpdateProduct>(UpdateProduct::build(variables))
        .await
        .unwrap()
        .update_product
        .unwrap()
        .id;
}

pub async fn delete_product(product: String) -> String {
    let variables = DeleteProductVariables { id: product.into() };

    client::<DeleteProduct>(DeleteProduct::build(variables))
        .await
        .unwrap()
        .delete_product
        .unwrap()
        .id
        .inner()
        .to_owned()
}

pub async fn upload_files(files: FileList) {
    let form = Form::new()
    .text("operations", format!("{{ 'query': 'mutation($file: [Upload!]) {{ updateProduct(id: &apos;30amj7a97zfbvi5rqxv2&apos;, data: {{}}, images: $file) {{ id images {{ file alt }} }} }}', 'variables': {{ 'file': null }}}}",).replace('\'', "\""))
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
