// mod components;
mod data;
mod query;
mod view;

use crate::pages::client::{client, MutationBuilder, QueryBuilder};
use data::Data;
use leptos::*;
use leptos_router::{use_params, Params};
use query::{
    crud::{
        CreateProduct, CreateProductVariables, DeleteProduct, DeleteProductVariables,
        UpdateProduct, UpdateProductVariables,
    },
    input::ProductInput,
    UnnamedQuery, Variables,
};
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
        create_signal::<String>(String::from("zinftrowowh62zcp64fj"));
    let (form_values, set_form_values) = create_signal::<HashMap<String, String>>(HashMap::new());
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

            client::<UnnamedQuery>(UnnamedQuery::build(variables))
                .await
                .unwrap()
                .get_product
        },
    );

    let add = move |_| set_value.update(|value| *value += 1);
    let sub = move |_| set_value.update(|value| *value -= 1);

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

        let value = form.get().expect("<form> should be mounted").value();

        // update.dispatch(value);
    };

    let custom_request = move |file_list: FileList| {};

    let data = Data {
        value,
        state,
        set_state,
        product: response,
    };

    view! {
        <View data=data events=(on_submit, custom_request, add, sub, sub) sub=on_submit form=form set_form_values=set_form_values />
    }
}
