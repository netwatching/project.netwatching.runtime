use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, unbounded};
use tokio::time::sleep;

use crate::executor::Executor;
use crate::command::Command;

pub struct ModuleRunner<DataFormat> {
    executor: Box<dyn Executor<DataFormat>>,
    timeout_duration: Duration,
    command_receiver: Receiver<Command>,
    pub command_sender:  Sender<Command>,
    outgoing_sender: Sender<DataFormat>,
    graceful_stop: bool,
    // TODO: introduce last_housekeeping_message
}

impl<DataFormat> ModuleRunner<DataFormat> {

    pub fn new(executor: Box<dyn Executor<DataFormat>>, timeout_duration: Duration, outgoing_sender: Sender<DataFormat>) -> Self {
        let (sender, receiver) = unbounded();
        Self {
            executor,
            timeout_duration,
            command_sender: sender,
            command_receiver: receiver,
            outgoing_sender,
            graceful_stop: false,
        }
    }

    fn handle_incoming_messages(&mut self) {
        // Handle messages, non-blocking. If multiple messages arrived, all of them will be executed
        self.command_receiver.try_iter().for_each(|message| {
            match message {
                Command::Stop => {
                    //
                    self.graceful_stop = true;
                    self.executor.on_stop();
                }
                Command::Housekeeping => {
                    // TODO: implement
                    // Will set the last received housekeeping message
                    self.executor.on_housekeeping_message();
                    panic!("Not implemented yet")
                }
                Command::SetTimeout(new_duration) => {
                    self.executor.on_timeout_change(new_duration);
                    self.timeout_duration = new_duration;
                }
            }
        });
    }

    /// Handles the messages, pausing and execution of the thread
    pub async fn runner(&mut self) {
        // TODO: state handling, sending messages, communicating with the Thread itself
        loop {
            self.handle_incoming_messages();
            match self.executor.execute() {
                None => {}
                Some(out) => {
                    self.outgoing_sender.send(out).expect("TODO: panic message");
                }
            }
            
            // TODO: state handling!

            // TODO: Check Housekeeping interval. If limit exceeded, set graceful stop
            if self.graceful_stop {
                break
            }
            sleep(self.timeout_duration).await;
        }
    }
}

unsafe impl<DataFormat> Send for ModuleRunner<DataFormat> {}
unsafe impl<DataFormat> Sync for ModuleRunner<DataFormat> {}