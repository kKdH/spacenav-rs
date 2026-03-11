use crate::client::command::Command;
use crate::client::ffi::spawn_ffi_thread;
use libspnav::OpenError;
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, info};

pub struct SpaceNavClient {
    command_sender: Option<mpsc::Sender<Command>>,
    shutdown_sender: Option<oneshot::Sender<()>>,
    ffi_thread: Option<std::thread::JoinHandle<()>>,
}

impl SpaceNavClient {

    pub fn create() -> Self {

        info!("Creating SpaceNav client.");

        let (command_sender, command_receiver) = mpsc::channel(64);
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        let ffi_thread = spawn_ffi_thread(command_receiver, shutdown_receiver);

        Self {
            command_sender: Some(command_sender),
            shutdown_sender: Some(shutdown_sender),
            ffi_thread: Some(ffi_thread),
        }
    }

    pub fn open(&self, name: impl Into<String>) -> impl Future<Output=Result<(), ()>> + 'static {

        let sender = Clone::clone(&self.command_sender)
            .expect("Sender should be valid until shutdown");
        let (reply_sender, reply_receiver) = oneshot::channel();
        let name = name.into();

        debug!("Opening connection.");

        async move {
            sender.send(Command::new_command_open(name, reply_sender)).await
                .map_err(|_| ())?;
            reply_receiver.await
                .map(|result| result.map_err(|_| ()))
                .map_err(|_| ())?
        }
    }

    pub fn close(&self) -> impl Future<Output=Result<(), ()>> + 'static {

        let sender = Clone::clone(&self.command_sender)
            .expect("Sender should be valid until shutdown");
        let (reply_sender, reply_receiver) = oneshot::channel();

        debug!("Closing connection.");

        async move {
            sender.send(Command::new_command_close(reply_sender)).await
                .map_err(|_| ())?;
            reply_receiver.await
                .map(|result| result.map_err(|_| ()))
                .map_err(|_| ())?
        }
    }


    pub fn subscribe(&self, subscriber: mpsc::Sender<libspnav::Event>) -> impl Future<Output=Result<(), ()>> + 'static {

        let sender = Clone::clone(&self.command_sender)
            .expect("Sender should be valid until shutdown");
        let (reply_sender, reply_receiver) = oneshot::channel();

        debug!("Subscribing to events.");

        async move {
            sender.send(Command::new_command_subscribe(subscriber, reply_sender)).await
                .map_err(|_| ())?;
            reply_receiver.await
                .map(|result| result.map_err(|_| ()))
                .map_err(|_| ())?
        }
    }

    pub fn get_device(&self) -> impl Future<Output=Result<libspnav::Device, ()>> + 'static {

        let sender = Clone::clone(&self.command_sender)
            .expect("Sender should be valid until shutdown");
        let (reply_sender, reply_receiver) = oneshot::channel();

        debug!("Fetching device information.");

        async move {
            sender.send(Command::new_command_get_device(reply_sender)).await
                .map_err(|_| ())?;
            reply_receiver.await
                .map(|result| result.map_err(|_| ()))
                .map_err(|_| ())?
        }
    }

    pub fn set_axes_speed(&self, speed: [f32; 6]) -> impl Future<Output=Result<(), ()>> + 'static {

        let sender = Clone::clone(&self.command_sender)
            .expect("Sender should be valid until shutdown");
        let (reply_sender, reply_receiver) = oneshot::channel();

        debug!("Setting axes speed: {speed:?}");

        async move {
            sender.send(Command::new_command_set_axes_speed(speed, reply_sender)).await
                .map_err(|_| ())?;
            reply_receiver.await
                .map(|result| result.map_err(|_| ()))
                .map_err(|_| ())?
        }
    }

    pub fn set_axes_threshold(&self, threshold: [i32; 6]) -> impl Future<Output=Result<(), ()>> + 'static {

        let sender = Clone::clone(&self.command_sender)
            .expect("Sender should be valid until shutdown");
        let (reply_sender, reply_receiver) = oneshot::channel();

        debug!("Setting axes threshold: {threshold:?}");

        async move {
            sender.send(Command::new_command_set_axes_threshold(threshold, reply_sender)).await
                .map_err(|_| ())?;
            reply_receiver.await
                .map(|result| result.map_err(|_| ()))
                .map_err(|_| ())?
        }
    }

    pub fn set_axes_inverted(&self, inverted: [bool; 6]) -> impl Future<Output=Result<(), ()>> + 'static {

        let sender = Clone::clone(&self.command_sender)
            .expect("Sender should be valid until shutdown");
        let (reply_sender, reply_receiver) = oneshot::channel();

        debug!("Setting axes inverted: {inverted:?}");

        async move {
            sender.send(Command::new_command_set_axes_inverted(inverted, reply_sender)).await
                .map_err(|_| ())?;
            reply_receiver.await
                .map(|result| result.map_err(|_| ()))
                .map_err(|_| ())?
        }
    }

    pub fn shutdown(mut self) {
        self.do_shutdown();
    }

    fn do_shutdown(&mut self) {

        info!("Terminating client.");

        self.command_sender.take();

        if let Some(sender) = self.shutdown_sender.take() {
            let _ = sender.send(());
        }

        // FFI thread will terminate because it's recv() function will return None.
        if let Some(handle) = self.ffi_thread.take() {
            match handle.join() {
                Ok(()) => {}
                Err(cause) => {
                    eprintln!("FFI thread panicked: {:?}", cause);
                }
            }
        }

        info!("Terminated client.");
    }
}

impl Drop for SpaceNavClient {
    fn drop(&mut self) {
        self.do_shutdown();
    }
}
