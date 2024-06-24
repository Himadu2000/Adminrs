mod data;
mod query;
mod view;

use crate::pages::client::{client, QueryBuilder};
use data::Data;
use leptos::*;
use query::UnnamedQuery;
use view::View;

#[island]
pub fn Home() -> impl IntoView {
    let (value, set_value) = create_signal::<i8>(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout

    let response = create_resource(
        || (),
        |_| async move {
            client::<UnnamedQuery>(UnnamedQuery::build(()))
                .await
                .unwrap()
                .status
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
