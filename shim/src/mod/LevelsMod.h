#pragma once

#include "ll/api/mod/NativeMod.h"

namespace levi_levels {

// the LeviLamina entry point: starts Rust, installs the hooks and the key listener
class LevelsMod {
public:
    static LevelsMod& getInstance();

    LevelsMod() : mSelf(*ll::mod::NativeMod::current()) {}

    [[nodiscard]] ll::mod::NativeMod& getSelf() const { return mSelf; }

    bool load();

    bool enable();

    bool disable();

private:
    bool startCore();

    ll::mod::NativeMod& mSelf;
};

} // namespace levi_levels
