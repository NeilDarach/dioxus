use dioxus::prelude::*;
mod models {
    pub mod scores;
}

use crate::models::scores::{Action, CribScore, CribScores, Player};

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
    let p1 = use_memo(move || (SCORES.read().player_1).clone());
    let p2 = use_memo(move || (SCORES.read().player_2).clone());
    fn update_score(player: Player, action: Action) {
        SCORES.write().update(player, action);
    }

    rsx! {
        document::Stylesheet { href: CSS }
        div { transform: "rotate(180deg)", Counter { player: p2, update_score: move |d| update_score(Player::PlayerTwo,Action::ChangeScore(d))} }
        Reset { onclick: move |_| { SCORES.write().update(Player::PlayerOne,Action::ResetScore); SCORES.write().update(Player::PlayerTwo,Action::ResetScore)}}
        div { Counter { player: p1, update_score: move |d| update_score(Player::PlayerOne,Action::ChangeScore(d)) } }

    }
}

#[component]
fn Counter(player: Memo<CribScore>, update_score: EventHandler<i16>) -> Element {
    rsx! {
           div { class: "title", "{player().name}" }
           br { }
           span { class: "prevscore", "{player().previous_score}" }
           span { class: "score", "{player().score}"}
           br { }
           button { class: "action", onclick: move |_| update_score.call(10), "+10" }
           button { class: "action", onclick: move |_| update_score.call(5), "+5" }
           button { class: "action", onclick: move |_| update_score.call(1), "+1" }
           button { class: "action", onclick: move |_| update_score.call(-1), "-1" }
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
