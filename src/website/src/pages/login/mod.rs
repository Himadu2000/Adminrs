mod data;
mod query;
mod view;

use crate::pages::client::client;
use cynic::MutationBuilder;
use data::Data;
use leptos::*;
use leptos_router::{use_params, Params};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use query::{MyMutation, Variables};
use view::View;

#[derive(Params, PartialEq)]
struct LoginParams {
    token: Option<String>,
}

#[island]
pub fn Login() -> impl IntoView {
    let (flag, set_flag) = use_cookie::<String, FromToStringCodec>("token");

    use_params::<LoginParams>().with(|params| {
        let token = params
            .as_ref()
            .map(|params| params.token.clone())
            .unwrap()
            .unwrap_or_default();

        if token.len() > 10 {
            set_flag.set(token);
        };
    });

    // 03124701209@gmail.com
    let login = create_action(move |email: &String| {
        let email = email.to_owned();

        async move {
            let variables = Variables { email };

            let token = client::<MyMutation>(MyMutation::build(variables))
                .await
                .unwrap()
                .login;

            set_flag.set(token);
        }
    });

    let form: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = form.get().expect("<input> should be mounted").value();

        login.dispatch(value);
    };

    let data = Data { form };

    view! {
        <View data=data events=(on_submit) form=form />
    }
}
