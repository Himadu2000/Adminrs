use crate::pages::schema;
use cynic::QueryFragment;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct MyQuery {
    pub get_products: IntConnection,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct IntConnection {
    pub edges: Vec<IntEdge>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct IntEdge {
    pub node: i32,
}
