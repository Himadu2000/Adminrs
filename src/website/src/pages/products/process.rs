use super::query::MyQuery;
use crate::pages::client::{client, QueryBuilder};

pub async fn get_products() -> Vec<String> {
    let data = client::<MyQuery>(MyQuery::build(())).await;

    if let Some(products) = data {
        let list = products
            .get_products
            .edges
            .iter()
            .map(|product| product.node.id.inner().to_owned())
            .collect();

        return list;
    }

    vec![]
}
