use super::input::ProductInput;
use crate::pages::schema;

// Create
#[derive(cynic::QueryVariables)]
pub struct CreateProductVariables {
    pub data: ProductInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "CreateProductVariables")]
pub struct CreateProduct {
    #[arguments(data: $data)]
    pub create_product: Vec<ProductRecord>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductRecord {
    pub id: String,
}

// Update
#[derive(cynic::QueryVariables)]
pub struct UpdateProductVariables {
    pub id: String,
    pub data: ProductInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "UpdateProductVariables")]
pub struct UpdateProduct {
    #[arguments(data: $data, id: $id)]
    pub update_product: Option<ProductRecord>,
}

// Delete
#[derive(cynic::QueryVariables)]
pub struct DeleteProductVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "DeleteProductVariables")]
pub struct DeleteProduct {
    #[arguments(id: $id)]
    pub delete_product: Option<ProductRecord>,
}

// Delete Image
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation")]
pub struct MyMutation {
    #[arguments(id: "", index: 10)]
    pub delete_image: Option<ProductRecord>,
}
