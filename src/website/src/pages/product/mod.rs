mod components;
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
    UnnamedQuery,
};
use view::View;

#[derive(Params, PartialEq)]
struct ProductParams {
    id: Option<String>,
}

#[island]
pub fn Product() -> impl IntoView {
    let (value, set_value) = create_signal::<i8>(0);

    let id = move || {
        use_params::<ProductParams>().with(|params| {
            params
                .as_ref()
                .map(|params| params.id.clone())
                .unwrap_or_default()
        })
    };

    let response = create_resource(
        || (),
        |_| async move {
            client::<UnnamedQuery>(UnnamedQuery::build(()))
                .await
                .unwrap()
                .get_product
                .unwrap()
                .id
        },
    );

    let add = move |_| set_value.update(|value| *value += 1);
    let sub = move |_| set_value.update(|value| *value -= 1);

    let update = create_action(move |product: &ProductInput| {
        let product = product.to_owned();

        async move {
            let variables = UpdateProductVariables {
                id: String::new(),
                data: product,
            };

            let token = client::<UpdateProduct>(UpdateProduct::build(variables))
                .await
                .unwrap()
                .update_product
                .unwrap()
                .id;
        }
    });

    let update = create_action(move |product: &ProductInput| {
        let product = product.to_owned();

        async move {
            let variables = UpdateProductVariables {
                id: String::new(),
                data: product,
            };

            let token = client::<UpdateProduct>(UpdateProduct::build(variables))
                .await
                .unwrap()
                .update_product
                .unwrap()
                .id;
        }
    });

    let update = create_action(move |product: &ProductInput| {
        let product = product.to_owned();

        async move {
            let variables = UpdateProductVariables {
                id: String::new(),
                data: product,
            };

            let token = client::<UpdateProduct>(UpdateProduct::build(variables))
                .await
                .unwrap()
                .update_product
                .unwrap()
                .id;
        }
    });

    let form: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = form.get().expect("<input> should be mounted").value();

        update.dispatch(value);
    };

    let data = Data {
        value,
        text: response,
    };

    view! {
        <View data=data events=(add, sub) />
    }
}
