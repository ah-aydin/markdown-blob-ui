use leptos::html;
use leptos::prelude::Get;
use leptos::prelude::NodeRef;

pub trait InputExtractor {
    fn extract_value(&self) -> String;
}

impl InputExtractor for NodeRef<html::Input> {
    fn extract_value(&self) -> String {
        self.get().expect("<input> should be mounted").value()
    }
}
