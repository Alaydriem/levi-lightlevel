#pragma once

#include "draw/CameraReader.h"

#include <ll/api/memory/Hook.h>
#include <mc/client/renderer/game/LevelRendererPlayer.h>
#include <mc/deps/renderer/Camera.h>

namespace levi_levels {

// the render camera each frame: keeps the direction it faces for the label cull. High priority runs this
// hook outside the Normal ones, so the direction is read after other mods (Freecam) have set it
LL_TYPE_INSTANCE_HOOK(
    SetupCameraHook,
    ll::memory::HookPriority::High,
    LevelRendererPlayer,
    &LevelRendererPlayer::setupCamera,
    void,
    mce::Camera& camera,
    float        alpha
) {
    origin(camera, alpha);
    CameraReader::observeForward(camera.mForward);
}

} // namespace levi_levels
