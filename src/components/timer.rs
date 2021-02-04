use std::f32::consts::PI;

use seed::{prelude::*, *};

use crate::msg::Msg;

const FULL_DASH_ARRAY: f32 = 2.0 * PI * 45.0;

pub fn timer(total_time: u16, time_remaining: u16) -> Node<Msg> {
    let time_elapsed = total_time - time_remaining;
    let perc_elapsed = (time_elapsed as f32 + 0.5) / total_time as f32;
    let fraction = FULL_DASH_ARRAY * perc_elapsed;

    div![
        C!["timer"],
        svg![
            C!["base-timer__svg"],
            attrs!(
                At::ViewBox => "0 0 100 100",
                At::Xmlns => "http://www.w3.org/2000/svg",
            ),
            g![
                C!["base-timer__circle"],
                path![
                    C!["base-timer__path-remaining"],
                    attrs!(
                        At::StrokeDashArray=>
                            &format!("{:.0} {:.0}", fraction.ceil(), FULL_DASH_ARRAY),
                        At::D => "M 50, 50 m -45, 0 a 45,45 0 1,0 90,0 a 45,45 0 1,0 -90,0"
                    ),
                ]
            ],
        ],
        span![
            C!["base-timer__label"],
            &format!("{:02}", time_remaining - 1)
        ]
    ]
}
