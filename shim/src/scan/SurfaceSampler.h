#pragma once

#include "levi_levels.h"

#include <cstdint>

class BlockSource;

namespace levi_levels {

// reads one column of the client's own BlockSource: the blocks a mob could stand on, with the light above them
class SurfaceSampler {
public:
    // writes at most LV_MAX_COLUMN_SURFACES samples into out and returns how many
    static uint32_t sample(BlockSource const& region, LvColumnRequest const& request, LvSurfaceSample* out);
};

} // namespace levi_levels
