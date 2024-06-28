use cynic::{
    serde::{Deserialize, Serialize},
    GraphQlResponse, QueryFragment,
};
use leptos::{create_signal, SignalGetUntracked, SignalSet};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use reqwest::{multipart::Form, Client};
use std::env::var;

pub use cynic::{MutationBuilder, QueryBuilder};

pub async fn client<UnnamedQuery>(operation: impl Serialize) -> Option<UnnamedQuery>
where
    UnnamedQuery: QueryFragment + for<'a> Deserialize<'a>,
{
    let (store_id, _) = create_signal(var("STORE_ID"));
    let (flag, _, _) = use_local_storage::<String, FromToStringCodec>("token");
    let (local_store_id, set_store_id, _) =
        use_local_storage::<String, FromToStringCodec>("store_id");

    //? Set environment variable to local storage for reqwest client.
    let store_id_env = store_id
        .get_untracked()
        // .expect("STORE_ID environment variable not provided...!")
        .unwrap_or("store:obgsketriakxn1wh3q2e".to_owned());

    set_store_id.set(store_id_env);

    Client::new()
        .post("http://127.0.0.1:8000/graphql")
        .json(&operation)
        .header("Authorization", flag.get_untracked())
        .header(
            "store_id",
            store_id
                .get_untracked()
                .unwrap_or(local_store_id.get_untracked()),
        )
        .send()
        .await
        .unwrap()
        .json::<GraphQlResponse<UnnamedQuery>>()
        .await
        .unwrap()
        .data
}

pub async fn upload_client<UnnamedQuery>(form: Form) -> String
where
    UnnamedQuery: QueryFragment + for<'a> Deserialize<'a>,
{
    let (store_id, _) = create_signal(var("STORE_ID"));
    let (flag, _, _) = use_local_storage::<String, FromToStringCodec>("token");
    let (local_store_id, set_store_id, _) =
        use_local_storage::<String, FromToStringCodec>("store_id");

    //? Set environment variable to local storage for reqwest client.
    let store_id_env = store_id
        .get_untracked()
        // .expect("STORE_ID environment variable not provided...!")
        .unwrap_or("n6whojbatetnjgn85kyx".to_owned());

    set_store_id.set(store_id_env);

    Client::new()
        .post("http://127.0.0.1:8000/graphql")
        .multipart(form)
        .header("Authorization", flag.get_untracked())
        .header(
            "store_id",
            store_id
                .get_untracked()
                .unwrap_or(local_store_id.get_untracked()),
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
