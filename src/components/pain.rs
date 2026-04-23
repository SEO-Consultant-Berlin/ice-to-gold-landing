use yew::prelude::*;

pub struct Pain;
impl Component for Pain {
    type Message = (); type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="section"><div class="container"><div class="pain-grid">
                <div class="pain-text">
                    <h2 class="section-title">{"Знакомая "}<span class="gradient-text">{"ситуация?"}</span></h2>
                    <div class="pain-list">
                        {["Вы купили сухой лёд за 180₽/кг и слили бюджет впустую","Порошок слипся в кирпич через 3 дня, и клиент вернул деньги","Жмых из смузи-бара дал выход 5% вместо 22%, и вы работали в ноль","Вы потратили 2 недели на переговоры, а продаж нет"].iter().map(|t| html!{<div class="pain-item"><span class="pain-icon">{"❌"}</span><span>{*t}</span></div>}).collect::<Html>()}
                    </div>
                    <div class="pain-quote"><p>{"Я прошёл этот путь. Слил 15 000 ₽, прежде чем понял, как работает система."}</p></div>
                </div>
                <div class="pain-images">
                    <div class="pain-image-card bad"><div class="image-placeholder bad-img">{"Испорченный порошок"}</div><p class="image-label error">{"← Ошибка новичка"}</p></div>
                    <div class="pain-image-card good"><div class="image-placeholder good-img">{"Качественный порошок"}</div><p class="image-label success">{"Результат по методу →"}</p></div>
                </div>
            </div></div></section>
        }
    }
}
