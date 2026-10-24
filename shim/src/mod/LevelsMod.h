#pragma once

#include "ll/api/mod/NativeMod.h"

#include <filesystem>

namespace levi_levels {

// the LeviLamina entry point: registers with the mod manager through Rust and installs the hooks
class LevelsMod {
public:
    static LevelsMod& getInstance();

    LevelsMod() : mSelf(*ll::mod::NativeMod::current()) {}

    [[nodiscard]] ll::mod::NativeMod& getSelf() const { return mSelf; }

    bool load();

    bool enable();

    bool disable();

private:
    ll::mod::NativeMod&   mSelf;
    std::filesystem::path mConfigDir;
};

} // namespace levi_levels
