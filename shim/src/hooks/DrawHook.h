#pragma once

#include "draw/LabelDrawer.h"

#include <ll/api/memory/Hook.h>
#include <mc/client/gui/screens/ScreenContext.h>
#include <mc/client/renderer/game/LevelRendererPlayer.h>
#include <mc/deps/minecraft_renderer/objects/NameTagRenderObjectCollection.h>

namespace levi_levels {

// the game's in-world name-tag list: Rust's labels are appended after the game's own tags
LL_TYPE_INSTANCE_HOOK(
    DrawHook,
    ll::memory::HookPriority::Normal,
    LevelRendererPlayer,
    &LevelRendererPlayer::$extractNameTags,
    NameTagRenderObjectCollection,
    ScreenContext& screenContext
) {
    NameTagRenderObjectCollection tags = origin(screenContext);
    LabelDrawer::append(screenContext, tags);
    return tags;
}

} // namespace levi_levels
