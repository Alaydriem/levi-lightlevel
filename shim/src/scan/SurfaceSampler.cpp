#include "scan/SurfaceSampler.h"

#include <mc/common/Brightness.h>
#include <mc/world/level/BlockPos.h>
#include <mc/world/level/BlockSource.h>
#include <mc/world/level/block/Block.h>
#include <mc/world/level/block/BlockType.h>
#include <mc/world/level/block/BrightnessPair.h>
#include <mc/world/phys/AABB.h>

#include <algorithm>

namespace levi_levels {

namespace {

constexpr uchar FACE_UP = 1;

// air, or a block whose collision shape is empty: flowers, torches, short grass
bool passable(BlockSource const& region, Block const& block, BlockPos const& pos) {
    if (block.isAir()) return true;
    AABB shape = block.getBlockType().getCollisionShape(block, region, pos, {});
    return shape.min.x == shape.max.x && shape.min.y == shape.max.y && shape.min.z == shape.max.z;
}

} // namespace

uint32_t SurfaceSampler::sample(BlockSource const& region, LvColumnRequest const& request, LvSurfaceSample* out) {
    // the block above the top one read must still be inside the dimension
    int      yMin  = std::max<int>(request.y_min, region.getMinHeight());
    int      yMax  = std::min<int>(request.y_max, region.getMaxHeight() - 2);
    uint32_t count = 0;
    if (yMin > yMax) return count;
    Block const* block = &region.getBlock(BlockPos{request.x, yMin, request.z});
    for (int y = yMin; y <= yMax && count < LV_MAX_COLUMN_SURFACES; ++y) {
        BlockPos     abovePos{request.x, y + 1, request.z};
        Block const& above = region.getBlock(abovePos);
        if (block->canProvideFullSupport(FACE_UP) && passable(region, above, abovePos)) {
            BrightnessPair light = region.getBrightnessPair(abovePos);
            out[count++]         = LvSurfaceSample{
                request.x,
                y,
                request.z,
                static_cast<uint8_t>(light.block->mValue),
                static_cast<uint8_t>(light.sky->mValue),
            };
        }
        block = &above;
    }
    return count;
}

} // namespace levi_levels
