#ifndef __FS_EVENTS_H__
#define __FS_EVENTS_H__

#include <memory>

#include <CoreServices/CoreServices.h>

struct FSEventContext;
struct FSEvent;
struct FSEventItem;
struct RawFSEventConfig;

enum class FSEventStartResult : ::std::uint8_t;

#include "rust/cxx.h"

typedef rust::Fn<void(FSEventContext&, FSEvent)> FSCallback;

class MacOSFSEventsMonitor {
public:
  MacOSFSEventsMonitor(
    const RawFSEventConfig& config,
    rust::Box<FSEventContext>& _context,
    FSCallback _callback
  );

  ~MacOSFSEventsMonitor();

  bool valid() const;
  bool start();
  void stop();

  rust::Box<FSEventContext> context;
  FSCallback callback;
  FSEventStreamRef stream;
  dispatch_queue_t queue;
};

std::unique_ptr<MacOSFSEventsMonitor> new_fs_events_monitor(
  const RawFSEventConfig& config,
  rust::Box<FSEventContext> context,
  FSCallback callback
);

#endif
