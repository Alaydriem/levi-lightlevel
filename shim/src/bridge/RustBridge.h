#pragma once

#include "levi_levels.h"

#include <filesystem>
#include <string>

namespace levi_levels {

// owns the Rust handle; every call into Rust goes through handle()
class RustBridge {
public:
    // registers with the mod manager; false when the manager is not loaded or refuses the mod
    static bool create(const std::filesystem::path& configDir);

    static void destroy();

    // null before create and after destroy
    static const LvHandle* handle();

    static std::string utf8(const std::filesystem::path& path);
};

} // namespace levi_levels
