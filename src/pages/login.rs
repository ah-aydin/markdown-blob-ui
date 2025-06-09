use leptos::component;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::IntoView;

use crate::api::auth::login;
use crate::context::auth::set_auth_context;
use crate::context::auth::AuthContext;
use crate::pages::utils::no_auth_required;
use crate::utils::InputExtractor;

#[component]
pub fn LoginPage() -> impl IntoView {
    let set_auth_context = set_auth_context();

    no_auth_required();

    let (error, set_error) = signal::<Option<String>>(None);
    let (disabled, set_disabled) = signal(false);
    let email_input: NodeRef<html::Input> = NodeRef::new();
    let password_input: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        set_disabled.set(true);

        let email = email_input.extract_value();
        let password = password_input.extract_value();

        spawn_local(async move {
            let login_result = login(email, password).await;
            match login_result {
                Ok(login_response) => {
                    set_error.set(None);
                    set_auth_context.set(Some(AuthContext {
                        access_token: login_response.access_token,
                        refresh_token: login_response.refresh_token,
                    }));
                }
                Err(Some(err)) => {
                    set_error.set(Some(err.message));
                }
                Err(_) => {
                    set_error.set(Some("Unknown error".to_string()));
                }
            };
            set_disabled.set(false);
        });
    };

    view! {
        <h1>"Login"</h1>
        <form on:submit=on_submit>
            <h3>"Email"</h3>
            <input type="email" value="" placeholder="some.email@domain.com" node_ref=email_input />
            <h3>"Password"</h3>
            <input
                type="password"
                value=""
                placeholder="super duper secret password"
                node_ref=password_input
            />
            <button type="submit" prop:disabled=disabled>
                Login
            </button>
        </form>
        <p class="red">{error}</p>
    }
}
