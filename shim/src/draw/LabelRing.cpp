#include "draw/LabelRing.h"

#include <array>
#include <vector>

namespace levi_levels {

namespace {

static_assert(alignof(NameTagRenderObject) <= __STDCPP_DEFAULT_NEW_ALIGNMENT__);

using Slots = std::array<std::vector<std::byte>, LabelRing::SLOTS>;

// allocated once and never freed: the game may still hold a list in a slot when the mod unloads
Slots&  g_slots = *new Slots();
size_t g_index = 0;

} // namespace

NameTagRenderObject* LabelRing::next(size_t count) {
    auto& slot   = g_slots[g_index];
    g_index      = (g_index + 1) % SLOTS;
    size_t bytes = count * sizeof(NameTagRenderObject);
    if (slot.size() < bytes) slot.resize(bytes);
    return reinterpret_cast<NameTagRenderObject*>(slot.data());
}

uint64_t LabelRing::residentBytes() {
    uint64_t bytes = 0;
    for (auto const& slot : g_slots) bytes += slot.capacity();
    return bytes;
}

} // namespace levi_levels
