use super::input::ProductInput;
use crate::pages::schema;
use cynic::{Id, QueryFragment, QueryVariables};

// Create
#[derive(QueryVariables)]
pub struct CreateProductVariables {
    pub data: ProductInput,
}

#[derive(QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "CreateProductVariables")]
pub struct CreateProduct {
    #[arguments(data: $data)]
    pub create_product: Vec<ProductRecord>,
}

#[derive(QueryFragment, Debug)]
pub struct ProductRecord {
    pub id: Id,
}

// Update
#[derive(QueryVariables)]
pub struct UpdateProductVariables {
    pub id: Id,
    pub data: ProductInput,
}

#[derive(QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "UpdateProductVariables")]
pub struct UpdateProduct {
    #[arguments(data: $data, id: $id)]
    pub update_product: Option<ProductRecord>,
}

// Delete
#[derive(QueryVariables)]
pub struct DeleteProductVariables {
    pub id: Id,
}

#[derive(QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "DeleteProductVariables")]
pub struct DeleteProduct {
    #[arguments(id: $id)]
    pub delete_product: Option<ProductRecord>,
}

// Products
#[derive(QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct MyQuery {
    pub get_products: ProductRecordConnection,
}

#[derive(QueryFragment, Debug)]
pub struct ProductRecordConnection {
    pub nodes: Vec<ProductRecord>,
}
