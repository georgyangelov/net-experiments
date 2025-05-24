#include "net-experiments/cpp/include/fs_events.h"
#include "net-experiments/src/fs_event_lib.rs.h"

#include <memory>

std::unique_ptr<MacOSFSEventsMonitor> new_fs_events_monitor(
    const RawFSEventConfig& config,
    rust::Box<FSEventContext> context,
    const FSCallback callback
) {
    return std::make_unique<MacOSFSEventsMonitor>(
        config,
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
    (void)_stream_ref;

    const auto monitor = static_cast<MacOSFSEventsMonitor*>(client_call_back_info);
    const auto paths = static_cast<char **>(event_paths);

    rust::Vec<FSEventItem> items;
    items.reserve(num_events);

    for (size_t i = 0; i < num_events; ++i) {
        items.push_back(FSEventItem{
            .path = paths[i],
            .flags = event_flags[i],
            .event_id = event_ids[i]
        });
    }

    monitor->callback(monitor->context.operator*(), FSEvent{
        .items = items
    });
}

MacOSFSEventsMonitor::MacOSFSEventsMonitor(
    const RawFSEventConfig& config,
    rust::Box<FSEventContext>& _context,
    const FSCallback _callback
  ) : context(std::move(_context)),
      callback(_callback) {

    const auto pathsToWatch = CFArrayCreateMutable(
        nullptr,
        config.paths_to_watch.size(),
        &kCFTypeArrayCallBacks
    );

    for (auto path: config.paths_to_watch) {
        const auto str = CFStringCreateWithCString(nullptr, path.c_str(), kCFStringEncodingUTF8);

        CFArrayAppendValue(pathsToWatch, str);

        // Retained by the array callback
        CFRelease(str);
    }

    auto *callbackInfo = new FSEventStreamContext{
        .info = this,
    };

    /* Create the stream, passing in a callback */
    this->stream = FSEventStreamCreate(
        nullptr,
        &fsCallback,
        callbackInfo,
        pathsToWatch,
        config.since_when,
        config.latency_sec,
        config.flags
    );

    if (this->stream) {
        // https://developer.apple.com/library/archive/documentation/General/Conceptual/ConcurrencyProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40008091-CH1-SW1
        this->queue = dispatch_queue_create("net.gangelov.fsevents", nullptr);

        FSEventStreamSetDispatchQueue(this->stream, this->queue);
    } else {
        this->queue = nullptr;
    }
}

MacOSFSEventsMonitor::~MacOSFSEventsMonitor() {
    // Removes FSEventStreamSetDispatchQueue
    if (this->stream) {
        FSEventStreamInvalidate(this->stream);
    }

    // Decrements the stream refcount, should become 0 now
    if (this->stream) {
        FSEventStreamRelease(this->stream);
    }

    // Release the queue
    if (this->queue) {
        dispatch_release(this->queue);
    }
}

bool MacOSFSEventsMonitor::valid() const {
    return this->stream != nullptr && this->queue != nullptr;
}

bool MacOSFSEventsMonitor::start() {
    if (!this->stream) {
        return false;
    }

    return FSEventStreamStart(stream);
}

void MacOSFSEventsMonitor::stop() {
    FSEventStreamStop(stream);
}