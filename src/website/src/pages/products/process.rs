use super::query::{MyQuery, ProductRecord};
use crate::pages::client::{client, upload_client, MutationBuilder, QueryBuilder};
use js_sys::Uint8Array;
use leptos::*;
use log::info;
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;
use thaw::FileList;

pub async fn get_products() -> Vec<ProductRecord> {
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
