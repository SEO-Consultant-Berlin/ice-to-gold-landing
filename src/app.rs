use yew::prelude::*;
use crate::components::{
    hero::Hero, pain::Pain, features::Features, target::Target,
    economics::Economics, testimonials::Testimonials, offer::Offer, footer::Footer
};

pub struct App;
impl Component for App {
    type Message = (); type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <Hero />
                <Pain />
                <Features />
                <Target />
                <Economics />
                <Testimonials />
                <Offer />
                <Footer />
            </main>
        }
    }
}
