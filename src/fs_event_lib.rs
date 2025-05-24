use std::fmt::{Debug, Formatter};
use std::time;
use cxx::UniquePtr;

pub use ffi::{FSEvent};
pub use ffi::{FSEventItem};
use crate::fs_event_lib::ffi::{RawFSEventConfig};

pub struct FSEvents {
    monitor: UniquePtr<ffi::MacOSFSEventsMonitor>
}

/// Read [/Library/Developer/CommandLineTools/SDKs/MacOSX15.4.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/FSEvents.framework/Versions/A/Headers/FSEvents.h]
/// for documentation on all values and flags
pub struct FSEventConfig {
    pub paths_to_watch: Vec<String>,
    pub latency: time::Duration,
    pub since_event_id: Option<u64>,
    pub flags: u32
}

const EVENT_ID_SINCE_NOW: u64 = 0xFFFFFFFFFFFFFFFFu64;

pub mod config_flags {
    pub const NONE: u32 = 0;
    pub const NO_DEFER: u32 = 0x00000002;
    pub const WATCH_ROOT: u32 = 0x00000004;
    pub const IGNORE_SELF: u32 = 0x00000008;
    pub const FILE_EVENTS: u32 = 0x00000010;
    pub const MARK_SELF: u32 = 0x00000020;
    pub const FULL_HISTORY: u32 = 0x00000080;
}

pub mod callback_flags {
    pub const NONE: u32 = 0x00000000;
    pub const MUST_SCAN_SUBDIRS: u32 = 0x00000001;
    pub const USER_DROPPED: u32 = 0x00000002;
    pub const KERNEL_DROPPED: u32 = 0x00000004;
    pub const EVENT_IDS_WRAPPED: u32 = 0x00000008;
    pub const HISTORY_DONE: u32 = 0x00000010;
    pub const ROOT_CHANGED: u32 = 0x00000020;
    pub const MOUNT: u32 = 0x00000040;
    pub const UNMOUNT: u32 = 0x00000080;
    pub const ITEM_CREATED: u32 = 0x00000100;
    pub const ITEM_REMOVED: u32 = 0x00000200;
    pub const ITEM_INODE_META_MOD: u32 = 0x00000400;
    pub const ITEM_RENAMED: u32 = 0x00000800;
    pub const ITEM_MODIFIED: u32 = 0x00001000;
    pub const ITEM_FINDER_INFO_MOD: u32 = 0x00002000;
    pub const ITEM_CHANGE_OWNER: u32 = 0x00004000;
    pub const ITEM_XATTR_MOD: u32 = 0x00008000;
    pub const ITEM_IS_FILE: u32 = 0x00010000;
    pub const ITEM_IS_DIR: u32 = 0x00020000;
    pub const ITEM_IS_SYMLINK: u32 = 0x00040000;
    pub const OWN_EVENT: u32 = 0x00080000;
    pub const ITEM_IS_HARDLINK: u32  = 0x00100000;
    pub const ITEM_IS_LAST_HARDLINK: u32  = 0x00200000;
    pub const ITEM_CLONED: u32 = 0x00400000;
}

pub struct CouldNotStartListeningErr {
    pub fs_events: FSEvents
}

impl Debug for CouldNotStartListeningErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("could not start listener")
    }
}

impl FSEvents {
    pub fn new(config: FSEventConfig, callback: Box<dyn Fn(&FSEvent)>) -> Result<Self, ()> {
        let context = FSEventContext { callback };
        let raw_config = RawFSEventConfig{
            paths_to_watch: config.paths_to_watch,
            latency_sec: config.latency.as_secs_f64(),
            since_when: config.since_event_id.unwrap_or(EVENT_ID_SINCE_NOW),
            flags: config.flags
        };

        let monitor = ffi::new_fs_events_monitor(
            &raw_config,
            Box::new(context),
            fs_event_handler
        );

        if !monitor.valid() {
            return Err(())
        }

        Ok(FSEvents { monitor })
    }

    pub fn start_listening(mut self) -> Result<FSEventsListening, CouldNotStartListeningErr> {
        let success = self.monitor.pin_mut().start();
        if !success {
            return Err(CouldNotStartListeningErr { fs_events: self });
        }

        Ok(FSEventsListening { fs_events: Some(self) })
    }
}

pub struct FSEventsListening {
    fs_events: Option<FSEvents>
}

impl FSEventsListening {
    pub fn stop_listening(mut self) -> FSEvents {
        let mut events = self.fs_events.take().unwrap();

        events.monitor.pin_mut().stop();
        
        events
    }
}

impl Drop for FSEventsListening {
    fn drop(&mut self) {
        match self.fs_events.take() {
            None => {}
            Some(mut fs_events) => { fs_events.monitor.pin_mut().stop() }
        };
    }
}

fn fs_event_handler(context: &mut FSEventContext, event: FSEvent) {
    (context.callback)(&event);
}

struct FSEventContext {
    // We need a box here so that the address of callback doesn't change
    callback: Box<dyn Fn(&FSEvent)>
}

#[cxx::bridge]
mod ffi {
    #[derive(Debug)]
    pub struct FSEvent {
        pub items: Vec<FSEventItem>,
    }

    #[derive(Debug)]
    pub struct FSEventItem {
        pub path: String,
        pub flags: u32,
        pub event_id: u64
    }

    #[derive(Debug)]
    pub struct RawFSEventConfig {
        pub latency_sec: f64,
        pub paths_to_watch: Vec<String>,
        pub since_when: u64,
        pub flags: u32,
    }

    extern "Rust" {
        type FSEventContext;
    }

    unsafe extern "C++" {
        include!("net-experiments/cpp/include/fs_events.h");

        type MacOSFSEventsMonitor;

        fn new_fs_events_monitor(
            config: &RawFSEventConfig,
            ctx: Box<FSEventContext>,
            callback: fn(&mut FSEventContext, FSEvent),
        ) -> UniquePtr<MacOSFSEventsMonitor>;

        fn valid(self: &MacOSFSEventsMonitor) -> bool;
        fn start(self: Pin<&mut MacOSFSEventsMonitor>) -> bool;
        fn stop(self: Pin<&mut MacOSFSEventsMonitor>);
    }
}