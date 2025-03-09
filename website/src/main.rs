use cross_language_prop_types::CrossCompiler;
use leptos::{ev, html::*, prelude::*};

const INITIAL_VALUE: &'static str = include_str!("./initial-value.clpt");

pub fn app() -> impl IntoView {
    let (input, set_input) = signal(INITIAL_VALUE.to_string());
    let parsed =
        Memo::new(move |_| format!("{:?}", CrossCompiler::parse_and_validate(&input.get())));
    div().child((
        h2().child("Input"),
        textarea()
            .child(input.get_untracked())
            .prop("value", move || input.get())
            .on_target(ev::input, move |ev| set_input.set(ev.target().value())),
        h2().child("Status"),
        pre().child(parsed),
    ))
}

fn main() {
    leptos::mount::mount_to_body(|| app())
}
