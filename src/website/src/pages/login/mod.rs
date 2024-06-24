mod data;
mod query;
mod view;

use crate::pages::client::client;
use cynic::MutationBuilder;
use data::Data;
use leptos::*;
use query::MyMutation;
use view::View;

#[island]
pub fn Login() -> impl IntoView {
    let (value, set_value) = create_signal::<i8>(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout

    let response = create_resource(
        || (),
        |_| async move {
            client::<MyMutation>(MyMutation::build(()))
                .await
                .unwrap()
                .login
        },
    );

    let add = move |_| set_value.update(|value| *value += 1);
    let sub = move |_| set_value.update(|value| *value -= 1);

    let form: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = form
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
    };

    let data = Data {
        form,
        value,
        text: response,
    };

    view! {
        <View data=data events=(on_submit) />
    }
}
