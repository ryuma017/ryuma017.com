use yew::prelude::*;

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1 class="section-title">{"About Me"}</h1>
                <div class="section-content">
                    <h3 class="name">
                        {"Ryuma Taguchi / 田口 琉真"}
                    </h3>
                    <p>
                        {"学校に通いながらプログラミングしている学生です。"}<br/>
                        {"日々学んだことのアウトプットとして制作した成果物は積極的にGitHub上に置いています。"}<br/>
                        {"是非、見に行って頂けたら嬉しいです。"}
                    </p>
                </div>
                <a class="btn" href="https://github.com/ryuma017" target="_blank">{"Check out my GitHub!"}</a>
            </>
        }
    }
}
