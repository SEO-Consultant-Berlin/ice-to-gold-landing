use yew::prelude::*;

pub struct Economics;
impl Component for Economics {
    type Message = (); type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="section alt-bg"><div class="container">
                <h2 class="section-title text-center">{"Посмотрите на эти цифры"}</h2>
                <p class="section-subtitle text-center">{"Это не магия. Это математика."}</p>
                <div class="econ-card">
                    <div class="econ-grid">
                        <div class="econ-item"><span class="econ-label">{"Сырьё"}</span><span class="econ-value">{"10 кг"}</span><span class="econ-note green">{"бесплатно"}</span></div>
                        <div class="econ-item"><span class="econ-label">{"Выход"}</span><span class="econ-value">{"110 стиков"}</span><span class="econ-note">{"по 20 г"}</span></div>
                        <div class="econ-item"><span class="econ-label">{"Затраты на цикл"}</span><span class="econ-value">{"3 150 ₽"}</span></div>
                        <div class="econ-item"><span class="econ-label">{"Выручка"}</span><span class="econ-value">{"16 500 ₽"}</span></div>
                    </div>
                    <div class="econ-profit"><span class="profit-label">{"Чистая прибыль с одного цикла"}</span><span class="profit-value gradient-text">{"13 350 ₽"}</span></div>
                    <div class="econ-total"><p>{"8 циклов в месяц = "}<strong>{"93 000 ₽ чистыми"}</strong></p><p class="econ-note">{"Уже за вычетом налогов и аренды ларя"}</p></div>
                </div>
            </div></section>
        }
    }
}
