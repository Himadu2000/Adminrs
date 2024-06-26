use cynic::{
    serde::{Deserialize, Serialize},
    GraphQlResponse, QueryFragment,
};
use leptos::{create_resource, SignalGet, SignalGetUntracked};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use reqwest::Client;
use std::env::var;

pub use cynic::{MutationBuilder, QueryBuilder};

pub async fn client<UnnamedQuery>(operation: impl Serialize) -> Option<UnnamedQuery>
where
    UnnamedQuery: QueryFragment + for<'a> Deserialize<'a>,
{
    let (flag, _, _) = use_local_storage::<String, FromToStringCodec>("token");
    let (local_store_id, _, _) = use_local_storage::<String, FromToStringCodec>("store_id");

    let store_id = create_resource(
        || (),
        move |_| async move { var("STORE_ID").unwrap_or("obgsketriakxn1wh3q2e".to_owned()) },
    );

    Client::new()
        .post("http://127.0.0.1:8000/graphql")
        .json(&operation)
        .header("Authorization", flag.get_untracked())
        .header("store_id", store_id.get().unwrap())
        .send()
        .await
        .unwrap()
        .json::<GraphQlResponse<UnnamedQuery>>()
        .await
        .unwrap()
        .data
}
