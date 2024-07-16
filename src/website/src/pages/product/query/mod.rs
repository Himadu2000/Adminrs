pub mod crud;
pub mod input;

use crate::pages::schema;
use cynic::{Enum, Id, QueryFragment, QueryVariables};
use serde::Serialize;

#[derive(QueryVariables)]
pub struct Variables {
    pub id: Id,
}

#[derive(QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "Variables")]
pub struct UnnamedQuery {
    #[arguments(id: $id)]
    pub get_product: ProductRecord,
}

#[derive(Clone, QueryFragment, Debug, Serialize)]
pub struct ProductRecord {
    pub name: String,
    pub description: String,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
    pub regular_price: f64,
    pub sale_price: f64,
    pub date_sale_from: String,
    pub date_sale_to: String,
    pub sku: String,
    pub stock_quantity: i32,
    pub weight: f64,
    pub date_stock_expected: String,
    pub stock_tracking: bool,
    pub stock_preorder: bool,
    pub stock_backorder: bool,
    pub discontinued: bool,
    pub enabled: bool,
    pub attributes: Vec<Attribute>,
    pub variants: Vec<Variant>,
    // pub category_ids: Vec<Id>,
    pub tags: Vec<String>,
    pub position: i32,
    // pub related_products: Vec<Id>,
    pub images: Vec<ImageOutput>,
}

#[derive(Clone, QueryFragment, Debug, Serialize)]
pub struct ImageOutput {
    pub file: String,
    pub mime: String,
    pub alt: String,
}

#[derive(Clone, QueryFragment, Debug, Serialize)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Enum, Clone, Copy, Debug)]
pub enum ProductOptionControl {
    SELECT,
}

#[derive(Clone, QueryFragment, Debug, Serialize)]
pub struct VariantOption {
    pub name: String,
    pub control: ProductOptionControl,
    pub required: bool,
    pub values: Vec<String>,
}

#[derive(Clone, QueryFragment, Debug, Serialize)]
pub struct Variant {
    pub sku: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub weight: f64,
    pub options: Vec<VariantOption>,
}
