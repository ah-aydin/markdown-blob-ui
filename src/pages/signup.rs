use leptos::component;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::IntoView;
use leptos_router::hooks::use_navigate;

use crate::api::auth::signup;
use crate::pages::utils::no_auth_required;
use crate::utils::InputExtractor;

#[component]
pub fn SignupPage() -> impl IntoView {
    no_auth_required();

    let (success, set_success) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);
    let (disabled, set_disabled) = signal(false);
    let email_input: NodeRef<html::Input> = NodeRef::new();
    let password_input: NodeRef<html::Input> = NodeRef::new();
    let repeate_password_input: NodeRef<html::Input> = NodeRef::new();

    Effect::new(move |_| {
        if success.get() {
            use_navigate()("/login", Default::default());
        }
    });

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        set_disabled.set(true);

        let email = email_input.extract_value();
        let password = password_input.extract_value();
        let repeate_password = repeate_password_input.extract_value();

        if password != repeate_password {
            set_error.set(Some("Passwords do not match".to_string()));
            set_disabled.set(false);
            return;
        }

        spawn_local(async move {
            let signup_result = signup(email, password).await;
            match signup_result {
                Ok(signup_response) => {
                    log!("Created user {:?}", signup_response);
                    set_error.set(None);
                    set_disabled.set(false);
                    set_success.set(true);
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
        <h1>"Sign Up"</h1>
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
            <h3>"Repeat Password"</h3>
            <input
                type="password"
                value=""
                placeholder="super duper secret password repeate"
                node_ref=repeate_password_input
            />
            <button type="submit" prop:disabled=disabled>
                "Sign Up"
            </button>
        </form>
        <p class="red">{error}</p>
    }
}
