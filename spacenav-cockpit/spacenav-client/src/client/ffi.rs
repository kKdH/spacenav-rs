use crate::client::command::Command;
use libspnav::{CloseError, Event, GetDeviceError, OpenError, SetAxesInvertedError, SetAxesMappingError, SetAxesSpeedError, SetAxesThresholdError};
use std::mem::forget;
use std::os::fd::OwnedFd;
use tokio::io::unix::AsyncFd;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info};

type DeviceFd = AsyncFd<OwnedFd>;

struct State {
    device_fd: Option<DeviceFd>,
    subscribers: Vec<mpsc::Sender<Event>>,
}

impl State {

    fn new() -> Self {
        Self {
            device_fd: None,
            subscribers: Vec::new(),
        }
    }
}

pub fn spawn_ffi_thread(mut command_receiver: mpsc::Receiver<Command>, mut shutdown_receiver: oneshot::Receiver<()>) -> std::thread::JoinHandle<()> {

    debug!("Spawning FFI thread.");

    std::thread::spawn(move || {
        // A single-threaded runtime to handle commands and file-descriptor readiness concurrently.
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("Failed to create FFI runtime");

        runtime.block_on(async move {

            let mut state: State = State::new();

            loop {
                select! {
                    command = command_receiver.recv() => {
                        match command {
                            Some(command) => {
                                handle_command(&mut state, command);
                            }
                            None => {
                                // the last command sender has been dropped.
                                break;
                            }
                        }
                    }
                    result = async {
                        state.device_fd.as_ref().unwrap().readable().await
                    }, if state.device_fd.is_some() => {
                        match result {
                            Ok(mut guard) => {
                                guard.clear_ready();
                                loop {
                                    match libspnav::poll() {
                                        Ok(Some(event)) => {
                                            for subscriber in state.subscribers.iter() {
                                                let _ = subscriber.send(Clone::clone(&event)).await; // TODO: Remove dropped subscribers.
                                            }
                                        }
                                        Ok(None) => {
                                            break;
                                        }
                                        Err(cause) => {
                                            error!("{cause}");
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(cause) => {
                                error!("File-Descriptor readiness error: {cause}");
                            }
                        }
                    }
                    _ = &mut shutdown_receiver => {
                        break;
                    }
                }
            }

            terminate();
        });
    })
}

fn handle_command(state: &mut State, command: Command) {
    match command {
        Command::Open { name, reply } =>
            handle_command_open(state, name, reply),
        Command::Close { reply } =>
            handle_command_close(state, reply),
        Command::Subscribe { subscriber, reply } =>
            handle_command_subscribe(state, subscriber, reply),
        Command::GetDevice { reply } =>
            handle_command_get_device(state, reply),
        Command::SetAxesSpeed { speed, reply } =>
            handle_command_set_axes_speed(state, speed, reply),
        Command::SetAxesThreshold { threshold, reply } =>
            handle_command_set_axes_threshold(state, threshold, reply),
        Command::SetAxesInverted { inverted, reply } =>
            handle_command_set_axes_inverted(state, inverted, reply),
        Command::SetAxesMapping { mapping, reply } =>
            handle_command_set_axes_mapping(state, mapping, reply),
    }
}

fn handle_command_open(
    state: &mut State,
    name: String,
    reply: oneshot::Sender<Result<(), OpenError>>
) {

    if state.device_fd.is_some() {
        debug!("Connection to daemon is already open");
        reply.send(Ok(()))
            .expect("Failed to send open success reply");
        return;
    }

    debug!("Opening connection to daemon.");

    match libspnav::open(&name) {
        Ok(fd) => {
            let async_fd = AsyncFd::new(fd)
                .expect("Failed to create an async fd");
            state.device_fd = Some(async_fd);
            reply.send(Ok(()))
                .expect("Failed to send open success reply");
            info!("Connection to daemon opened successfully.")
        }
        Err(cause) => {
            error!("Failed to open connection to daemon: {cause}");
            reply.send(Err(cause))
                .expect("Failed to send open failure reply");
        }
    }
}

fn handle_command_close(
    state: &mut State,
    reply: oneshot::Sender<Result<(), CloseError>>
) {

    if state.device_fd.is_none() {
        debug!("Connection to daemon is already closed");
        reply.send(Ok(()))
            .expect("Failed to send close success reply");
        return;
    }

    debug!("Closing connection to daemon.");

    match libspnav::close() {
        Ok(_) => {
            let fd = state.device_fd.take();
            forget(fd); // TODO: What should be done with the fd? It should not be drop, because libspnav already closed it?
            reply.send(Ok(()))
                .expect("Failed to send close success reply");
            info!("Connection to daemon closed successfully.")
        }
        Err(cause) => {
            error!("Failed to close connection to daemon: {cause}");
            reply.send(Err(cause))
                .expect("Failed to send close failure reply");
        }
    }
}

fn handle_command_get_device(
    _state: &mut State,
    reply: oneshot::Sender<Result<libspnav::Device, GetDeviceError>>
) {

    debug!("Fetching device information from daemon.");

    match libspnav::get_device() {
        Ok(device) => {
            reply.send(Ok(device))
                .expect("Failed to send GetDevice success reply");
            info!("Fetched device information from daemon successfully.")
        }
        Err(cause) => {
            error!("Failed to fetch device information from daemon: {cause}");
            reply.send(Err(cause))
                .expect("Failed to send GetDevice failure reply");
        }
    }
}

fn handle_command_set_axes_speed(
    _state: &mut State,
    speed: [f32; 6],
    reply: oneshot::Sender<Result<(), SetAxesSpeedError>>
) {
    debug!("Setting axes speed.");

    match libspnav::set_axes_speed(speed) {
        Ok(device) => {
            reply.send(Ok(device))
                .expect("Failed to send SetAxesSpeed success reply");
            info!("Set axes speed successfully: {speed:?}")
        }
        Err(cause) => {
            error!("Failed to set axes speed: {cause}");
            reply.send(Err(cause))
                .expect("Failed to send SetAxesSpeed failure reply");
        }
    }
}

fn handle_command_set_axes_threshold(
    _state: &mut State,
    threshold: [u8; 6],
    reply: oneshot::Sender<Result<(), SetAxesThresholdError>>
) {
    debug!("Setting axes threshold.");

    match libspnav::set_axes_threshold(threshold) {
        Ok(device) => {
            reply.send(Ok(device))
                .expect("Failed to send SetAxesThreshold success reply");
            info!("Set axes threshold successfully: {threshold:?}")
        }
        Err(cause) => {
            error!("Failed to set axes threshold: {cause}");
            reply.send(Err(cause))
                .expect("Failed to send SetAxesThreshold failure reply");
        }
    }
}

fn handle_command_set_axes_inverted(
    _state: &mut State,
    inverted: [bool; 6],
    reply: oneshot::Sender<Result<(), SetAxesInvertedError>>
) {
    debug!("Setting axes inverted.");

    match libspnav::set_axes_inverted(inverted) {
        Ok(device) => {
            reply.send(Ok(device))
                .expect("Failed to send SetAxesInverted success reply");
            info!("Set axes inverted successfully: {inverted:?}")
        }
        Err(cause) => {
            error!("Failed to set axes inverted: {cause}");
            reply.send(Err(cause))
                .expect("Failed to send SetAxesInverted failure reply");
        }
    }
}

fn handle_command_set_axes_mapping(
    _state: &mut State,
    mapping: [u8; 6],
    reply: oneshot::Sender<Result<(), SetAxesMappingError>>
) {
    debug!("Setting axes mapping.");

    match libspnav::set_axes_mapping(mapping) {
        Ok(device) => {
            reply.send(Ok(device))
                .expect("Failed to send SetAxesMapping success reply");
            info!("Set axes mapping successfully: {mapping:?}")
        }
        Err(cause) => {
            error!("Failed to set axes mapping: {cause}");
            reply.send(Err(cause))
                .expect("Failed to send SetAxesMapping failure reply");
        }
    }
}

fn handle_command_subscribe(state: &mut State, subscriber: mpsc::Sender<Event>, reply: oneshot::Sender<Result<(), ()>>) {

    debug!("Subscribing to events.");

    state.subscribers.push(subscriber);

    reply.send(Ok(()))
        .expect("Failed to send SetAxesSpeed success reply");

    info!("Subscribed to events successfully.")
}

fn terminate() {
    debug!("FFI thread terminated.");
}
