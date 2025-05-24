#include "net-experiments/cpp/include/fs_events.h"
#include "net-experiments/src/fs_event_lib.rs.h"

#include <CoreServices/CoreServices.h>

#include <iostream>
#include <memory>

std::unique_ptr<MacOSFSEventsMonitor> new_fs_events_monitor(
    rust::Box<FSEventContext> context,
    const FSCallback callback
) {
    return std::make_unique<MacOSFSEventsMonitor>(
        context,
        callback
    );
}

void fsCallback(
    ConstFSEventStreamRef _stream_ref,
    void * client_call_back_info,
    size_t num_events,
    void * event_paths,
    const FSEventStreamEventFlags * event_flags,
    const FSEventStreamEventId * event_ids
) {
    auto monitor = static_cast<MacOSFSEventsMonitor*>(client_call_back_info);

    auto paths = static_cast<char **>(event_paths);

    rust::Vec<FSEventItem> items;

    std::cout << "Callback, num events: " << num_events << std::endl;

    for (size_t i = 0; i < num_events; ++i) {
        items.push_back(FSEventItem{
            .path = paths[i],
            .flags = event_flags[i],
            .event_id = event_ids[i]
        });

        std::cout << "Changed path: " << paths[i] << std::endl;
        std::cout << "Flags: 0x" << std::hex << event_flags[i] << std::endl;
    }

    monitor->callback(monitor->context.operator*(), FSEvent{
        .items = items
    });

//    monitor->callback(monitor->context.operator*(), "Event");
    
    // on_fs_change("Callback executed");
}

FSEventStartResult MacOSFSEventsMonitor::start() {
    std::cout << "Registering for events..." << std::endl;

    // Create a Run loop: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/RunLoopManagement/RunLoopManagement.html#//apple_ref/doc/uid/10000057i-CH16-SW1
    // CFRunLoopGetCurrent();

    /* Define variables and create a CFArray object containing
       CFString objects containing paths to watch.
     */
    CFStringRef mypath = CFSTR("/Users/stormbreaker/dev/net-experiments");
    CFArrayRef pathsToWatch = CFArrayCreate(nullptr, (const void **)&mypath, 1, nullptr);

    // TODO: Clean up for this using the callback
    FSEventStreamContext *callbackInfo = new FSEventStreamContext{
        .info = this
    }; // could put stream-specific data here.
    FSEventStreamRef stream = nullptr;
    CFAbsoluteTime latency = 3.0; /* Latency in seconds */

    /* Create the stream, passing in a callback */
    stream = FSEventStreamCreate(
        nullptr,
        &fsCallback,
        callbackInfo,
        pathsToWatch,
        kFSEventStreamEventIdSinceNow, /* Or a previous event ID */
        latency,
        kFSEventStreamCreateFlagIgnoreSelf | kFSEventStreamCreateFlagFileEvents
    );

    std::cout << "Created event stream" << std::endl;

    /* Create the stream before calling this. */
    FSEventStreamScheduleWithRunLoop(stream, CFRunLoopGetCurrent(), kCFRunLoopDefaultMode);

    auto success = FSEventStreamStart(stream);
    if (!success) {
        return FSEventStartResult::Failed;
        // return "FSEventStreamStart failed";
    }

    std::cout << "Starting the thread run loop" << std::endl;
    CFRunLoopRun();

    std::cout << "Run loop ended" << std::endl;

    // return "Hello world";
    return FSEventStartResult::Done;
}