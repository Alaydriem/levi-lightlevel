#pragma once

#include <cstddef>
#include <optional>

struct NameTagRenderObject;
struct NameTagRenderObjectCollection;

namespace levi_levels {

// the game's name-tag vector seen through its raw MSVC layout: 16 bytes of LinearAllocator, then the
// first / last / end pointers. It is the only way to grow the list, because the allocator's allocate is
// not exported
class NameTagList {
public:
    // empty when the pointers are out of order or disagree with size(), so the layout is not the one this
    // build expects
    static std::optional<NameTagList> open(NameTagRenderObjectCollection& tags);

    size_t size() const;

    // relocates the game's own tags bit for bit to the front of storage and points the list at storage,
    // which then holds `size() + added` objects; the old storage is never destroyed or freed by the game
    void moveTo(NameTagRenderObject* storage, size_t added);

private:
    NameTagList(NameTagRenderObject** first, NameTagRenderObject** last, NameTagRenderObject** end);

    NameTagRenderObject** mFirst;
    NameTagRenderObject** mLast;
    NameTagRenderObject** mEnd;
};

} // namespace levi_levels
