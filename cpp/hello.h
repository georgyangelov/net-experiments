#ifndef __HELLO_H__
#define __HELLO_H__

#include <memory>

struct FSEventContext;
enum class FSEventStartResult : ::std::uint8_t;

#include "rust/cxx.h"

class MacOSFSEventsMonitor {
public:
  explicit MacOSFSEventsMonitor(
    rust::Box<FSEventContext>& _context,
    const rust::Fn<void(FSEventContext&, rust::String)> _callback
  ) : context(std::move(_context)), callback(_callback) {}

  FSEventStartResult start();

  rust::Box<FSEventContext> context;
  rust::Fn<void(FSEventContext&, rust::String)> callback;
};

std::unique_ptr<MacOSFSEventsMonitor> new_fs_events_monitor(
  rust::Box<FSEventContext> context,
  rust::Fn<void(FSEventContext&, rust::String)> callback
);

#endif

