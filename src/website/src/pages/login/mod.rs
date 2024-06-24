mod data;
mod query;
mod view;

use crate::pages::client::client;
use cynic::MutationBuilder;
use data::Data;
use leptos::*;
use leptos_router::{use_params, Params};
use query::MyMutation;
use view::View;

#[derive(Params, PartialEq)]
struct LoginParams {
    token: Option<String>,
}

#[island]
pub fn Login() -> impl IntoView {
    let (value, set_value) = create_signal::<i8>(0);

    let token = move || {
        use_params::<LoginParams>().with(|params| {
            params
                .as_ref()
                .map(|params| params.token.clone())
                .unwrap_or_default()
        })
    };

    let response = create_resource(
        || (),
        |_| async move {
            client::<MyMutation>(MyMutation::build(()))
                .await
                .unwrap()
                .login
        },
    );

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
        <View data=data events=(on_submit) form=form />
    }
}
