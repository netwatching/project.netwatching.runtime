use std::time::Duration;

use async_trait::async_trait;

#[async_trait]
pub trait Executor<OutgoingDataFormat> {

    /// Main code will be executed here!
    fn execute(&self) -> Option<OutgoingDataFormat>;

    /// Will be called when stop is received
    fn on_stop(&mut self) {

    }

    /// Will be called when message is received
    fn on_message(&mut self) {

    }

    /// Will be called when housekeeping message is received
    fn on_housekeeping_message(&mut self) {

    }

    /// Will be called if housekeeping interval could not be satisfied
    fn on_housekeeping_shutdown(&mut self) {

    }

    /// Change timeout duration
    fn on_timeout_change(&mut self, _new_duration: Duration) {

    }
}