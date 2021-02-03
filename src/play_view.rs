use crate::{
    components::routine_table,
    msg::Msg,
    routine::{Routine, RoutineStep},
    PlayData,
};
use seed::{prelude::*, *};

pub fn play_header_view(play_data: &PlayData, step: &RoutineStep) -> Node<Msg> {
    header![
        C!["play-header"],
        div![
            input![
                C!["button"],
                attrs! {
                    At::Type => "button"
                    At::Value => "🔄",
                },
                mouse_ev(Ev::Click, |_| Msg::RoutineRestarted),
            ],
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
        ],
        h1![&step.title],
        div![
            C!["play-header-top"],
            div![
                C!["routine-step-header-img-container"],
                img![
                    C!["routine-step-header-img"],
                    attrs!(At::Src => &step.image_url)
                ],
            ],
            timer_view(play_data),
        ],
    ]
}

pub fn player_main_view(routine: &Routine, step_idx: u16) -> Node<Msg> {
    let steps: Vec<&RoutineStep> = routine.steps.iter().skip(step_idx as usize + 1).collect();
    section![C!["main"], routine_table(&steps)]
}

fn timer_view(data: &PlayData) -> Node<Msg> {
    p![C!["timer"], data.time_remaining.to_string()]
}
