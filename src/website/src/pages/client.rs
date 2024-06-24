use cynic::{
    serde::{Deserialize, Serialize},
    GraphQlResponse, QueryFragment,
};
use reqwest::Client;

pub use cynic::QueryBuilder;

pub async fn client<UnnamedQuery>(operation: impl Serialize) -> Option<UnnamedQuery>
where
    UnnamedQuery: QueryFragment + for<'a> Deserialize<'a>,
{
    Client::new()
        .post("http://127.0.0.1:8000/graphql")
        .json(&operation)
        .send()
        .await
        .unwrap()
        .json::<GraphQlResponse<UnnamedQuery>>()
        .await
        .unwrap()
        .data
}
