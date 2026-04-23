use yew::prelude::*;
use web_sys::SubmitEvent;

pub struct Offer;
impl Component for Offer {
    type Message = (); type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onsubmit = |e: SubmitEvent| {
            e.prevent_default();
            let window = web_sys::window().unwrap();
            let _ = window.alert_with_message("Спасибо! Сейчас вы будете перенаправлены на оплату.");
        };
        html! {
            <section class="section alt-bg" id="offer"><div class="container"><div class="offer-grid">
                <div class="offer-info">
                    <h2 class="section-title">{"Гайд "}<span class="gradient-text">{"«ЛЁД, КОТОРЫЙ ДЕЛАЕТ ЗОЛОТО»"}</span></h2>
                    <ul class="offer-features">
                        <li>{"✅ PDF-гайд (35 страниц концентрированной пользы)"}</li>
                        <li>{"✅ Excel-калькулятор рентабельности"}</li>
                        <li>{"✅ 30 минут личного созвона в Zoom"}</li>
                    </ul>
                    <div class="offer-price-block"><span class="offer-price gradient-text">{"1 490 ₽"}</span><p class="offer-compare">{"Это меньше, чем 5 кг сухого льда в розницу. Но эта информация сэкономит вам десятки тысяч рублей."}</p></div>
                    <div class="offer-guarantee"><h4>{"🔒 ГАРАНТИЯ ВОЗВРАТА"}</h4><p>{"Если в течение 14 дней вы поймёте, что гайд не принёс пользы — я верну деньги. Без вопросов."}</p></div>
                </div>
                <div class="offer-form-wrapper"><h3>{"Оформить заказ"}</h3>
                    <form onsubmit={onsubmit} class="offer-form">
                        <label>{"Имя *"}</label><input type="text" required=true placeholder="Александр" />
                        <label>{"Email *"}</label><input type="email" required=true placeholder="alex@example.com" />
                        <label>{"Telegram (необязательно)"}</label><input type="text" placeholder="@username" />
                        <button type="submit" class="btn-primary btn-full">{"ОПЛАТИТЬ 1 490 ₽ И ПОЛУЧИТЬ ГАЙД"}</button>
                    </form>
                    <p class="form-disclaimer">{"Нажимая кнопку, вы соглашаетесь с офертой и политикой конфиденциальности"}</p>
                </div>
            </div></div></section>
        }
    }
}
