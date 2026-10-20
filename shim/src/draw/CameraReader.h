#pragma once

#include "levi_levels.h"

#include <glm/glm.hpp>

namespace levi_levels {

// the camera the latest frame is drawn from: where it is and the direction it faces. It is the render
// camera, so third person and other mods' free cameras are covered
class CameraReader {
public:
    static void observePosition(glm::vec3 const& position);

    static void observeForward(glm::vec3 const& forward);

    // false until both a position and a direction are seen, and after reset
    static bool read(LvCamera& out);

    static void reset();
};

} // namespace levi_levels
