use yew::prelude::*;

pub struct Testimonials;
impl Component for Testimonials {
    type Message = (); type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let items = vec![
            ("До гайда я платил 180₽ за кг льда и думал, что это норма. После — нашёл поставщика за 80₽. Гайд окупился с первой же закупки льда.","Андрей, Москва"),
            ("Скрипт для кондитеров — это бомба. Отправил 5 сообщений — получил 3 заказа на пробники. Две пекарни теперь мои постоянные клиенты.","Елена, Санкт-Петербург"),
            ("Excel-калькулятор расставил всё по местам. Думала, что работаю в плюс, а оказалось — в ноль. Пересчитала — вышла на 60 000 ₽ в первый месяц.","Ольга, Казань"),
        ];
        html! {
            <section class="section"><div class="container">
                <h2 class="section-title text-center">{"Что говорят те, "}<span class="gradient-text">{"кто уже применил систему"}</span></h2>
                <div class="testimonials-grid">
                    {items.into_iter().map(|(text,author)| html!{
                        <div class="testimonial-card"><div class="testimonial-quote">{"\""}</div><p class="testimonial-text">{text}</p><p class="testimonial-author">{"— "}{author}</p></div>
                    }).collect::<Html>()}
                </div>
            </div></section>
        }
    }
}
