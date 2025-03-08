use leptos::{ev, html::*, prelude::*};

const INITIAL_VALUE: &'static str = include_str!("./initial-value.clpt");

pub fn app() -> impl IntoView {
    let (input, set_input) = signal(INITIAL_VALUE.to_string());
    div().child(
        (textarea()
            .child(input.get_untracked())
            .prop("value", move || input.get())
            .on_target(ev::input, move |ev| set_input.set(ev.target().value()))),
    )
}

fn main() {
    leptos::mount::mount_to_body(|| app())
}
