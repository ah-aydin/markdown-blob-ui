mod api;
mod api_types;
mod components;
mod context;
mod pages;
mod utils;

use context::auth::get_auth_context;
use context::auth::set_auth_context;
use context::auth::setup_auth_context;
use leptos::component;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::IntoView;
use leptos_router::components::Route;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use leptos_router::path;
use pages::login::LoginPage;
use pages::signup::SignupPage;
use pages::tutorial::Tutorial;

#[component]
fn App() -> impl IntoView {
    setup_auth_context();
    let auth_context = get_auth_context();
    let set_auth_context = set_auth_context();

    view! {
        <Router>
            <nav>
                <Show when=move || { auth_context.read().is_some() } fallback=|| view! { <div /> }>
                    <button on:click=move |e| {
                        e.prevent_default();
                        set_auth_context.set(None);
                    }>Logout</button>
                </Show>
            </nav>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=Tutorial />
                    <Route path=path!("/login") view=LoginPage />
                    <Route path=path!("/signup") view=SignupPage />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
