#include "scan/ScanDriver.h"

#include "bridge/RustBridge.h"
#include "scan/FrameStatusReader.h"
#include "scan/SurfaceSampler.h"

#include <mc/client/game/ClientInstance.h>
#include <mc/client/player/LocalPlayer.h>
#include <mc/world/level/BlockSource.h>

#include <atomic>

namespace levi_levels {

namespace {

constexpr double MICROS_PER_SECOND = 1000000.0;

std::atomic<ClientInstance*> g_client{nullptr};
std::atomic<float>           g_sampleMicros{0.0f};

void sampleColumns(const LvHandle* handle, ClientInstance& client, LvScanView const& view) {
    LocalPlayer* player = client.getLocalPlayer();
    if (!player || view.count == 0) return;
    BlockSource const& region = player->getDimensionBlockSourceConst();
    LvSurfaceSample    samples[LV_MAX_COLUMN_SURFACES];
    for (uint32_t i = 0; i < view.count; ++i) {
        LvColumnRequest const& request = view.columns[i];
        uint32_t               count   = SurfaceSampler::sample(region, request, samples);
        lv_submit_column(handle, request.x, request.z, samples, count);
    }
}

} // namespace

void ScanDriver::tick(ClientInstance& client) {
    g_client.store(&client, std::memory_order_relaxed);
    const LvHandle* handle = RustBridge::handle();
    if (!handle) return;
    LvFrameStatus status = FrameStatusReader::read(client);
    LvScanView    view{};
    if (lv_frame(handle, &status, &view) != LV_STATUS_OK) return;
    double start = FrameStatusReader::nowSeconds();
    sampleColumns(handle, client, view);
    float micros = static_cast<float>((FrameStatusReader::nowSeconds() - start) * MICROS_PER_SECOND);
    g_sampleMicros.store(micros, std::memory_order_relaxed);
}

ClientInstance* ScanDriver::client() { return g_client.load(std::memory_order_relaxed); }

float ScanDriver::lastSampleMicros() { return g_sampleMicros.load(std::memory_order_relaxed); }

void ScanDriver::reset() {
    g_client.store(nullptr, std::memory_order_relaxed);
    g_sampleMicros.store(0.0f, std::memory_order_relaxed);
}

} // namespace levi_levels
