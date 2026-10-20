#pragma once

#include <mc/deps/minecraft_renderer/objects/NameTagRenderObject.h>

#include <cstddef>
#include <cstdint>

namespace levi_levels {

// mod-owned storage for the grown name-tag list, because LinearAllocator<NameTagRenderObject>::allocate
// is not exported. A ring of SLOTS buffers, one per frame in flight: the game destroys the elements of a
// frame's list but never frees its storage, so a slot is reused SLOTS frames later. A slot only grows,
// and never past the largest list the drawer asks for: core caps the labels so that the game's tags plus
// the labels stay within max_labels plus the game-tag headroom
class LabelRing {
public:
    static constexpr size_t SLOTS = 8;

    // the next slot, holding room for at least count objects; its contents are raw memory
    static NameTagRenderObject* next(size_t count);

    // bytes held by every slot, for telemetry
    static uint64_t residentBytes();
};

} // namespace levi_levels
