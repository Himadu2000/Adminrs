use leptos::{ReadSignal, Resource};

pub struct Data {
    pub value: ReadSignal<i8>,
    pub text: Resource<(), String>,
}
