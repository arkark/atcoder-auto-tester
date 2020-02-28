use std::any::type_name;
use std::error;
use std::process;

use crate::message::Messageable;

pub trait ErrorHandleable<T> {
    fn handle_error(self, msg: &str) -> T;
}

impl<T, E: error::Error> ErrorHandleable<T> for Result<T, E> {
    fn handle_error(self, msg: &str) -> T {
        self.unwrap_or_else(|e| ErrorHandler::handle(e, msg))
    }
}

struct ErrorHandler;

impl ErrorHandler {
    pub fn handle<E: error::Error>(err: E, msg: &str) -> ! {
        format!("{}: {}", type_name::<E>(), err)
            .trim()
            .warning_message()
            .show();
        msg.error_message().show();
        process::exit(1)
    }
}
