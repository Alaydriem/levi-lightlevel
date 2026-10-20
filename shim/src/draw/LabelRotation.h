#pragma once

#include <mc/deps/core/math/Quaternion.h>

#include <optional>

namespace levi_levels {

// the rotation Rust computes, held in the in-memory shape of std::optional<Quaternion>: x, y, z, w, then
// the engaged flag. The game's Quaternion has no exported constructor from components
class LabelRotation {
public:
    explicit LabelRotation(float const (&xyzw)[4]);

    std::optional<Quaternion> const& value() const;

private:
    alignas(4) float mQuat[4];
    // read only through the std::optional view of this object
    [[maybe_unused]] bool mEngaged;
    [[maybe_unused]] char mPad[3];
};

} // namespace levi_levels
