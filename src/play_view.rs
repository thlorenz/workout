use crate::{
    components::routine_table,
    msg::Msg,
    routine::{Routine, RoutineStep},
};
use seed::{prelude::*, *};

pub fn play_header_view(step: &RoutineStep) -> Node<Msg> {
    header![
        C!["play-header"],
        h1![&step.title],
        div![
            C!["routine-step-header-img-container"],
            img![
                C!["routine-step-header-img"],
                attrs!(At::Src => &step.image_url)
            ],
        ],
        div![
            input![
                C!["button"],
                attrs! {
                    At::Type => "button"
                    At::Value => "⏪️",
                },
                mouse_ev(Ev::Click, |_| Msg::RoutineReversed),
            ],
            input![
                C!["button"],
                attrs! {
                    At::Type => "button"
                    At::Value => "⏯️"
                },
                mouse_ev(Ev::Click, |_| Msg::RoutineToggled),
            ],
            input![
                C!["button"],
                attrs! {
                    At::Type => "button"
                    At::Value => "⏩️",
                },
                mouse_ev(Ev::Click, |_| Msg::RoutineForwarded),
            ],
            input![
                C!["button"],
                attrs! {
                    At::Type => "button"
                    At::Value => "⏏️"
                },
                mouse_ev(Ev::Click, |_| Msg::RoutineStopped),
            ]
        ]
    ]
}

pub fn player_main_view(routine: &Routine, step_idx: u16) -> Node<Msg> {
    let steps: Vec<&RoutineStep> = routine.steps.iter().skip(step_idx as usize + 1).collect();
    section![C!["main"], routine_table(&steps)]
}
