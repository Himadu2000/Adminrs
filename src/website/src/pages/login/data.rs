use leptos::{html::Input, NodeRef, ReadSignal, Resource};

pub struct Data {
    pub text: Resource<(), String>,
    pub form: NodeRef<Input>,
}
