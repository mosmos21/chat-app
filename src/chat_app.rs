use crate::components::form::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ExecPromptArgs<'a> {
    prompt: &'a str,
}

#[component]
pub fn ChatApp(cx: Scope) -> impl IntoView {
    let (talks, set_talks) = create_signal(cx, vec![]);

    let handle_submit = move |prompt: String| {
        let prompt2 = prompt.clone();
        set_talks.update(move |talks| talks.push(prompt2));
        spawn_local(async move {
            if prompt.is_empty() {
                return;
            }

            let args = to_value(&ExecPromptArgs { prompt: &prompt }).unwrap();
            let result = invoke("exec_prompt", args).await.as_string().unwrap();
            set_talks.update(move |talks| talks.push(result));
        })
    };

    view! { cx,
        <main class="h-screen">
            <div class="h-screen flex">
                //<div class="h-screen w-72 bg-gray-900 text-gray-100">
                //    "list"
                //</div>
                <div class="h-screen flex-grow bg-gray-800 flex flex-col">
                    <div class="flex-grow overflow-y-scroll">
                        {move || talks.get().iter().map(|talk| {
                            view! { cx,
                                <div class="bg-gray-600 m-4 text-gray-100 p-2 flex items-start gap-4">
                                    <div class="rounded-full w-12 h-12 bg-gray-900"></div>
                                    <p class="whitespace-pre-wrap flex-grow">{talk}</p>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                    <Form on_submit=handle_submit />
                </div>
            </div>
        </main>
    }
}
