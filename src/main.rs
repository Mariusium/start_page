mod app;
mod quotes;
mod search;
use app::App;

fn main() {
    yew::start_app::<App>();
}
