use crate::pages::schema;
use cynic::QueryFragment;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation")]
pub struct MyMutation {
    #[arguments(email: "")]
    pub login: String,
}
