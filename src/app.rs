use yew::prelude::*;

use crate::quotes::Quotes;
use crate::search::Search;

pub enum Msg {}

#[derive(Debug, Default)]
pub struct App {}
impl App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <main>
            <section class="section">
                <div class="container">
                    <h1 class="title">{ "Hello Marius :)" }</h1>
                    <p class="subtitle">{ "My start page built with "} <strong>{"Rust, Yew and Bulma"}</strong>{"!"}</p>
                </div>
            </section>
           <div>
                <Search />
            </div>
            <div>
                <Quotes />
            </div>
        </main>
            }
    }
}
