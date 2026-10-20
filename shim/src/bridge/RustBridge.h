#pragma once

#include "levi_levels.h"

#include <filesystem>
#include <string>

namespace levi_levels {

// owns the Rust handle and the log callback; every call into Rust goes through handle()
class RustBridge {
public:
    static bool create(const std::filesystem::path& configFile);

    static void destroy();

    // null before create and after destroy
    static const LvHandle* handle();

private:
    static std::string utf8(const std::filesystem::path& path);
};

} // namespace levi_levels
