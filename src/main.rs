use yew::{html, function_component, Component, Html};

struct Board{
    board_state_x : i64,
    board_state_o : i64,
}

impl Board {
    fn to_pretty_string(& self) -> String {
        let mut s = "".to_string();
        let mut o = self.board_state_o;
        let mut x = self.board_state_x;
        for i in 0..42 {
            let co = o & 1;
            let cx = x & 1;
            o >>= 1;
            x >>= 1;
            if i % 7 == 0 {
                s += "\n";
            }
            s+= if co == 1 { "O" } else if cx == 1 { "X" } else { "_" };
        }
        s
    }
}

impl Component for Board{
    type Message = ();
    type Properties = ();
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.to_pretty_string();
        true
    }

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self { board_state_x: 0b11111110000000, board_state_o: 0b1111111 }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html!{self.to_pretty_string()}
    }
}

#[function_component(App)]
fn app() -> Html {
    html!{<Board/>}
}

fn main() {
    yew::start_app::<App>();
}