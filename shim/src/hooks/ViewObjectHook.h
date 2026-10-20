#pragma once

#include "draw/CameraReader.h"

#include <ll/api/memory/Hook.h>
#include <mc/client/gui/screens/ScreenContext.h>
#include <mc/client/renderer/game/LevelRendererPlayer.h>
#include <mc/deps/minecraft_renderer/objects/ViewRenderData.h>
#include <mc/deps/minecraft_renderer/objects/ViewRenderObject.h>

namespace levi_levels {

// the view the game builds for each frame: keeps its camera position for the label cull. High priority
// runs this hook outside the Normal ones, so the position is read after other mods (Freecam) have set it
LL_TYPE_INSTANCE_HOOK(
    ViewObjectHook,
    ll::memory::HookPriority::High,
    LevelRendererPlayer,
    &LevelRendererPlayer::createViewRenderObject,
    ViewRenderObject,
    ScreenContext& screenContext,
    SubClientId    clientSubId
) {
    ViewRenderObject result = origin(screenContext, clientSubId);
    CameraReader::observePosition(result.mViewData->mCameraPos);
    return result;
}

} // namespace levi_levels
