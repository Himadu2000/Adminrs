use cynic::{
    serde::{Deserialize, Serialize},
    GraphQlResponse, QueryFragment,
};
use leptos::{create_signal, SignalGetUntracked};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use reqwest::Client;
use std::env::var;

pub use cynic::{MutationBuilder, QueryBuilder};

pub async fn client<UnnamedQuery>(operation: impl Serialize) -> Option<UnnamedQuery>
where
    UnnamedQuery: QueryFragment + for<'a> Deserialize<'a>,
{
    let (store_id, _) = create_signal(var("STORE_ID"));
    let (flag, _, _) = use_local_storage::<String, FromToStringCodec>("token");
    let (local_store_id, _, _) = use_local_storage::<String, FromToStringCodec>("store_id");

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
