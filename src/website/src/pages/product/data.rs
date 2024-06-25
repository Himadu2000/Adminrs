use super::query::ProductRecord;
use leptos::{ReadSignal, Resource};

pub struct Data {
    pub value: ReadSignal<i8>,
    pub product: Resource<(), ProductRecord>,
}
