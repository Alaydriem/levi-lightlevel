#include "bridge/RustBridge.h"

#include "mod/LevelsMod.h"

#include <atomic>
#include <string_view>

namespace levi_levels {

namespace {

std::atomic<LvHandle*> g_handle{nullptr};

void logFromRust(uint8_t level, const uint8_t* message, uintptr_t len) {
    std::string_view text(reinterpret_cast<const char*>(message), len);
    auto&            logger = LevelsMod::getInstance().getSelf().getLogger();
    switch (level) {
    case 1:
        logger.error("{}", text);
        break;
    case 2:
        logger.warn("{}", text);
        break;
    case 3:
        logger.info("{}", text);
        break;
    default:
        logger.debug("{}", text);
        break;
    }
}

} // namespace

bool RustBridge::create(const std::filesystem::path& configFile) {
    std::string config = utf8(configFile);
    LvStr       path{reinterpret_cast<const uint8_t*>(config.data()), config.size()};
    LvHandle*   handle = lv_create(&path, &logFromRust);
    g_handle.store(handle, std::memory_order_release);
    return handle != nullptr;
}

void RustBridge::destroy() {
    LvHandle* handle = g_handle.exchange(nullptr, std::memory_order_acq_rel);
    if (handle) lv_destroy(handle);
}

const LvHandle* RustBridge::handle() { return g_handle.load(std::memory_order_acquire); }

std::string RustBridge::utf8(const std::filesystem::path& path) {
    auto bytes = path.u8string();
    return std::string(reinterpret_cast<const char*>(bytes.data()), bytes.size());
}

} // namespace levi_levels
