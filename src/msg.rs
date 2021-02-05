pub enum Msg {
    // Config
    RoutineTextChanged(String),
    RoutineLoadSample,
    // Play
    RoutineStarted,
    RoutineStopped,
    RoutineToggled,
    RoutineReversed,
    RoutineForwarded,
    RoutineRestarted,
    OnTick,
}
