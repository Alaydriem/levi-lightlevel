#include "draw/CameraReader.h"

#include <mutex>

namespace levi_levels {

namespace {

std::mutex g_mutex;
LvCamera   g_camera{};
bool       g_hasPosition = false;
bool       g_hasForward  = false;

} // namespace

void CameraReader::observePosition(glm::vec3 const& position) {
    std::lock_guard lock(g_mutex);
    g_camera.x    = position.x;
    g_camera.y    = position.y;
    g_camera.z    = position.z;
    g_hasPosition = true;
}

void CameraReader::observeForward(glm::vec3 const& forward) {
    std::lock_guard lock(g_mutex);
    g_camera.forward_x = forward.x;
    g_camera.forward_y = forward.y;
    g_camera.forward_z = forward.z;
    g_hasForward       = true;
}

bool CameraReader::read(LvCamera& out) {
    std::lock_guard lock(g_mutex);
    out = g_camera;
    return g_hasPosition && g_hasForward;
}

void CameraReader::reset() {
    std::lock_guard lock(g_mutex);
    g_hasPosition = false;
    g_hasForward  = false;
}

} // namespace levi_levels
