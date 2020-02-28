use std::env;
use std::thread;
use std::time;

use inotify::{EventMask, Events, Inotify, WatchMask};
use itertools::Itertools;

use crate::error::ErrorHandleable;

pub struct Detector {
    inotify: Inotify,
    buffer: [u8; 4096],
}

impl Default for Detector {
    fn default() -> Self {
        let mut inotify = Inotify::init().handle_error("Failed to initialize inotify");
        let current_dir = env::current_dir().handle_error("Failed to determine current directory");
        inotify
            .add_watch(current_dir, WatchMask::MODIFY | WatchMask::CREATE)
            .handle_error("Failed to add inotify watch");
        Self {
            inotify,
            buffer: [0u8; 4096],
        }
    }
}

impl Detector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn wait_file_names(self: &mut Self) -> Vec<String> {
        let mut file_names: Vec<String> = Self::wait_events(&mut self.inotify, &mut self.buffer)
            .filter_map(|e| Self::extract_file_name(&e))
            .collect();
        assert!(!file_names.is_empty());
        thread::sleep(time::Duration::from_millis(200));
        file_names.extend(
            Self::get_events(&mut self.inotify, &mut self.buffer)
                .filter_map(|e| Self::extract_file_name(&e)),
        );
        file_names.into_iter().unique().collect()
    }

    fn file_changed(event: &inotify::Event<&std::ffi::OsStr>) -> bool {
        (event.mask.contains(EventMask::CREATE) || event.mask.contains(EventMask::MODIFY))
            && !event.mask.contains(EventMask::ISDIR)
    }

    fn extract_file_name(event: &inotify::Event<&std::ffi::OsStr>) -> Option<String> {
        if Self::file_changed(&event) {
            event.name.map(|name| String::from(name.to_string_lossy()))
        } else {
            None
        }
    }

    fn wait_events<'a>(inotify: &'a mut Inotify, buffer: &'a mut [u8]) -> Events<'a> {
        inotify
            .read_events_blocking(buffer)
            .handle_error("Failed to read inotify events")
    }

    fn get_events<'a>(inotify: &'a mut Inotify, buffer: &'a mut [u8]) -> Events<'a> {
        inotify
            .read_events(buffer)
            .handle_error("Failed to read inotify events")
    }
}

impl IntoIterator for Detector {
    type Item = String;
    type IntoIter = FileGen;

    fn into_iter(self) -> Self::IntoIter {
        FileGen::new(self)
    }
}

pub struct FileGen {
    detector: Detector,
    file_names: Vec<String>,
}

impl FileGen {
    pub fn new(detector: Detector) -> Self {
        Self {
            detector,
            file_names: vec![],
        }
    }
}

impl Iterator for FileGen {
    type Item = String;

    fn next(self: &mut Self) -> Option<Self::Item> {
        if self.file_names.is_empty() {
            self.file_names = self.detector.wait_file_names();
        }
        self.file_names.pop()
    }
}
