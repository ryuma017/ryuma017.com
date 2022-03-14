use yew::{prelude::*, html::Scope};
use yew_router::prelude::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
