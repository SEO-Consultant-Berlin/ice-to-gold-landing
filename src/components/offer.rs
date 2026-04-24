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
            <section class="section alt-bg" id="offer">
                <div class="container">
                    <h2 class="section-title text-center">{"Выберите "}<span class="gradient-text">{"тариф"}</span></h2>
                    <p class="section-subtitle text-center">{ "Каждый тариф включает полный гайд. Чем выше уровень — тем быстрее и глубже погружение в бизнес." }</p>

                    <div class="pricing-grid">
                        // Карточка 1: Базовый
                        <div class="pricing-card">
                            <div class="pricing-header">
                                <h3>{"🧊 Базовый"}</h3>
                                <p class="pricing-desc">{"Для тех, кто любит разбираться сам"}</p>
                            </div>
                            <div class="pricing-price"><span class="gradient-text">{"1 490 ₽"}</span></div>
                            <ul class="pricing-features">
                                <li>{"✅ PDF-гайд (35 страниц)"}</li>
                                <li>{"✅ Excel-калькулятор"}</li>
                                <li class="disabled">{"❌ Личный разбор ниши"}</li>
                                <li class="disabled">{"❌ Готовые договоры с клиентами"}</li>
                                <li class="disabled">{"❌ База поставщиков «под ключ»"}</li>
                            </ul>
                            <button class="btn-primary btn-full" onclick={onsubmit.clone()}>{"ВЗЯТЬ БАЗОВЫЙ"}</button>
                        </div>

                        // Карточка 2: Расширенный (Выделенный)
                        <div class="pricing-card popular">
                            <div class="popular-badge">{"⚡ САМЫЙ ВЫГОДНЫЙ"}</div>
                            <div class="pricing-header">
                                <h3>{"💎 Расширенный"}</h3>
                                <p class="pricing-desc">{"Для быстрого старта с гарантией"}</p>
                            </div>
                            <div class="pricing-price"><span class="gradient-text">{"2 990 ₽"}</span></div>
                            <ul class="pricing-features">
                                <li>{"✅ PDF-гайд (35 страниц)"}</li>
                                <li>{"✅ Excel-калькулятор"}</li>
                                <li>{"✅ 30 минут Zoom-разбора"}</li>
                                <li class="disabled">{"❌ Готовые договоры с клиентами"}</li>
                                <li class="disabled">{"❌ База поставщиков «под ключ»"}</li>
                            </ul>
                            <button class="btn-primary btn-full" onclick={onsubmit.clone()}>{"ВЗЯТЬ РАСШИРЕННЫЙ"}</button>
                        </div>

                        // Карточка 3: VIP
                        <div class="pricing-card">
                            <div class="pricing-header">
                                <h3>{"👑 VIP: Всё включено"}</h3>
                                <p class="pricing-desc">{"Бизнес «под ключ» за 1 месяц"}</p>
                            </div>
                            <div class="pricing-price"><span class="gradient-text">{"9 990 ₽"}</span></div>
                            <ul class="pricing-features">
                                <li>{"✅ PDF-гайд (35 страниц)"}</li>
                                <li>{"✅ Excel-калькулятор"}</li>
                                <li>{"✅ 60 минут Zoom-разбора"}</li>
                                <li>{"✅ Готовые договоры с клиентами"}</li>
                                <li>{"✅ База поставщиков «под ключ»"}</li>
                            </ul>
                            <button class="btn-primary btn-full" onclick={onsubmit.clone()}>{"СТАТЬ VIP"}</button>
                        </div>
                    </div>

                    // Секция с формой быстрого заказа (без привязки к тарифу)
                    <div class="quick-order" id="order-form">
                        <div class="offer-guarantee text-center">
                            <h4>{"🔒 ГАРАНТИЯ ВОЗВРАТА"}</h4>
                            <p>{"Если в течение 14 дней вы поймёте, что гайд не принёс пользы — я верну деньги. Без вопросов."}</p>
                        </div>
                        <div class="offer-form-wrapper">
                            <h3>{"Оформить заказ"}</h3>
                            <form onsubmit={onsubmit} class="offer-form">
                                <label>{"Имя *"}</label><input type="text" required=true placeholder="Александр" />
                                <label>{"Email *"}</label><input type="email" required=true placeholder="alex@example.com" />
                                <label>{"Telegram (необязательно)"}</label><input type="text" placeholder="@username" />
                                <button type="submit" class="btn-primary btn-full">{"ОПЛАТИТЬ И ПОЛУЧИТЬ ДОСТУП"}</button>
                            </form>
                            <p class="form-disclaimer">{"Нажимая кнопку, вы соглашаетесь с офертой и политикой конфиденциальности"}</p>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
