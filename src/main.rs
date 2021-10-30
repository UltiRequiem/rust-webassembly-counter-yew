use yew::prelude::*;

enum Msg {
    AddOne,
    RemoveOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::RemoveOne => {
                if self.value > 0 {
                    self.value -= 1;
                }

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="container">
            <main class=classes!("main")>
                <h1>{ "A Counter" }</h1>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::RemoveOne)>{ "-1" }</button>

                <p>{ "The current count is " } { self.value }</p>

                <footer>
                    <p>{ "© 2021 Eliaz Bobadilla (a.k.a UltiRequiem)" }</p>
                </footer>
            </main>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
