#include "bridge/RustBridge.h"

#include <atomic>

namespace levi_levels {

namespace {

std::atomic<LvHandle*> g_handle{nullptr};

} // namespace

bool RustBridge::create(const std::filesystem::path& configDir) {
    std::string dir = utf8(configDir);
    LvStr       text{reinterpret_cast<const uint8_t*>(dir.data()), dir.size()};
    LvHandle*   handle = lv_create(&text);
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
