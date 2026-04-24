use yew::prelude::*;
use web_sys::window;

pub struct Hero;

impl Component for Hero {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Функция скролла с ПРАВИЛЬНЫМ синтаксисом
        let scroll_to_order = Callback::from(|_| {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id("order") {
                        // Используем простой и надёжный вызов
                        element.scroll_into_view_with_bool(true);
                    }
                }
            }
        });

        html! {
            <section class="hero">
                <div class="hero-video-wrapper">
                    <div class="hero-video-placeholder"></div>
                    <div class="hero-overlay"></div>
                </div>
                <div class="hero-glow">
                    <div class="glow-orb glow-pink"></div>
                    <div class="glow-orb glow-orange"></div>
                </div>
                <div class="hero-content">
                    <div class="hero-badge"><span class="pulse-dot"></span>{"Пошаговый гайд от практика"}</div>
                    <h1 class="hero-title">{"ЛЁД, КОТОРЫЙ "}<span class="gradient-text">{"ДЕЛАЕТ ЗОЛОТО"}</span></h1>
                    <p class="hero-subtitle">{"Как превратить бесплатный жмых в "}<strong>{"90 000+ ₽/мес"}</strong>{" на домашней кухне"}</p>
                    <p class="hero-description">{"Переработал 300+ кг сырья, нашёл сухой лёд по 80 ₽/кг в Москве и упаковал всё в гайд"}</p>
                    <div class="hero-actions">
                        // Кнопка с текстом в две строки
                        <button class="btn-primary" onclick={scroll_to_order}>
                            {"🚀 ХОЧУ ДЕЛАТЬ ЗОЛОТО"}
                            <br/>
                            {"ИЗО ЛЬДА"}
                        </button>
                    </div>
                    <p class="hero-price">{"Гайд + Excel-калькулятор + 30 мин созвона. "}<span class="price-highlight">{"Цена 1 490 ₽"}</span></p>
                    <div class="hero-stats">
                        <div class="stat"><span class="stat-number">{"300+"}</span><span class="stat-label">{"кг переработано"}</span></div>
                        <div class="stat"><span class="stat-number">{"22%"}</span><span class="stat-label">{"выход продукта"}</span></div>
                        <div class="stat"><span class="stat-number">{"93 000 ₽"}</span><span class="stat-label">{"в месяц чистыми"}</span></div>
                    </div>
                </div>
                <div class="scroll-indicator">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2"><path d="M7 13l5 5 5-5M7 6l5 5 5-5"/></svg>
                </div>
            </section>
        }
    }
}
