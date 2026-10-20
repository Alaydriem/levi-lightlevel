#pragma once

class ClientInstance;

namespace levi_levels {

// the per-tick round trip: read the game, ask Rust which columns to sample, sample them, hand them back
class ScanDriver {
public:
    static void tick(ClientInstance& client);

    // the client of the last tick; null before the first tick and after reset
    static ClientInstance* client();

    // microseconds the last tick spent sampling columns
    static float lastSampleMicros();

    static void reset();
};

} // namespace levi_levels
