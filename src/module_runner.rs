use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, unbounded};
use tokio::time::sleep;
use chrono;
use chrono::{DateTime, Utc};

use crate::executor::Executor;
use crate::command::Command;

#[derive(Clone, Debug)]
pub struct ModuleRunner<OutgoingDataFormat> {
    executor: Box<dyn Executor<OutgoingDataFormat>>,
    timeout_duration: Duration,
    command_receiver: Receiver<Command>,
    pub command_sender:  Sender<Command>,
    outgoing_sender: Sender<OutgoingDataFormat>,
    graceful_stop: bool,
    last_housekeeping_message: Option<DateTime<Utc>>
}

impl<OutgoingDataFormat> ModuleRunner<OutgoingDataFormat> {

    pub fn new(executor: Box<dyn Executor<OutgoingDataFormat>>, timeout_duration: Duration, outgoing_sender: Sender<OutgoingDataFormat>) -> Self {
        let (sender, receiver) = unbounded();
        Self {
            executor,
            timeout_duration,
            command_sender: sender,
            command_receiver: receiver,
            outgoing_sender,
            graceful_stop: false,
            last_housekeeping_message: None
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
                    // Will set the last received housekeeping message
                    self.executor.on_housekeeping_message();
                    self.last_housekeeping_message = Some(chrono::Utc::now());
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

            // Execute executor. Only send messages if data is received
            if let Some(out) = self.executor.execute() {
                self.outgoing_sender.send(out).expect("TODO: panic message");
            }
            
            // TODO: state handling!

            // Housekeeping interval, ignore it if never explicitly set!
            if let Some(time) = self.last_housekeeping_message {
                // Todo: make configurable
                if Utc::now() - Duration::from_secs(300) > time {
                    self.graceful_stop = false;
                    self.executor.on_housekeeping_shutdown();
                }
            }
            if self.graceful_stop {
                break
            }
            sleep(self.timeout_duration).await;
        }
    }
}

unsafe impl<OutgoingDataFormat: Clone> Send for ModuleRunner<OutgoingDataFormat> {}
unsafe impl<OutgoingDataFormat: Clone> Sync for ModuleRunner<OutgoingDataFormat> {}