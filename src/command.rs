use std::time::Duration;
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub enum Command {
    Stop, // stop runner
    Housekeeping, // update housekeeping timer (=watchdog)
    SetTimeout(Duration) // update timeout between thread runs
}