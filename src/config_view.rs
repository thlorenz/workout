use crate::{components::routine_table, routine::Routine};
use seed::{prelude::*, *};

use crate::msg::Msg;

pub fn config_header_view(routine_text: &str) -> Node<Msg> {
    header![
        C!["header"],
        h1!["workout routine"],
        textarea![
            C!["routine-input"],
            attrs! {
                At::Placeholder => "title;https://img-url.png;duration-secs;rest-secs;";
                At::AutoFocus => true.as_at_value();
                At::Value => routine_text;
                At::SpellCheck => false,
            },
            input_ev(Ev::Input, Msg::RoutineTextChanged),
        ],
        input![
            C!["button"],
            attrs! {
                At::Type => "button"
                At::Value => "submit"
            },
            mouse_ev(Ev::Click, |_| Msg::RoutineTextSubmitted),
        ],
        input![
            C!["button"],
            attrs! {
                At::Type => "button"
                At::Value => "▶",
            },
            mouse_ev(Ev::Click, |_| Msg::RoutineStarted),
        ]
    ]
}

pub fn config_main_view(routine: &Routine) -> Node<Msg> {
    section![C!["main"], routine_table(&routine.steps())]
}
