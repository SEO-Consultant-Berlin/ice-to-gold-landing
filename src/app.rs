use yew::prelude::*;
use crate::components::{
    hero::Hero, pain::Pain, features::Features, target::Target,
    economics::Economics, testimonials::Testimonials, offer::Offer,
    offer::Plan, footer::Footer
};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // 1. Создаём колбэки нужного типа Callback<()>
        let onbuy = Callback::from(|_| {
            // Здесь будет логика для кнопок "Купить" (Базовый и Расширенный)
            // Пока просто выводим сообщение в консоль
            web_sys::console::log_1(&"Нажата кнопка Купить".into());
        });

        let onsubmit = Callback::from(|_| {
            // Здесь будет логика для кнопки "Стать VIP"
            web_sys::console::log_1(&"Нажата кнопка Стать VIP".into());
        });

        // 2. Описываем три тарифа
        let plans = vec![
            Plan {
                name: "Базовый".to_string(),
                price: "1 490 ₽".to_string(),
                features: vec![
                    "✅ PDF-гайд (35 страниц)".to_string(),
                    "✅ Excel-калькулятор".to_string(),
                    "📧 Поддержка в Telegram".to_string(),
                ],
                highlighted: false,
                cta: "ЗАКАЗАТЬ".to_string(),
            },
            Plan {
                name: "Расширенный".to_string(),
                price: "2 990 ₽".to_string(),
                features: vec![
                    "✅ Всё из «Базового»".to_string(),
                    "✅ 30 мин созвона в Zoom".to_string(),
                    "✅ Чек-лист «Запуск за 7 дней»".to_string(),
                    "✅ 3 скрипта доп. продаж".to_string(),
                ],
                highlighted: true, // <-- Самый выгодный, будет выделен
                cta: "ВЫБРАТЬ".to_string(),
            },
            Plan {
                name: "VIP: Всё включено".to_string(),
                price: "9 990 ₽".to_string(),
                features: vec![
                    "✅ Всё из «Расширенного»".to_string(),
                    "✅ Личное наставничество 2 часа".to_string(),
                    "✅ Аудит вашей первой партии".to_string(),
                    "✅ Список из 50 клиентов в вашем городе".to_string(),
                ],
                highlighted: false,
                cta: "СТАТЬ VIP".to_string(),
            },
        ];

        html! {
            <main>
                <Hero />
                <Pain />
                <Features />
                <Target />
                <Economics />
                <Testimonials />
                // 3. Передаём всё в компонент
                <Offer plans={plans} onbuy={onbuy} onsubmit={onsubmit} />
                <Footer />
            </main>
        }
    }
}
