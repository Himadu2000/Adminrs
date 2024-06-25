use cynic::{
    serde::{Deserialize, Serialize},
    GraphQlResponse, QueryFragment,
};
use leptos::SignalGet;
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use reqwest::Client;

pub use cynic::{MutationBuilder, QueryBuilder};

pub async fn client<UnnamedQuery>(operation: impl Serialize) -> Option<UnnamedQuery>
where
    UnnamedQuery: QueryFragment + for<'a> Deserialize<'a>,
{
    let (flag, _, _) = use_local_storage::<String, FromToStringCodec>("token");

    Client::new()
        .post("http://127.0.0.1:8000/graphql")
        .json(&operation)
        .header("Authorization", flag.get())
        .header("store_id", flag.get())
        .send()
        .await
        .unwrap()
        .json::<GraphQlResponse<UnnamedQuery>>()
        .await
        .unwrap()
        .data
}
