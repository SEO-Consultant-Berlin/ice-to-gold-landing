use yew::prelude::*;

pub struct Features;
impl Component for Features {
    type Message = (); type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let items = vec![
            ("🗺️","Карта поставщиков","5 реальных точек в Москве, где сухой лёд стоит 80₽/кг, а не 200₽. С именами и паролями."),
            ("🧪","Технология сублимации","Пошаговый протокол для 5 видов жмыха. Как не убить продукт за 24 часа."),
            ("📊","Excel-калькулятор","Вбили свои цифры — увидели реальную прибыль. Без иллюзий."),
            ("💬","Скрипты продаж","Что написать кондитеру, чтобы он попросил пробник, а не послал вас."),
            ("⚖️","Юридический минимум","Самозанятость, декларация, маркировка — как не получить штраф от Роспотребнадзора."),
            ("🎁","Бонус: созвон 30 мин","Личная Zoom-консультация. Разберём вашу конкретную ситуацию."),
        ];
        html! {
            <section class="section alt-bg"><div class="container">
                <h2 class="section-title text-center">{"Что внутри "}<span class="gradient-text">{"гайда"}</span></h2>
                <p class="section-subtitle text-center">{"Вся информация, которую обычно продают за 15 000 ₽ на платных курсах"}</p>
                <div class="features-grid">
                    {items.into_iter().map(|(icon,title,desc)| html!{
                        <div class="feature-card"><div class="feature-icon">{icon}</div><h3>{title}</h3><p>{desc}</p></div>
                    }).collect::<Html>()}
                </div>
            </div></section>
        }
    }
}
