pub enum Msg {
    // Config
    RoutineTextChanged(String),
    RoutineSave,
    RoutineClear,
    RoutineLoad,
    // Play
    RoutineStarted,
    RoutineStopped,
    RoutineToggled,
    RoutineReversed,
    RoutineForwarded,
    RoutineRestarted,
    OnTick,
}
