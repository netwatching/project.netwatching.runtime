use std::time::Duration;
pub enum IncomingMessage {
    Stop, // stop runner
    Housekeeping, // update housekeeping timer (=watchdog)
    SetTimeout(Duration) // update timeout between thread runs
}

pub enum OutgoingMessage<T> {
    CollectedData(T)
}