use dioxus::prelude::*;

mod models {
    pub mod scores;
}

use crate::models::scores::CribScores;

static CSS: Asset = asset!("/assets/main.css");
static SCORES: GlobalSignal<CribScores> =
    Global::new(|| CribScores::new().expect("Failed to load crib scores"));

#[component]
fn App() -> Element {
    #[cfg(feature = "mobile")]
    // Disable the idle timer on iphones
    {
        let mtm = objc2::MainThreadMarker::new().unwrap();
        let app = objc2_ui_kit::UIApplication::sharedApplication(mtm);
        app.setIdleTimerDisabled(true);
    }
    let p1_name = use_memo(move || SCORES.read().player_1_name.clone());
    let p2_name = use_memo(move || SCORES.read().player_2_name.clone());
    let mut score_a = use_memo(move || SCORES.read().player_1_score);
    let mut score_b = use_memo(move || SCORES.read().player_2_score);
    rsx! {
        document::Stylesheet { href: CSS }
        div { transform: "rotate(180deg)", Counter { name: p2_name, score: score_b , up: move |(i,v)| if v+i > 121 { *score_b.write() = 121 } else { score_b += i }, down: move |(i,v)| if v > 0 { score_b -= i } } }
        Reset { onclick: move |_| { SCORES.write().player_1_score = 0; SCORES.write().player_2_score = 0 } }
        div { Counter { name: p1_name, score: score_a , up: move |(i,v)| if v+i > 121 { *score_a.write() = 121 } else { score_a += i }, down: move |(i,v)| if v > 0 { score_a -= i } } }

    }
}

#[component]
fn Counter(
    name: Memo<String>,
    score: Memo<u16>,
    up: EventHandler<(u16, u16)>,
    down: EventHandler<(u16, u16)>,
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
fn Reset(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
          div { button { class: "reset", onclick , "Reset" } }
    }
}

fn main() {
    dioxus::launch(App);
}
