use std::collections::HashMap;
use std::marker::PhantomData;

use crossbeam_channel::Sender;
use rayon::prelude::*;

use crate::message::IncomingMessage;
use crate::module_runner::ModuleRunner;

pub struct Handler<DataFormat> {
    modules_sender: HashMap<u64, Sender<IncomingMessage>>,
    pd: PhantomData<DataFormat>
}

impl<DataFormat: 'static> Handler<DataFormat> {
    pub fn new() -> Self {
        Self {
            modules_sender: HashMap::new(),
            pd: PhantomData::default()
        }
    }

    /// Spawn a new ModuleRunner task and execute it immediately
    pub fn spawn(&mut self, id: u64, mut module: ModuleRunner<DataFormat>) {
        let sender = module.incoming_sender.clone();
        // TODO: do something meaningful with handle
        let _handle = tokio::spawn(async move {
            module.runner().await;
        });
        self.modules_sender.insert(id, sender);
    }

    /// Send message to specified module. Returns false if it does not exist
    pub fn send_message(&self, id: u64, message: IncomingMessage) -> bool {
        match self.modules_sender.get(&id) {
            None => {
                false
            }
            Some(sender) => {
                // TODO: program could crash here, introduce some kind of mechanism that prevents this!
                sender.send(message).expect("Could not send message!");
                true
            }
        }
    }

    /// Has to be called periodically to reset the housekeeping timer of the modules.
    /// If the thread does not receive a housekeeping message for the specified time it will end itself gracefully!
    pub fn housekeeping(&self) {
        self.modules_sender.par_iter().for_each(|(id, _)| {
           self.send_message(*id, IncomingMessage::Housekeeping);
        });
    }
}

unsafe impl<DataFormat> Send for Handler<DataFormat> {}
unsafe impl<DataFormat> Sync for Handler<DataFormat> {}