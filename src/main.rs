use web_sys::{console, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 1);

    let increment_value = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    let decrement_value = {
        let counter = counter.clone();

        console::log_1(&"This shouldn't go lower than 0.".into());

        if *counter <= 0 {
            alert(&"This value doesn't wanna be lower than one!");
            Callback::from(move |_| counter.set(*counter))
        } else {
            Callback::from(move |_| counter.set(*counter - 1))
        }
    };

    let input_node_ref = use_node_ref();

    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    console::log_1(&format!("Input is {}", input_value).into());

    let onchange = {
        let input_node_ref = input_node_ref.clone();

        let counter = counter.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();

                input_value_handle.set(value);

                // counter.set(*value.parse::<i32>().unwrap_or(0));
            }
        })
    };

    console::log_1(&"Hello using web-sys".into());

    html! {
        <main>
           <p>
                <b>{ " Current value: " }</b>
                { *counter }
            </p>
            <button onclick={increment_value}>{ "Increment value" }</button>
            <button onclick={decrement_value}>{ "Decrement value" }</button>

             <label for="my-input">
                { "Hack the current value:" }
                <input ref={input_node_ref}
                    {onchange}
                    id="my-input"
                    type="text"
                    value={input_value}
                />
            </label>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
