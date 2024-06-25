mod data;
mod query;
mod view;

use crate::pages::client::client;
use cynic::MutationBuilder;
use data::Data;
use leptos::*;
use leptos_router::{use_query, Params};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use query::{MyMutation, Variables};
use view::View;

#[derive(Params, PartialEq)]
struct LoginParams {
    token: Option<String>,
}

#[island]
pub fn Login() -> impl IntoView {
    // Not working because use_context::<RouterContext>() is None.
    // let query = use_query::<LoginParams>();
    let (_, set_flag, _) = use_local_storage::<String, FromToStringCodec>("token");

    //? Save the token if it's in the URL.
    // let resource = create_local_resource(
    //     || (),
    //     move |_| async move {
    //         let token = query.with(|params| {
    //             params
    //                 .as_ref()
    //                 .map(|params| params.token.clone())
    //                 .unwrap()
    //                 .unwrap_or_default()
    //         });

    //         set_flag.set(token.clone());

    //         token
    //     },
    // );

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

    let data = Data {
        text: resource,
        form,
    };

    view! {
        <View data=data events=(on_submit) form=form />
    }
}
