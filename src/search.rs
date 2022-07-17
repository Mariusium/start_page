use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[function_component(Search)]
pub fn search() -> Html {
    let my_text_handle = use_state(|| "  ".to_string());
    let my_text = (*my_text_handle).clone();

    let handle_input = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap_throw();
        let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
        let value = input_elem.value();
        web_sys::console::log_1(&input_elem.value().into());
        my_text_handle.set(value); // update as user types
    });

    html! {
    <div class="container">
        <section class="section">
            <div class="field has-addons">
                <div class="control is-expanded">
                    <input oninput={handle_input} type="text" class="input" placeholder="d duckduckgo, g google, t twitch" />
                </div>
                <p class="control">
                    <a href={search_term_builder(my_text)} class="button">{"SEARCH"}</a>
                </p>
            </div>
        </section>
    </div>
    }
}

pub fn search_term_builder(my_text: String) -> String {
    let search_url = "https://duckduckgo.com/?q=";

    let sdf = my_text.as_str();
    let search_term;

    if my_text.chars().count() > 2 {
        let first_two_characters = &sdf[..2];
        let everything_else = &sdf[2..];

        match first_two_characters {
            "g " => {
                search_term = format!("{}{}", "https://www.google.com/search?q=", &everything_else)
            }
            "d " => search_term = format!("{}{}", "https://duckduckgo.com/?q=", &everything_else),
            "t " => {
                search_term = format!(
                    "{}{}",
                    "https://www.twitch.tv/search?term=", &everything_else
                )
            }
            _ => search_term = format!("{}{}", search_url, &my_text),
        }
    } else {
        search_term = format!("{}{}", search_url, &my_text);
    }

    search_term
}
