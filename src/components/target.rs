use yew::prelude::*;

pub struct Target;
impl Component for Target {
    type Message = (); type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let items = vec![
            ("👩‍🍳","Кондитеры и пекари","Ищете натуральный краситель, который не разжижает крем и стоит в 3 раза дешевле покупного? Вам сюда."),
            ("🧀","Крафтовики и сыроделы","Хотите добавить в линейку премиальные ягодные порошки с маржой 300%? Этот гайд — ваша инструкция."),
            ("🏠","Самозанятые и ищущие нишу","Нужен бизнес с нулевым порогом входа и быстрой окупаемостью? Начните с этого гайда сегодня."),
        ];
        html! {
            <section class="section"><div class="container">
                <h2 class="section-title text-center">{"Для кого этот "}<span class="gradient-text">{"гайд"}</span></h2>
                <div class="target-grid">
                    {items.into_iter().map(|(emoji,title,desc)| html!{
                        <div class="target-card"><div class="target-emoji">{emoji}</div><h3>{title}</h3><p>{desc}</p></div>
                    }).collect::<Html>()}
                </div>
            </div></section>
        }
    }
}
