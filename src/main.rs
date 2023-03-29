mod chat_app;
mod components;

use chat_app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <ChatApp />
        }
    })
}
