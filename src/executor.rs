use std::fmt::Debug;
use std::time::Duration;

use dyn_clone::DynClone;

pub trait Executor<OutgoingDataFormat>: DynClone + Debug {

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

// Works like witchcraft, no clue how this works :D
dyn_clone::clone_trait_object!(<OutgoingDataFormat> Executor<OutgoingDataFormat>);
