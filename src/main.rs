use yew::{html::Scope, prelude::*};
use yew_router::prelude::*;

mod pages;
use pages::{
    about::About, contact::Contact, home::Home, page_not_found::PageNotFound, works::Works,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/works")]
    Works,
    #[at("/404")]
    NotFound,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer>
                    <div>
                        { "© " } <a href="#"> { "ryuma017. " } </a> { "2022. " }
                        { "Powered by " } <a href="https://yew.rs">{ "Yew" }</a>
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <header>
                // <img class="logo" src="public/images/R_icon.svg" alt="logo" />
                <div class="logo">{ "R" }</div>
                <nav>
                    <ul class="nav__links">
                        <li>
                            <Link<Route> to={Route::Home}>
                                { "Home" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route> to={Route::About}>
                                { "About" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route> to={Route::Works}>
                                { "Works" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route> to={Route::Contact}>
                                { "Contact" }
                            </Link<Route>>
                        </li>
                    </ul>
                </nav>
            </header>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::About => {
            html! { <About/> }
        }
        Route::Contact => {
            html! { <Contact/> }
        }
        Route::Home => {
            html! { <Home/> }
        }
        Route::Works => {
            html! { <Works/> }
        }
        Route::NotFound => {
            html! { <PageNotFound/> }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
