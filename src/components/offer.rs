use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Plan {
    pub name: String,
    pub price: String,
    pub features: Vec<String>,
    pub highlighted: bool,
    pub cta: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct OfferProps {
    pub plans: Vec<Plan>,
    pub onbuy: Callback<()>,
    pub onsubmit: Callback<()>,
}

#[function_component(Offer)]
pub fn offer(props: &OfferProps) -> Html {
    let OfferProps { plans, onbuy, onsubmit } = props.clone();

    html! {
        <section class="offer-section">
            <div class="offer-container">
                <h2 class="offer-title">{"Выберите свой тариф"}</h2>
                <div class="offer-grid">
                    { for plans.into_iter().map(|plan| {
                        let highlighted = plan.highlighted;
                        let cta = plan.cta.clone();
                        let name = plan.name.clone();
                        let price = plan.price.clone();
                        let features = plan.features.clone();
                        let onbuy = onbuy.clone();
                        let onsubmit = onsubmit.clone();

                        html! {
                            <div class={classes!("offer-card", highlighted.then(|| Some("offer-card--highlighted")))}>
                                if highlighted {
                                    <span class="offer-card__badge">{"ХИТ"}</span>
                                }
                                <h3 class="offer-card__title">{name}</h3>
                                <div class="offer-card__price">
                                    <span class="offer-card__amount">{price}</span>
                                </div>
                                <ul class="offer-card__features">
                                    { for features.into_iter().map(|feature| {
                                        html! { <li class="offer-card__feature">{feature}</li> }
                                    })}
                                </ul>
                                <button class={classes!(
                                    "offer-card__btn",
                                    highlighted.then(|| Some("offer-card__btn--accent"))
                                )}
                                onclick={
                                    let cb = if highlighted { onsubmit.clone() } else { onbuy.clone() };
                                    move |_| cb.emit(())
                                }>
                                    {cta}
                                </button>
                            </div>
                        }
                    })}
                </div>
            </div>
        </section>
    }
}
