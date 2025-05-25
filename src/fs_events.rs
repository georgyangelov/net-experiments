use std::thread;
use std::time::Duration;
use net_experiments::fs_event_lib::{config_flags, FSEventConfig, FSEvents};

pub fn run() {
    let config = FSEventConfig{
        paths_to_watch: vec!["/Users/stormbreaker/dev/net-experiments".into()],
        latency: Duration::from_millis(1000),
        since_event_id: None,
        flags: config_flags::FILE_EVENTS | config_flags::IGNORE_SELF
    };

    let fs_events = FSEvents::new(config, |e| {
        println!("Event: {:?}", e.items);
    }).expect("could not configure FSEvents listener");

    let listener = fs_events.start_listening().expect("could not start listening");
    println!("Started listening");

    thread::sleep(Duration::from_secs(5));

    listener.stop_listening();
    println!("Stopped listening");
}
