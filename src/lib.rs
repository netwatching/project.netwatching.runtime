pub mod handler;
pub mod module_runner;
pub mod executor;
pub mod message;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crossbeam_channel::unbounded;
    use log::{info, warn};

    use crate::executor::Executor;
    use crate::handler::Handler;
    use crate::message::{IncomingMessage, OutgoingMessage};
    use crate::module_runner::ModuleRunner;

    struct TestExecutor {
        name: String
    }

    impl Executor<u32> for TestExecutor {
        fn execute(&self) -> u32 {
            info!("{} is currently working....", self.name);
            0
        }

        fn on_stop(&mut self) {
            warn!("Gracefully stopped {}", self.name)
        }

        fn on_message(&mut self) {
            todo!()
        }

        fn on_housekeeping_message(&mut self) {
            todo!()
        }
    }


    #[tokio::test]
    async fn create_handler() {
        tracing_subscriber::fmt::init();
        let mut handler = Handler::new();
        let (sender, _receiver) = unbounded::<OutgoingMessage<u32>>();
        let module = ModuleRunner::<u32>::new(Box::new(TestExecutor {name: String::from("Slow worker")}), Duration::from_secs(1), sender.clone());
        let module2 = ModuleRunner::<u32>::new(Box::new(TestExecutor {name: String::from("Faster worker")}), Duration::from_secs(2), sender);
        handler.spawn(0, module);
        handler.spawn(1, module2);
        tokio::time::sleep(Duration::from_secs(5)).await;
        handler.send_message(0, IncomingMessage::Stop);
        tokio::time::sleep(Duration::from_secs(5)).await;
        _receiver.try_iter().for_each(|message| {
            match message {
                OutgoingMessage::CollectedData(data) => {
                    info!("Received message {data}")
                }
            }
        });
    }
}
