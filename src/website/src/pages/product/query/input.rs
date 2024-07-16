use crate::pages::schema;

#[derive(Clone, cynic::InputObject, Debug, Default)]
pub struct ProductInput {
    pub name: String,
    pub description: String,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
    pub regular_price: f64,
    pub sale_price: f64,
    // pub date_sale_from: Datetime,
    // pub date_sale_to: Datetime,
    pub sku: String,
    pub stock_quantity: i32,
    pub weight: f64,
    // pub date_stock_expected: Datetime,
    pub stock_tracking: bool,
    pub stock_preorder: bool,
    pub stock_backorder: bool,
    pub discontinued: bool,
    pub enabled: bool,
    pub attributes: Vec<AttributeInput>,
    // pub variants: Vec<VariantInput>,
    // pub category_ids: Vec<Thing>,
    pub tags: Vec<String>,
    pub position: i32,
    // pub related_products: Vec<Thing>,
    // pub images: Vec<ImageInput>,
}

// #[derive(Clone, cynic::InputObject, Debug)]
// pub struct ImageInput {
//     pub alt: String,
//     pub file: String,
// }

#[derive(Clone, cynic::InputObject, Debug)]
pub struct AttributeInput {
    pub name: String,
    pub value: String,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum ProductOptionControl {
    Select,
}

#[derive(Clone, cynic::InputObject, Debug)]
pub struct ProductOptionInput {
    pub name: String,
    pub control: ProductOptionControl,
    pub required: bool,
    pub values: Vec<String>,
}

#[derive(Clone, cynic::InputObject, Debug)]
pub struct VariantInput {
    pub sku: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub weight: f64,
    pub options: Vec<ProductOptionInput>,
}
