use crate::{
    components::{routine_table, timer},
    msg::Msg,
    routine::{Routine, RoutineStep},
    PlayData,
};
use seed::{prelude::*, *};

pub fn play_header_view(play_data: &PlayData, step: &RoutineStep) -> Node<Msg> {
    let total_time = if play_data.is_resting {
        step.rest.unwrap_or_default()
    } else {
        step.duration
    };

    header![
        C!["play-header"],
        div![
            C!["play-header-step"],
            div![
                C!["play-header-step-title"],
                IF!(!play_data.is_resting => h3![&step.title]),
                IF!(play_data.is_resting => p![
                    span![
                    C!["play-header-step-resting-preamble"],
                    "Resting, next up"],
                    h3![
                        C!["play-header-step-resting-title"],
                        &step.title
                    ]
                ]),
            ],
            div![
                C!["play-header-step-img-container"],
                img![
                    C!["play-header-step-img"],
                    attrs!(At::Src => &step.image_url)
                ],
            ],
            timer(total_time, play_data.time_remaining),
        ],
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
    ]
}

pub fn player_main_view(routine: &Routine, step_idx: u16) -> Node<Msg> {
    let steps: Vec<&RoutineStep> = routine.steps.iter().skip(step_idx as usize + 1).collect();
    section![C!["main"], routine_table(&steps)]
}

fn timer_view(data: &PlayData) -> Node<Msg> {
    p![C!["timer"], data.time_remaining.to_string()]
}
