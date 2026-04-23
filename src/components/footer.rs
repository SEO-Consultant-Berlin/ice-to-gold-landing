use yew::prelude::*;

pub struct Footer;
impl Component for Footer {
    type Message = (); type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class="footer"><div class="container"><div class="footer-content">
                <p>{"© 2026 | Тюлюков А. О. | ИНН 770000000000"}</p>
                <div class="footer-links"><a href="#">{"Политика конфиденциальности"}</a><a href="#">{"Договор-оферта"}</a><a href="#">{"Telegram"}</a></div>
            </div></div></footer>
        }
    }
}
