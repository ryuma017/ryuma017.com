use yew::prelude::*;

pub struct Contact;

impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1 class="section-title">{"Get In Touch"}</h1>
                <p>
                    {"ご覧いただきありがとうございました。"}<br/>
                    {"このサイトを通して、私のことを少しでも知っていただけたのなら嬉しいです。"}<br/>
                    {"もし、このサイトや私について何かコメントがありましたら、下のボタンをご利用ください。"}
                </p>
                <a class="btn" href="mailto:ryuma.taguchi@gmail.com">{"send email"}</a>
            </>
        }
    }
}
