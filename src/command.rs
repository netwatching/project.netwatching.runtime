use std::time::Duration;
pub enum Command {
    Stop, // stop runner
    Housekeeping, // update housekeeping timer (=watchdog)
    SetTimeout(Duration) // update timeout between thread runs
}