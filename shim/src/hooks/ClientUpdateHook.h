#pragma once

#include "scan/ScanDriver.h"

#include <ll/api/memory/Hook.h>
#include <mc/client/game/ClientInstance.h>

namespace levi_levels {

// once per client tick, after the game's own update: runs the frame and samples the columns Rust asks for
LL_TYPE_INSTANCE_HOOK(
    ClientUpdateHook,
    ll::memory::HookPriority::Normal,
    ClientInstance,
    &ClientInstance::$update,
    bool,
    bool a1
) {
    bool result = origin(a1);
    ScanDriver::tick(*this);
    return result;
}

} // namespace levi_levels
