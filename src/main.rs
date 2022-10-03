use yew::{html, function_component};

#[function_component(App)]
fn app() -> Html {
    html!{<p>{"Test"}</p>}
}

fn main() {
    yew::start_app::<App>();
}