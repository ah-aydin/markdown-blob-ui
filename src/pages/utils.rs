use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::context::auth::get_auth_context;

pub fn auth_required() {
    let auth_context = get_auth_context();
    Effect::new(move |_| {
        if auth_context.read().is_none() {
            use_navigate()("/login", Default::default());
        }
    });
}

pub fn no_auth_required() {
    let auth_context = get_auth_context();
    Effect::new(move |_| {
        if auth_context.read().is_some() {
            use_navigate()("/", Default::default());
        }
    });
}
