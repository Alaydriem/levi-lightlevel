#include "mod/LevelsMod.h"

#include "bridge/ModLog.h"
#include "bridge/RustBridge.h"
#include "hooks/ClientUpdateHook.h"
#include "hooks/DrawHook.h"
#include "hooks/SetupCameraHook.h"
#include "hooks/ViewObjectHook.h"
#include "scan/ScanDriver.h"

#include "ll/api/mod/RegisterHelper.h"

#include <system_error>

namespace levi_levels {

namespace {

// the mod manager keeps config.hcl in this folder and requires an absolute path
std::filesystem::path absoluteConfigDir(std::filesystem::path const& dir) {
    std::error_code       error;
    std::filesystem::path absolute = std::filesystem::absolute(dir, error);
    return error ? dir : absolute;
}

} // namespace

LevelsMod& LevelsMod::getInstance() {
    static LevelsMod instance;
    return instance;
}

// load runs under the OS loader lock, so registration waits for enable
bool LevelsMod::load() {
    mConfigDir = absoluteConfigDir(getSelf().getConfigDir());
    return true;
}

bool LevelsMod::enable() {
    // a disable destroys the core, so an enable after it registers again
    if (!RustBridge::handle() && !RustBridge::create(mConfigDir)) {
        // the manager's log is not reachable without a registration, so this one line goes to the LeviLamina console
        getSelf().getLogger().error("alaydriem-mod-manager is not loaded or refused levi-levels; the overlay stays off");
        return true;
    }
    ClientUpdateHook::hook();
    SetupCameraHook::hook();
    ViewObjectHook::hook();
    DrawHook::hook();
    ModLog::info("levi-levels enabled", "{\"config_dir\":" + ModLog::quote(RustBridge::utf8(mConfigDir)) + "}");
    return true;
}

bool LevelsMod::disable() {
    if (!RustBridge::handle()) return true;
    // unhook first so no game or render call can enter Rust, then unregister and stop Rust
    DrawHook::unhook();
    ViewObjectHook::unhook();
    SetupCameraHook::unhook();
    ClientUpdateHook::unhook();
    ScanDriver::reset();
    CameraReader::reset();
    RustBridge::destroy();
    return true;
}

} // namespace levi_levels

LL_REGISTER_MOD(levi_levels::LevelsMod, levi_levels::LevelsMod::getInstance());
