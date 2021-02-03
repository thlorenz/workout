pub enum Msg {
    RoutineTextChanged(String),
    RoutineTextSubmitted,
    RoutineStarted,
    RoutineStopped,
    RoutineToggled,
    RoutineReversed,
    RoutineForwarded,
    RoutineRestarted,
    OnTick,
}
