#![allow(clippy::wildcard_imports)]
// TODO(thlorenz): remove
#![allow(dead_code, unused_variables, unused_imports)]

use iced::{
    button, scrollable, text_input, Align, Application, Button, Column, Command, Container,
    Element, HorizontalAlignment, Length, Row, Scrollable, Settings, Text, TextInput,
};
use icons::edit_icon;
use persistence::{LoadError, SavedState};
use routine::{Routine, RoutineStep};
use sample::SAMPLE;

mod icons;
mod persistence;
mod routine;
mod sample;
mod style;

pub fn main() -> iced::Result {
    Workout::run(Settings::default())
}

#[derive(Debug, Default)]
struct ConfigScreen {
    routine_text: String,
    routine: Routine,
    routine_steps: Vec<RoutineStepItem>,

    scroll: scrollable::State,
    input: text_input::State,
}

#[derive(Debug, Default)]
struct State {
    config_data: ConfigScreen,
}

enum Workout {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone)]
enum Msg {
    Loaded(Result<SavedState, LoadError>),
    RoutineTextChanged(String),

    RoutineStepMsg(usize, RoutineStepMsg),
}

impl Application for Workout {
    type Message = Msg;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Workout, Command<Msg>) {
        (
            Self::Loading,
            Command::perform(SavedState::load(), Msg::Loaded),
        )
    }

    fn title(&self) -> String {
        match self {
            Workout::Loading => "Workout *".to_string(),
            Workout::Loaded(_) => "Workout".to_string(),
        }
    }

    fn update(&mut self, message: Msg) -> Command<Msg> {
        match self {
            Workout::Loading => {
                match message {
                    Msg::Loaded(Ok(state)) => {
                        let (routine_text, routine) = (
                            state.routine_text.clone(),
                            Routine::from(state.routine_text.as_str()),
                        );
                        let routine_steps = routine
                            .steps
                            .iter()
                            .map(|&step| RoutineStepItem::from(step.clone()))
                            .collect();
                        *self = Workout::Loaded(State {
                            config_data: ConfigScreen {
                                routine_text,
                                routine,
                                routine_steps,
                                ..Default::default()
                            },
                        });
                    }
                    Msg::Loaded(Err(_)) => {
                        // TODO: keep this empty and only populate via sample when user "loads"
                        // without ever saving anything
                        let (routine_text, routine) = (SAMPLE.to_string(), Routine::from(SAMPLE));
                        *self = Workout::Loaded(State {
                            config_data: ConfigScreen {
                                routine_text,
                                routine,
                                ..Default::default()
                            },
                        });
                    }
                    _ => {}
                }
                Command::none()
            }
            Workout::Loaded(_) => Command::none(),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        match self {
            Workout::Loading => loading_message(),
            // TODO: separate into config/play screens
            Workout::Loaded(State {
                config_data:
                    ConfigScreen {
                        routine_text,
                        routine,
                        routine_steps,
                        scroll,
                        input,
                    },
            }) => {
                let title = Text::new("workout")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);

                // TODO: broken as it doesn't support multiline text
                let input = TextInput::new(
                    input,
                    "title;https://img-url.png;duration-secs;rest-secs;",
                    routine_text,
                    Msg::RoutineTextChanged,
                )
                .padding(15)
                .size(30);

                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title)
                    .push(input);

                let steps: Element<_> = if routine.has_steps() {
                    routine
                        .steps
                        .iter()
                        .enumerate()
                        .fold(Column::new().spacing(20), |column, (i, step)| todo!())
                        .into()
                } else {
                    empty_message("Please load/add some steps above.")
                };

                Scrollable::new(scroll)
                    .padding(40)
                    .push(Container::new(content).width(Length::Fill).center_x())
                    .into()
            }
        }
    }
}

fn loading_message<'a>() -> Element<'a, Msg> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

fn empty_message<'a>(message: &str) -> Element<'a, Msg> {
    Container::new(
        Text::new(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(HorizontalAlignment::Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Units(200))
    .center_y()
    .into()
}

//
// Routine Step item in List which is editable
//

#[derive(Debug)]
enum RoutineStepState {
    Idle {
        edit_button: button::State,
    },
    Editing {
        text_input: text_input::State,
        delete_button: button::State,
    },
}

impl Default for RoutineStepState {
    fn default() -> Self {
        Self::Idle {
            edit_button: button::State::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum RoutineStepMsg {
    Edit,
    DescriptionEdited(String),
    Completed(bool),
    Delete,
}

#[derive(Debug, Default)]
struct RoutineStepItem {
    step: RoutineStep,
    state: RoutineStepState,
}

impl From<RoutineStep> for RoutineStepItem {
    fn from(step: RoutineStep) -> Self {
        Self {
            step,
            state: Default::default(),
        }
    }
}

impl RoutineStepItem {
    fn new(step: RoutineStep, edit_button: button::State) -> Self {
        Self {
            step,
            state: RoutineStepState::Idle { edit_button },
        }
    }

    fn view(&mut self) -> Element<RoutineStepMsg> {
        match &mut self.state {
            RoutineStepState::Idle { edit_button } => Row::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(
                    Button::new(edit_button, edit_icon())
                        .on_press(RoutineStepMsg::Edit)
                        .padding(10)
                        .style(style::Button::Icon),
                )
                .into(),
            RoutineStepState::Editing {
                text_input,
                delete_button,
            } => {
                todo!()
            }
        }
    }
}
