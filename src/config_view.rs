use crate::{components::routine_table, routine::Routine};
use seed::{prelude::*, *};

use crate::msg::Msg;

pub fn config_header_view(routine_text: &str) -> Node<Msg> {
    header![
        C!["header"],
        h1![C!["header-title"], "Workout Routine"],
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
        div![
            C!["config-buttons"],
            input![
                C!["button"],
                attrs! {
                    At::Type => "button"
                    At::Value => "Load Sample"
                },
                mouse_ev(Ev::Click, |_| Msg::RoutineLoadSample),
            ],
            input![
                C!["button"],
                attrs! {
                    At::Type => "button"
                    At::Value => "▶️",
                },
                mouse_ev(Ev::Click, |_| Msg::RoutineStarted),
            ]
        ]
    ]
}

pub fn config_main_view(routine: &Routine) -> Node<Msg> {
    section![C!["main"], routine_table(&routine.steps())]
}
