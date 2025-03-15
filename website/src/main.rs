use cross_language_prop_types::CrossCompiler;
use leptos::{ev, html::*, prelude::*};

const INITIAL_VALUE: &'static str = include_str!("./initial-value.clpt");

pub fn app() -> impl IntoView {
    let (input, set_input) = signal(INITIAL_VALUE.to_string());
    let parsed = Memo::new(move |_| {
        let input = input.get();
        let cross_compiler = CrossCompiler::parse_and_validate(&input);
        let parse_status = format!("{:?}", &cross_compiler);
        (
            parse_status,
            cross_compiler.ok().map_or(String::new(), |c| c.to_java()),
        )
    });
    div().child((
        h2().child("Input"),
        textarea()
            .child(input.get_untracked())
            .prop("value", move || input.get())
            .prop("rows", 4)
            .on_target(ev::input, move |ev| set_input.set(ev.target().value())),
        h2().child("Status"),
        pre().child(move || parsed.get().0),
        h2().child("Java"),
        pre().child(move || parsed.get().1),
    ))
}

fn main() {
    leptos::mount::mount_to_body(|| app())
}
