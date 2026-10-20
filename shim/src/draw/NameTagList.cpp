#include "draw/NameTagList.h"

#include <mc/deps/minecraft_renderer/objects/NameTagRenderObject.h>
#include <mc/deps/minecraft_renderer/objects/NameTagRenderObjectCollection.h>

#include <cstring>
#include <vector>

namespace levi_levels {

namespace {

using TagVector = std::vector<NameTagRenderObject, LinearAllocator<NameTagRenderObject>>;

// the sizes of the LeviLamina 26.51.6 headers this surgery was proven against
static_assert(sizeof(TagVector) == 40);
static_assert(sizeof(NameTagRenderObject) == 144);
constexpr size_t FIRST_OFFSET = 16;
constexpr size_t LAST_OFFSET  = 24;
constexpr size_t END_OFFSET   = 32;

} // namespace

NameTagList::NameTagList(NameTagRenderObject** first, NameTagRenderObject** last, NameTagRenderObject** end)
: mFirst(first),
  mLast(last),
  mEnd(end) {}

std::optional<NameTagList> NameTagList::open(NameTagRenderObjectCollection& tags) {
    auto*       base = reinterpret_cast<char*>(&*tags.mNameTags);
    NameTagList list(
        reinterpret_cast<NameTagRenderObject**>(base + FIRST_OFFSET),
        reinterpret_cast<NameTagRenderObject**>(base + LAST_OFFSET),
        reinterpret_cast<NameTagRenderObject**>(base + END_OFFSET)
    );
    bool ordered = *list.mFirst <= *list.mLast && *list.mLast <= *list.mEnd
                && (*list.mFirst != nullptr || *list.mLast == nullptr);
    if (!ordered || list.size() != tags.mNameTags->size()) return std::nullopt;
    return list;
}

size_t NameTagList::size() const { return static_cast<size_t>(*mLast - *mFirst); }

void NameTagList::moveTo(NameTagRenderObject* storage, size_t added) {
    size_t existing = size();
    if (existing) std::memcpy(static_cast<void*>(storage), *mFirst, existing * sizeof(NameTagRenderObject));
    *mFirst = storage;
    *mLast  = storage + existing + added;
    *mEnd   = *mLast;
}

} // namespace levi_levels
