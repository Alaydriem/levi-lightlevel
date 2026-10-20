#include "mod/LevelsMod.h"

#include "bridge/RustBridge.h"
#include "hooks/ClientUpdateHook.h"
#include "hooks/DrawHook.h"
#include "hooks/SetupCameraHook.h"
#include "hooks/ViewObjectHook.h"
#include "input/KeyListener.h"
#include "scan/ScanDriver.h"

#include "ll/api/mod/RegisterHelper.h"

namespace levi_levels {

LevelsMod& LevelsMod::getInstance() {
    static LevelsMod instance;
    return instance;
}

bool LevelsMod::load() { return startCore(); }

bool LevelsMod::enable() {
    // a disable destroys the core, so an enable after it starts a new one
    if (!RustBridge::handle() && !startCore()) return false;
    ClientUpdateHook::hook();
    SetupCameraHook::hook();
    ViewObjectHook::hook();
    DrawHook::hook();
    KeyListener::registerListener();
    getSelf().getLogger().info("levi-levels loaded");
    return true;
}

bool LevelsMod::disable() {
    // unhook first so no game or render call can enter Rust, then stop Rust
    KeyListener::unregisterListener();
    DrawHook::unhook();
    ViewObjectHook::unhook();
    SetupCameraHook::unhook();
    ClientUpdateHook::unhook();
    ScanDriver::reset();
    CameraReader::reset();
    RustBridge::destroy();
    return true;
}

bool LevelsMod::startCore() {
    auto& self       = getSelf();
    auto  configFile = self.getConfigDir() / "config.json";
    if (RustBridge::create(configFile)) return true;
    self.getLogger().error("levi-levels could not start its Rust core");
    return false;
}

} // namespace levi_levels

LL_REGISTER_MOD(levi_levels::LevelsMod, levi_levels::LevelsMod::getInstance());
