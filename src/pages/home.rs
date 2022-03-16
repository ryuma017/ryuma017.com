use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{"Welcome to ryuma017's parsonal portfolio site."}</p>
                <p>{"This site is powered by WASM compiled by Rust, which I am currently studying"}</p>
            </>
        }
    }
}
