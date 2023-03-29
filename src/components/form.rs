use leptos::{
    ev::{Event, SubmitEvent},
    *,
};

#[component]
pub fn Form<F>(cx: Scope, on_submit: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let (prompt, set_prompt) = create_signal(cx, String::new());

    let handle_change_textarea = move |e: Event| {
        let value = event_target_value(&e);
        set_prompt.set(value);
    };

    let handle_submit = move |e: SubmitEvent| {
        e.prevent_default();
        on_submit(prompt.get());
        set_prompt.set(String::from(""));
    };

    view! { cx,
        <form
            class="text-gray-100 flex m-4 p-4 bg-gray-900 gap-4 items-end"
            on:submit=handle_submit
        >
            <div class="bg-gray-800 p-2 flex-grow">
                <textarea
                    rows="4"
                    class="bg-transparent focus:outline-none resize-none w-full"
                    value={move || prompt.get()}
                    on:change=handle_change_textarea
                ></textarea>
            </div>
            <input type="submit" value="submit" class="w-32 p-2 bg-gray-800"/>
        </form>
    }
}
