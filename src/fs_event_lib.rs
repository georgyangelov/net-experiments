use cxx::UniquePtr;

pub use ffi::{FSEvent};
pub use ffi::{FSEventItem};

pub struct FSEvents {
    monitor: UniquePtr<ffi::MacOSFSEventsMonitor>
}

impl FSEvents {
    pub fn new(callback: Box<dyn Fn(&FSEvent)>) -> Self {
        let context = FSEventContext { callback };
        let monitor = ffi::new_fs_events_monitor(Box::new(context), fs_event_handler);

        FSEvents { monitor }
    }

    pub fn listen(&mut self) {
        let pinned = self.monitor.pin_mut();

        pinned.start();
    }
}

impl Drop for FSEvents {
    fn drop(&mut self) {
        todo!("cleanup")
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
    enum FSEventStartResult {
        Done,
        Failed
    }

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

    extern "Rust" {
        type FSEventContext;
    }

    unsafe extern "C++" {
        include!("net-experiments/cpp/include/fs_events.h");

        type MacOSFSEventsMonitor;

        fn new_fs_events_monitor(
            ctx: Box<FSEventContext>,
            callback: fn(&mut FSEventContext, FSEvent),
        ) -> UniquePtr<MacOSFSEventsMonitor>;

        fn start(self: Pin<&mut MacOSFSEventsMonitor>) -> FSEventStartResult;
    }
}