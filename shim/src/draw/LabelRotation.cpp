#include "draw/LabelRotation.h"

#include <cstring>

namespace levi_levels {

static_assert(sizeof(LabelRotation) == sizeof(std::optional<Quaternion>));

LabelRotation::LabelRotation(float const (&xyzw)[4]) : mEngaged(true), mPad{} {
    std::memcpy(mQuat, xyzw, sizeof(mQuat));
}

std::optional<Quaternion> const& LabelRotation::value() const {
    return *reinterpret_cast<std::optional<Quaternion> const*>(this);
}

} // namespace levi_levels
