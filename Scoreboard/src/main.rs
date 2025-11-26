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
    fn update(player: Player, action: Action) {
        SCORES.write().update(player, action);
    }

    rsx! {
        document::Stylesheet { href: CSS }
        div { transform: "rotate(180deg)", Counter { player: p2, update: move |a| update(Player::PlayerTwo,a)} }
        Reset { onclick: move |_| { SCORES.write().update(Player::PlayerOne,Action::ResetScore); SCORES.write().update(Player::PlayerTwo,Action::ResetScore)}}
        div { Counter { player: p1, update: move |a| update(Player::PlayerOne,a) } }

    }
}

#[component]
fn Counter(player: Memo<CribScore>, update: EventHandler<Action>) -> Element {
    let mut editing_title = use_signal(|| false);
    let id = use_signal(|| uuid::Uuid::new_v4().to_string());
    let ds_name = use_memo(move || if editing_title() { "none" } else { "block" });
    let ds_input = use_memo(move || if editing_title() { "block" } else { "none" });
    let set_focus = move |i| async move {
        let js = format!(r#" document.getElementById("{}").focus() "#, i);
        let _ = dioxus::document::eval(&js).await;
    };
    use_effect(move || {
        let editing = editing_title();
        let i = id().clone();
        if editing {
            spawn(set_focus(i));
        };
    });
    let pname = if player().name.is_empty() {
        "Anon"
    } else {
        &player().name
    };
    rsx! {
           div { class: "title", display: ds_name, onclick: move |_| { editing_title.set(true); },
                 "{pname}" }
           div { class: "title", display: ds_input,
                 input { id,
                         autocomplete: false, autocorrect: false, autocapitalize: false, spellcheck: false,
                         oninput: move |e| update.call(Action::ChangeName(e.value().clone())),
                         onkeydown: move |e| if e.key() == Key::Enter { editing_title.set(false); },
                         onfocusout: move |_| editing_title.set(false) } }
           br { }
           span { class: "prevscore", "{player().previous_score}" }
           span { class: "score", "{player().score}"}
           br { }
           button { class: "action", onclick: move |_| update.call(Action::ChangeScore(10)), "+10" }
           button { class: "action", onclick: move |_| update.call(Action::ChangeScore(5)), "+5" }
           button { class: "action", onclick: move |_| update.call(Action::ChangeScore(1)), "+1" }
           button { class: "action", onclick: move |_| update.call(Action::ChangeScore(-1)), "-1" }
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
