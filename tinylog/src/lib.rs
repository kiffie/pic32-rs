//! a very small replacement for the log crate to be used on microcontrollers

#![no_std]

use core;
use core::fmt;
use core::fmt::Write;

static mut LOGGER: Option<Log> = None;

/// An enum representing the available verbosity levels of the logger.
///
/// Typical usage includes: checking if a certain `Level` is enabled with
/// [`log_enabled!`](macro.log_enabled.html), specifying the `Level` of
/// [`log!`](macro.log.html), and comparing a `Level` directly to a
/// [`LevelFilter`](enum.LevelFilter.html).
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = 1, // This way these line up with the discriminants for LevelFilter below
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

pub struct Log {

    bwrite_all: Option<fn(&[u8])>, 
}

impl Log {

    pub fn new(bwrite_all: fn(&[u8])) -> Log {
        Log{ bwrite_all: Some(bwrite_all) }
    }

    pub fn set_bwrite_all(&mut self, bwrite_all: fn(&[u8])){
        self.bwrite_all = Some(bwrite_all);
    }


    fn log(&mut self, args: core::fmt::Arguments){
        writeln!(self, "{}", args).unwrap();
    }
}

impl Write for Log {

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if let Some(bw) = self.bwrite_all {
            bw(s.as_bytes());
        }
        Ok(())
    }
}


pub fn set_bwrite_all(bwrite_all: fn(&[u8])){

    unsafe {
        LOGGER = Some(Log::new(bwrite_all));
    }
}

pub fn __private_api_log(
    args: fmt::Arguments,
    _level: Level,
    &(module_path, line): &(&str, u32))
{
    unsafe {
        if let Some(ref mut l) = LOGGER {
            l.log(format_args!("[{}:{}] {}", module_path, line, args));
        }
    }
}


/// The standard logging macro.
///
/// This macro will generically log with the specified `Level` and `format!`
/// based argument list.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// use log::Level;
///
/// # fn main() {
/// let data = (42, "Forty-two");
/// let private_data = "private";
///
/// log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
/// log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
///     data.0, data.1, private_data);
/// # }
/// ```
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= TL_LOGLEVEL {
            $crate::__private_api_log(
                format_args!($($arg)+),
                lvl,
                &(file!(), line!()),
            );
        }
    });
    ($lvl:expr, $($arg:tt)+) => ($crate::log!(target: module_path!(), $lvl, $($arg)+))
}

/// Logs a message at the error level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// let (err_info, port) = ("No connection", 22);
///
/// error!("Error: {} on port {}", err_info, port);
/// error!(target: "app_events", "App Error: {}, Port: {}", err_info, 22);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Error, $($arg)+);
    )
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// let warn_description = "Invalid Input";
///
/// warn!("Warning! {}!", warn_description);
/// warn!(target: "input_events", "App received warning: {}", warn_description);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Warn, $($arg)+);
    )
}

/// Logs a message at the info level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// # struct Connection { port: u32, speed: f32 }
/// let conn_info = Connection { port: 40, speed: 3.20 };
///
/// info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
/// info!(target: "connection_events", "Successfull connection, port: {}, speed: {}",
///       conn_info.port, conn_info.speed);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Info, $($arg)+);
    )
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
/// debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Debug, $($arg)+);
    )
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate log;
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
/// trace!(target: "app_events", "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Trace, $($arg)+);
    )
}
