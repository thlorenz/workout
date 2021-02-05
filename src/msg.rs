pub enum Msg {
    // Config
    RoutineTextChanged(String),
    RoutineTextSubmitted,
    // Play
    RoutineStarted,
    RoutineStopped,
    RoutineToggled,
    RoutineReversed,
    RoutineForwarded,
    RoutineRestarted,
    OnTick,
}
