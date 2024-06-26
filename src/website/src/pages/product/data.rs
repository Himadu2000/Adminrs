use super::query::ProductRecord;
use leptos::{ReadSignal, Resource, WriteSignal};

pub struct Data {
    pub value: ReadSignal<i8>,
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,
    pub product: Resource<(), ProductRecord>,
}
