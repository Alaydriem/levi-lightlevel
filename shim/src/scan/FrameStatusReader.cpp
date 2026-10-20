#include "scan/FrameStatusReader.h"

#include <mc/client/game/ClientInstance.h>
#include <mc/client/options/IOptionRegistry.h>
#include <mc/client/player/LocalPlayer.h>
#include <mc/common/Brightness.h>
#include <mc/deps/core/math/Vec2.h>
#include <mc/world/level/BlockPos.h>
#include <mc/world/level/BlockSource.h>
#include <mc/world/level/dimension/Dimension.h>
#include <mc/world/level/dimension/DimensionType.h>

#include <windows.h>

namespace levi_levels {

double FrameStatusReader::nowSeconds() {
    static const double frequency = [] {
        LARGE_INTEGER value;
        QueryPerformanceFrequency(&value);
        return static_cast<double>(value.QuadPart);
    }();
    LARGE_INTEGER now;
    QueryPerformanceCounter(&now);
    return static_cast<double>(now.QuadPart) / frequency;
}

LvFrameStatus FrameStatusReader::read(ClientInstance& client) {
    LvFrameStatus status{};
    status.time_seconds = nowSeconds();
    if (!client.isWorldActive()) return status;
    status.in_world     = 1;
    LocalPlayer* player = client.getLocalPlayer();
    if (!player) return status;
    BlockPos           feet   = player->getFeetBlockPos();
    BlockSource const& region = player->getDimensionBlockSourceConst();
    status.has_player         = 1;
    status.menu_open          = client.isShowingMenu() ? 1 : 0;
    status.hud_hidden         = client.getOptions().getHideHud() ? 1 : 0;
    status.dimension          = static_cast<int32_t>(player->getDimensionId());
    status.feet_x             = feet.x;
    status.feet_y             = feet.y;
    status.feet_z             = feet.z;
    status.sky_darken         = static_cast<uint8_t>(region.getDimensionConst().mSkyDarken->mValue);
    status.yaw                = player->getRotation().y;
    return status;
}

} // namespace levi_levels
