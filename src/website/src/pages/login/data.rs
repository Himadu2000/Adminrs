use leptos::{html::Input, NodeRef, ReadSignal, Resource};

pub struct Data {
    pub form: NodeRef<Input>,
    pub value: ReadSignal<i8>,
    pub text: Resource<(), String>,
}
