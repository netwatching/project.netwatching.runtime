pub mod handler;
pub mod module_runner;
pub mod executor;
pub mod command;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crossbeam_channel::unbounded;
    use log::{info, warn};

    use crate::executor::Executor;
    use crate::handler::Handler;
    use crate::module_runner::ModuleRunner;

    #[derive(Debug, Clone)]
    struct TestExecutor {
        name: String
    }

    impl Executor<u32> for TestExecutor {
        fn execute(&self) -> Option<u32> {
            info!("{} is currently working....", self.name);
            Some(0)
        }

        fn on_stop(&mut self) {
            warn!("Gracefully stopped {}", self.name)
        }
    }


    #[tokio::test]
    async fn create_handler() {
        tracing_subscriber::fmt::init();
        let mut handler = Handler::new();
        let (sender, _receiver) = unbounded::<u32>();
        let module = ModuleRunner::<u32>::new(Box::new(TestExecutor {name: String::from("Slow worker")}), Duration::from_secs(1), sender.clone());
        let module2 = ModuleRunner::<u32>::new(Box::new(TestExecutor {name: String::from("Faster worker")}), Duration::from_secs(2), sender);
        handler.spawn(0, module);
        for i in 1..20 {
            handler.spawn(i, module2.clone());
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
        handler.send_stop_message(0);
        tokio::time::sleep(Duration::from_secs(5)).await;
        _receiver.try_iter().for_each(|message| {
            info!("Received message {message}");
        });
    }
}
