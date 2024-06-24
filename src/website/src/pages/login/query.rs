use crate::pages::schema;
use cynic::QueryFragment;

#[derive(cynic::QueryVariables)]
pub struct Variables {
    pub email: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "Variables")]
pub struct MyMutation {
    #[arguments(email: $email)]
    pub login: String,
}
