#![allow(clippy::wildcard_imports)]
// TODO(thlorenz): remove
#![allow(dead_code, unused_variables)]

use audio::Audio;
use config_view::{config_header_view, config_main_view};
use msg::Msg;
use play_view::{play_header_view, player_main_view};
use routine::Routine;
use seed::{prelude::*, *};
use web_sys::HtmlInputElement;

mod audio;
mod components;
mod config_view;
mod msg;
mod play_view;
mod routine;

const ENTER_KEY: u32 = 13;
const ESC_KEY: u32 = 27;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    console_error_panic_hook::set_once();

    orders.stream(streams::interval(500, || Msg::OnTick));

    let text = "\
crunch;https://hips.hearstapps.com/hmg-prod.s3.amazonaws.com/images/crunch-1588842220.jpg;10;5;
left crunch;https://www.wikihow.com/images/thumb/7/75/Do-a-Side-Crunch-Step-4-Version-2.jpeg/aid2055959-v4-728px-Do-a-Side-Crunch-Step-4-Version-2.jpeg;10;5;
right crunch;https://www.wikihow.com/images/thumb/f/fc/Do-a-Side-Crunch-Step-3-Version-2.jpeg/aid2055959-v4-728px-Do-a-Side-Crunch-Step-3-Version-2.jpeg;10;5;
russian twist;https://www.snapfitness.com/assets/_blog/images/2013-nov-13-1113-workout-content1.jpg;10;5
plank;https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ-NLh0Gt0LJ2bfP8L0CAmIr6u5vmeqVAREbA&usqp=CAU;10;5
";
    let routine = Routine::from(text);

    Model {
        config_data: ConfigData {
            routine_text: text.to_string(),
            routine,
            ..Default::default()
        },
        play_data: PlayData {
            ..Default::default()
        },
        screen: Screen::Config,
        refs: Refs::default(),
        audio: None,
        is_front_of_tick: true,
    }
}

struct Model {
    config_data: ConfigData,
    play_data: PlayData,
    screen: Screen,
    audio: Option<Audio>,
    refs: Refs,
    is_front_of_tick: bool,
}

#[derive(Default)]
struct ConfigData {
    routine_text: String,
    routine: Routine,
}

#[derive(Default)]
pub struct PlayData {
    step_idx: u16,
    time_remaining: u16,
    is_resting: bool,
    is_paused: bool,
}

enum Screen {
    Config,
    Play,
}

#[derive(Default)]
struct Refs {
    routine_text_input: ElRef<HtmlInputElement>,
}

fn reset_step(data: &mut PlayData) {
    data.time_remaining = 5;
    data.is_resting = true;
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let mut data = &mut model.config_data;
    match msg {
        // Config
        Msg::RoutineTextSubmitted => data.routine = Routine::from(data.routine_text.as_str()),
        Msg::RoutineTextChanged(text) => data.routine_text = text,

        // Play
        Msg::RoutineRestarted | Msg::RoutineStarted => {
            let mut play_data = &mut model.play_data;
            play_data.step_idx = 0;
            play_data.time_remaining = 5;
            play_data.is_paused = false;
            play_data.is_resting = true;
            model.screen = Screen::Play;
            if model.audio.is_none() {
                model.audio = Audio::new().ok();
            }
        }
        Msg::RoutineStopped => model.screen = Screen::Config,
        Msg::RoutineReversed => {
            reset_step(&mut model.play_data);
            if model.play_data.step_idx > 0 {
                model.play_data.step_idx -= 1;
            }
        }
        Msg::RoutineForwarded => {
            reset_step(&mut model.play_data);
            if model.play_data.step_idx as usize + 1 < model.config_data.routine.nsteps() {
                model.play_data.step_idx += 1;
            }
        }
        Msg::RoutineToggled => {
            model.play_data.is_paused = !model.play_data.is_paused;
            if let Some(audio) = &model.audio {
                audio.stop();
            }
        }

        Msg::OnTick if !model.play_data.is_paused => {
            if model.is_front_of_tick {
                let mut data = &mut model.play_data;
                if data.time_remaining > 1 {
                    data.time_remaining -= 1;
                } else {
                    if data.is_resting {
                        let step = model.config_data.routine.get(data.step_idx);
                        data.time_remaining = step.duration;
                        data.is_resting = false;
                    } else {
                        let step = model.config_data.routine.get(data.step_idx);
                        data.time_remaining = step.rest.unwrap_or_default();
                        data.is_resting = true;

                        // Determine and show next step
                        let nsteps = model.config_data.routine.nsteps();
                        if (data.step_idx as usize) < nsteps - 1 {
                            data.step_idx += 1;
                        } else {
                            model.screen = Screen::Config;
                        }
                    }
                }

                if let Some(audio) = &model.audio {
                    match data.time_remaining {
                        x if 1 < x && x <= 4 => {
                            audio.play(440.0);
                        }
                        x if x == 1 => {
                            audio.play(880.0);
                        }
                        _ => audio.stop(),
                    }
                }
            } else {
                if let Some(audio) = &model.audio {
                    audio.stop();
                }
            }
            model.is_front_of_tick = !model.is_front_of_tick
        }
        Msg::OnTick => {}
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    match model.screen {
        Screen::Config => {
            let data = &model.config_data;
            nodes![
                config_header_view(&data.routine_text),
                config_main_view(&data.routine)
            ]
        }
        Screen::Play => {
            let step = model.config_data.routine.get(model.play_data.step_idx);
            nodes![
                play_header_view(&model.play_data, step),
                player_main_view(&model.config_data.routine, model.play_data.step_idx)
            ]
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    App::start("app", init, update, view);
    Ok(())
}
