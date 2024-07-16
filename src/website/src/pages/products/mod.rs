mod data;
mod query;
mod view;

use crate::pages::client::{client, QueryBuilder};
use data::Data;
use leptos::*;
use query::MyQuery;
use view::View;

#[island]
pub fn Products() -> impl IntoView {
    let (value, set_value) = create_signal::<i8>(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout

    let response = create_resource(
        || (),
        |_| async move {
            <cynic::Id as Clone>::clone(
                &client::<MyQuery>(MyQuery::build(()))
                    .await
                    .unwrap()
                    .get_products
                    .edges
                    .first()
                    .unwrap()
                    .node
                    .id,
            )
            .into_inner()
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
