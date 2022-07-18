use rand::seq::SliceRandom;
use rand::Rng;
use yew::prelude::*;

#[function_component(Quotes)]
pub fn quotes() -> Html {
    let demotivational_quote_list = vec!["Light travels faster than sound. This is why some people appear bright until you hear them speak. - Alan Dundes", 
    "Everything happens for a reason. Sometimes the reason is you're stupid and make bad decisions. - Marion G. Harmon", 
    "Trying is the first step toward failure. - Homer Simpson", "Not everything is a lesson. Sometimes you just fail. - Dwight Schrute"];

    let motivational_quote_list = vec!["Life is 10 percent what happens to me and 90 percent of how I react to it. - Charles Swindoll",
    "Happiness is not something readymade. It comes from your own actions. - Dalai Lama"];

    // let quote_list = [&demotivational_quote_list[..], &motivational_quote_list[..]].concat();

    let mut quote = "";
    let mut notification_class = "";

    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..2);

    if random_number == 0 {
        // demotivational quotes
        quote = demotivational_quote_list
            .choose(&mut rand::thread_rng())
            .unwrap();
        notification_class = "notification is-danger";
    } else if random_number == 1 {
        // motivaional quotes
        quote = motivational_quote_list
            .choose(&mut rand::thread_rng())
            .unwrap();
        notification_class = "notification is-success";
    }
    web_sys::console::log_1(&random_number.into());

    html! {
    <div class="container">
        <section class="section">
            <p class={notification_class}>{quote}</p>
        </section>
    </div>
        }
}
