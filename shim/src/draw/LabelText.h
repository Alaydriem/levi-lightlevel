#pragma once

#include <cstdint>
#include <string>
#include <vector>

class ClientInstance;

namespace levi_levels {

// the 16 fixed strings "0" to "15" and their line widths in the game font, measured once
class LabelText {
public:
    // measures the strings on the first call; later calls do nothing
    static void prepare(ClientInstance& client);

    // value is clamped to 0..15
    static std::string const& text(uint8_t value);

    static std::vector<int> const& widths(uint8_t value);
};

} // namespace levi_levels
