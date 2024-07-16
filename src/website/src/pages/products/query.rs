use crate::pages::schema;
use cynic::{Id, QueryFragment};

#[derive(QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct MyQuery {
    pub get_products: ProductRecordConnection,
}

#[derive(QueryFragment, Debug)]
pub struct ProductRecordConnection {
    pub edges: Vec<ProductRecordEdge>,
}

#[derive(QueryFragment, Debug)]
pub struct ProductRecordEdge {
    pub node: ProductRecord,
}

#[derive(QueryFragment, Debug)]
pub struct ProductRecord {
    pub id: Id,
}
