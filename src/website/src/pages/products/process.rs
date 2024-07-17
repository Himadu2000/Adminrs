use super::query::MyQuery;
use crate::pages::client::{client, QueryBuilder};

pub async fn get_products() -> Vec<String> {
    client::<MyQuery>(MyQuery::build(()))
        .await
        .unwrap()
        .get_products
        .edges
        .iter()
        .map(|product| product.node.id.inner().to_owned())
        .collect()
}
