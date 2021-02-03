use crate::{msg::Msg, routine::RoutineStep};
use seed::{prelude::*, *};

pub fn routine_table(steps: &[&RoutineStep]) -> Node<Msg> {
    table![
        C!["routine-steps"],
        tbody![steps.iter().map(|step| { routine_row(&step) })]
    ]
}

fn routine_row(step: &RoutineStep) -> Node<Msg> {
    tr![
        td![C!["routine-step-label"], label![&step.title],],
        td![img![
            C!["routine-step-img"],
            attrs!(At::Src => &step.image_url)
        ]],
        td![
            C!["routine-step-duration"],
            label![&format!(
                "{}s / {}s",
                step.duration,
                step.rest.unwrap_or_default()
            )],
        ],
    ]
}
