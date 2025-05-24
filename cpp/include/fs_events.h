#ifndef __FS_EVENTS_H__
#define __FS_EVENTS_H__

#include <memory>

struct FSEventContext;
struct FSEvent;
struct FSEventItem;
enum class FSEventStartResult : ::std::uint8_t;

#include "rust/cxx.h"

typedef rust::Fn<void(FSEventContext&, FSEvent)> FSCallback;

class MacOSFSEventsMonitor {
public:
  explicit MacOSFSEventsMonitor(
    rust::Box<FSEventContext>& _context,
    const FSCallback _callback
  ) : context(std::move(_context)), callback(_callback) {}

  FSEventStartResult start();

  rust::Box<FSEventContext> context;
  FSCallback callback;
};

std::unique_ptr<MacOSFSEventsMonitor> new_fs_events_monitor(
  rust::Box<FSEventContext> context,
  FSCallback callback
);

#endif
