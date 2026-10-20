#pragma once

#include "levi_levels.h"

class ClientInstance;

namespace levi_levels {

// reads the game state Rust needs for one frame; read-only, it never changes the game
class FrameStatusReader {
public:
    // with no active world only the time is set; in a world with no local player, has_player is 0
    static LvFrameStatus read(ClientInstance& client);

    static double nowSeconds();
};

} // namespace levi_levels
