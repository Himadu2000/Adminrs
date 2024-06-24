use crate::pages::schema;
use cynic::QueryFragment;

#[derive(QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct UnnamedQuery {
    pub status: String,
}
