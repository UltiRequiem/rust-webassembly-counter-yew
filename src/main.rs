use wasm_bindgen::prelude::*;
use web_sys::{console};
use yew::prelude::*;

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

    console::log_1(&"Hello using web-sys".into());

    html! {
        <main>
           <p>
                <b>{ " Current value: " }</b>
                { *counter }
           </p>

            <button onclick={increment_value}>{ "Increment value" }</button>
            <button onclick={decrement_value}>{ "Decrement value" }</button>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
