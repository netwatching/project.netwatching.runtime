use std::collections::HashMap;
use std::marker::PhantomData;

use crossbeam_channel::Sender;
use rayon::prelude::*;
use tokio::net::windows::named_pipe::PipeMode::Message;

use crate::command::Command;
use crate::module_runner::ModuleRunner;

pub struct Handler<OutgoingDataFormat> {
    modules_sender: HashMap<u64, Sender<Command>>,
    pd: PhantomData<OutgoingDataFormat>
}

impl<OutgoingDataFormat: 'static> Handler<OutgoingDataFormat> {
    pub fn new() -> Self {
        Self {
            modules_sender: HashMap::new(),
            pd: PhantomData::default()
        }
    }

    /// Spawn a new ModuleRunner task and execute it immediately
    pub fn spawn(&mut self, id: u64, mut module: ModuleRunner<OutgoingDataFormat>) {
        let sender = module.command_sender.clone();
        // TODO: do something meaningful with handle
        let _handle = tokio::spawn(async move {
            module.runner().await;
        });
        self.modules_sender.insert(id, sender);
    }

    /// Send message to specified module. Returns false if it does not exist
    /// If stop message is sent, the reference to will be deleted and the thread will shut down gracefully after another execution
    fn send_message(&self, id: u64, message: Command) -> bool {
        match self.modules_sender.get(&id) {
            None => {
                false
            }
            Some(sender) => {
                // TODO: program could crash here, introduce some kind of mechanism that prevents this!
                sender.send(message).is_ok()
            }
        }
    }

    pub fn send_stop_message(&mut self, id: u64) -> bool {
        if self.send_message(id, Command::Stop) {
            self.modules_sender.remove(&id);
            return true;
        }
        false
    }

    /// Has to be called periodically to reset the housekeeping timer of the modules.
    /// If the thread does not receive a housekeeping message for the specified time it will end itself gracefully!
    /// Note that as long as this function has never been called the mechanism is disabled!
    pub fn housekeeping(&self) {
        self.modules_sender.par_iter().for_each(|(id, _)| {
           self.send_message(*id, Command::Housekeeping);
        });
    }
}

unsafe impl<OutgoingDataFormat> Send for Handler<OutgoingDataFormat> {}
unsafe impl<OutgoingDataFormat> Sync for Handler<OutgoingDataFormat> {}