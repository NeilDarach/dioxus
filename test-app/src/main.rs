use dioxus::prelude::*;
use objc2::{msg_send, runtime::AnyObject, MainThreadMarker};
use objc2_ui_kit::{UIApplication, UIApplicationDelegate};

static CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    let mut score_a: Signal<u32> = use_signal(|| 0);
    let mut score_b: Signal<u32> = use_signal(|| 0);
    rsx! {
        document::Stylesheet { href: CSS }
        div { transform: "rotate(180deg)", Counter { name: "Marion".to_string(), score: score_b , up: move |(i,v)| if v+i > 121 { *score_b.write() = 121 } else { score_b += i }, down: move |(i,v)| if v > 0 { score_b -= i } } }
        Reset { scores: vec![score_a, score_b], onclick: move |_| { *score_a.write() = 0; *score_b.write() = 0 } }
        div { Counter { name: "Neil".to_string(), score: score_a , up: move |(i,v)| if v+i > 121 { *score_a.write() = 121 } else { score_a += i }, down: move |(i,v)| if v > 0 { score_a -= i } } }

    }
}

#[component]
fn Counter(
    name: String,
    score: Signal<u32>,
    up: EventHandler<(u32, u32)>,
    down: EventHandler<(u32, u32)>,
) -> Element {
    let current = score();
    rsx! {
           div { class: "title", "Counter for {name}"
           br { }
           span { class: "score", "{score}"}
           br { }
           button { class: "action", onclick: move |_| up.call((10,current)), "+10" }
           button { class: "action", onclick: move |_| up.call((5,current)), "+5" }
           button { class: "action", onclick: move |_| up.call((1,current)), "+1" }
           button { class: "action", onclick: move |_| down.call((1,current)), "-1" }
    }
       }
}

#[component]
fn Reset(scores: Vec<Signal<u32>>, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
          div { button { class: "reset", onclick , "Reset" } }
    }
}

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    let app = UIApplication::sharedApplication(mtm);
    app.setIdleTimerDisabled(true);
    dioxus::launch(App);
}
