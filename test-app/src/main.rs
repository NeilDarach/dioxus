use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let mut score_a: Signal<u32> = use_signal(|| 0);
    let mut score_b: Signal<u32> = use_signal(|| 0);
    rsx! {
        Counter { name: "Neil".to_string(), score: score_a , up: move |i| score_a += i, down: move |(i,v)| if v > 0 { score_a -= i } }
        Reset { scores: vec![score_a, score_b], onclick: move |_| { *score_a.write() = 0; *score_b.write() = 0 } }
        div { transform: "rotate(180deg)", Counter { name: "Marion".to_string(), score: score_b , up: move |i| score_b += i, down: move |(i,v)| if v > 0 { score_b -= i } } }

    }
}

#[component]
fn Counter(
    name: String,
    score: Signal<u32>,
    up: EventHandler<u32>,
    down: EventHandler<(u32, u32)>,
) -> Element {
    rsx! {
           div { "align": "center", "Counter for {name}"
           br { }
           "{score}"
           br { }
           button { onclick: move |_| up.call(10), "+10" }
           button { onclick: move |_| up.call(5), "+5" }
           button { onclick: move |_| up.call(1), "+1" }
           button { onclick: move |_| down.call((1,score())), "-1" }
    }
       }
}

#[component]
fn Reset(scores: Vec<Signal<u32>>, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
          div { "align": "center", button { onclick , "Reset" } }
    }
}

fn main() {
    dioxus::launch(App);
}
