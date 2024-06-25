use crate::pages::schema;
use cynic::QueryFragment;

// Create
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "FilmVariables")]
pub struct MyMutation {
    #[arguments(data: {  })]
    pub create_product: Vec<ProductRecord>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductRecord {
    pub id: String,
}

// Update
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "FilmVariables")]
pub struct MyMutation {
    #[arguments(data: {  }, id: "")]
    pub update_product: Option<ProductRecord>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductRecord {
    pub id: String,
}

// Delete
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation")]
pub struct MyMutation {
    #[arguments(id: "")]
    pub delete_product: Option<ProductRecord>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductRecord {
    pub id: String,
}

// Delete Image
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation")]
pub struct MyMutation {
    #[arguments(id: "", index: 10)]
    pub delete_image: Option<ProductRecord>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductRecord {
    pub id: String,
}
