pub mod crud;
pub mod input;

use crate::pages::schema;
use serde::Serialize;

#[derive(cynic::QueryVariables)]
pub struct Variables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "Variables")]
pub struct UnnamedQuery {
    #[arguments(id: $id)]
    pub get_product: Option<ProductRecord>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct ProductRecord {
    pub attributes: Vec<Attribute>,
    pub code: Option<String>,
    pub cost_price: Option<f64>,
    pub date_sale_from: String,
    pub date_sale_to: String,
    pub date_stock_expected: String,
    pub description: Option<String>,
    pub dimensions: Option<Dimensions>,
    pub discontinued: bool,
    pub enabled: bool,
    pub id: String,
    pub images: Vec<Image>,
    pub meta_description: Option<String>,
    pub meta_title: Option<String>,
    pub name: Option<String>,
    pub options: Vec<ProductOption>,
    pub position: i32,
    pub prices: Vec<f64>,
    pub quantity_inc: i32,
    pub quantity_min: i32,
    pub regular_price: f64,
    pub sale_price: Option<f64>,
    pub sku: Option<String>,
    pub slug: Option<String>,
    pub stock_backorder: bool,
    pub stock_preorder: bool,
    pub stock_quantity: i32,
    pub stock_tracking: bool,
    pub tags: Vec<String>,
    pub tax_class: Option<String>,
    pub variants: Vec<Variant>,
    pub weight: Option<f64>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct Variant {
    pub options: String,
    pub price: f64,
    pub sku: Option<String>,
    pub stock_quantity: i32,
    pub weight: Option<f64>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct ProductOption {
    pub control: ProductOptionControl,
    pub name: String,
    pub position: i32,
    pub required: bool,
    pub values: Vec<Value>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct Value {
    pub name: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct Image {
    pub alt: Option<String>,
    pub file: String,
    pub position: i32,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct Dimensions {
    pub height: f64,
    pub length: f64,
    pub width: f64,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum ProductOptionControl {
    Select,
}
