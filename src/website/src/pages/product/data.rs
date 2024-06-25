use super::query::ProductRecord;
use leptos::{ReadSignal, Resource};

pub struct Data {
    pub value: ReadSignal<i8>,
    pub state: ReadSignal<bool>,
    pub product: Resource<(), ProductRecord>,
}
