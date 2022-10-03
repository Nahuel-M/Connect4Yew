use yew::{html, function_component, Component, Html};

struct Board{
    board_state : i64,
}

impl Component for Board{
    type Message = ();
    type Properties = ();
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self { board_state: 0 }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        todo!()
    }
}

#[function_component(App)]
fn app() -> Html {
    html!{<Board/>}
}

fn main() {
    yew::start_app::<App>();
}