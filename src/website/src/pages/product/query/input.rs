use crate::pages::schema;
use cynic::QueryFragment;

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "Query")]
pub struct UnnamedQuery {
    #[arguments(id: "")]
    pub get_product: Option<ProductRecord>,
}

#[derive(cynic::InputObject, cynic::QueryVariables, Debug)]
pub struct ProductRecord {
    pub attributes: Vec<AttributeInput>,
    pub code: Option<String>,
    pub cost_price: Option<f64>,
    pub date_sale_from: String,
    pub date_sale_to: String,
    pub date_stock_expected: String,
    pub description: Option<String>,
    pub dimensions: Option<DimensionsInput>,
    pub discontinued: bool,
    pub enabled: bool,
    pub id: String,
    pub images: Vec<ImageInput>,
    pub meta_description: Option<String>,
    pub meta_title: Option<String>,
    pub name: Option<String>,
    pub options: Vec<ProductOptionInput>,
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
    pub variants: Vec<VariantInput>,
    pub weight: Option<f64>,
}

#[derive(cynic::InputObject, Debug)]
pub struct VariantInput {
    pub price: f64,
    pub sku: Option<String>,
    pub stock_quantity: i32,
    pub weight: Option<f64>,
}

#[derive(cynic::InputObject, Debug)]
pub struct ProductOptionInput {
    pub control: ProductOptionControl,
    pub name: String,
    pub position: i32,
    pub required: bool,
    pub values: Vec<ValueInput>,
}

#[derive(cynic::InputObject, Debug)]
pub struct ValueInput {
    pub name: String,
}

#[derive(cynic::InputObject, Debug)]
pub struct ImageInput {
    pub alt: Option<String>,
    pub position: i32,
}

#[derive(cynic::InputObject, Debug)]
pub struct DimensionsInput {
    pub height: f64,
    pub length: f64,
    pub width: f64,
}

#[derive(cynic::InputObject, Debug)]
pub struct AttributeInput {
    pub name: String,
    pub value: String,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum ProductOptionControl {
    Select,
}
