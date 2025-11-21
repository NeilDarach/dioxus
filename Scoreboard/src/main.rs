use dioxus::prelude::*;

mod models {
    pub mod scores;
}

use crate::models::scores::{Action, CribScores, Player};

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
    /*
        SCORES
            .write()
            .update(Player::PlayerOne, Action::ChangeName("Neil".to_owned()));
        SCORES
            .write()
            .update(Player::PlayerTwo, Action::ChangeName("Marion".to_owned()));
    */
    let p1_name = use_memo(move || SCORES.read().player_1_name.clone());
    let p2_name = use_memo(move || SCORES.read().player_2_name.clone());
    let p1_score = use_memo(move || SCORES.read().player_1_score);
    let p2_score = use_memo(move || SCORES.read().player_2_score);
    fn update_score(player: Player, action: Action) {
        SCORES.write().update(player, action);
    }

    rsx! {
        document::Stylesheet { href: CSS }
        div { transform: "rotate(180deg)", Counter { name: p2_name, score: p2_score , update_score: move |d| update_score(Player::PlayerTwo,Action::ChangeScore(d))} }
        Reset { onclick: move |_| { SCORES.write().player_1_score = 0; SCORES.write().player_2_score = 0 } }
        div { Counter { name: p1_name, score: p1_score , update_score: move |d| update_score(Player::PlayerOne,Action::ChangeScore(d)) } }

    }
}

#[component]
fn Counter(name: Memo<String>, score: Memo<u16>, update_score: EventHandler<i16>) -> Element {
    rsx! {
           div { class: "title", "{name}"
           br { }
           span { class: "score", "{score}"}
           br { }
           button { class: "action", onclick: move |_| update_score.call(10), "+10" }
           button { class: "action", onclick: move |_| update_score.call(5), "+5" }
           button { class: "action", onclick: move |_| update_score.call(1), "+1" }
           button { class: "action", onclick: move |_| update_score.call(-1), "-1" }
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
