mod data;
mod query;
mod view;

use crate::pages::client::{client, QueryBuilder};
use data::Data;
use leptos::*;
use leptos_router::{use_params, Params};
use query::UnnamedQuery;
use view::View;

#[derive(Params, PartialEq)]
struct ProductParams {
    id: Option<String>,
}

#[island]
pub fn Product() -> impl IntoView {
    let (value, set_value) = create_signal::<i8>(0);

    let id = move || {
        use_params::<ProductParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
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

    let data = Data {
        value,
        text: response,
    };

    view! {
        <View data=data events=(add, sub) />
    }
}
