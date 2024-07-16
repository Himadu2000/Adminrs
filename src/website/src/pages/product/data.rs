use super::query::ProductRecord;
use leptos::{ReadSignal, Resource, WriteSignal};

pub struct Data {
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,
    pub product: Resource<String, Option<ProductRecord>>,
    pub products: Resource<(), Vec<String>>,
}
