use leptos::prelude::*;

const ACCESS_TOKEN_KEY: &str = "markdown_blob_access_token";
const REFRESH_TOKEN_KEY: &str = "markdown_blob_refresh_token";

#[derive(Clone)]
pub struct AuthContext {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn setup_auth_context() {
    let (auth_context, set_auth_context) = signal::<Option<AuthContext>>(None);
    provide_context(auth_context);
    provide_context(set_auth_context);

    // Load tokens from local storage
    Effect::new(move |_| {
        let local_storage = window()
            .local_storage()
            .expect("JS call to get local storage failed")
            .expect("Failed to get local storage");

        let access_token_maybe = local_storage
            .get(ACCESS_TOKEN_KEY)
            .expect("Failed to read access token from local storage");
        let refresh_token_maybe = local_storage
            .get(REFRESH_TOKEN_KEY)
            .expect("Failed to read refresh token from local storage");

        let auth_context = match (access_token_maybe, refresh_token_maybe) {
            (Some(access_token), Some(refresh_token)) => Some(AuthContext {
                access_token,
                refresh_token,
            }),
            _ => None,
        };
        set_auth_context.set(auth_context);
    });

    // Save tokens to local storage
    Effect::new(move |_| {
        let local_storage = window()
            .local_storage()
            .expect("JS call to get local storage failed")
            .expect("Failed to get local storage");

        let auth_context = auth_context.get();
        if auth_context.is_none() {
            local_storage
                .remove_item(ACCESS_TOKEN_KEY)
                .expect("Failed to remove access token from local storage");
            local_storage
                .remove_item(REFRESH_TOKEN_KEY)
                .expect("Failed to remove refresh token from local storage");
            return;
        }

        let auth_context = auth_context.unwrap();
        local_storage
            .set(ACCESS_TOKEN_KEY, &auth_context.access_token)
            .expect("Failed to store access token");
        local_storage
            .set(REFRESH_TOKEN_KEY, &auth_context.refresh_token)
            .expect("Failed to store refresh token");
    });
}

pub fn get_auth_context() -> ReadSignal<Option<AuthContext>> {
    use_context::<ReadSignal<Option<AuthContext>>>().expect("to have found auth read provider")
}

pub fn set_auth_context() -> WriteSignal<Option<AuthContext>> {
    use_context::<WriteSignal<Option<AuthContext>>>().expect("to have found auth set proivder")
}
