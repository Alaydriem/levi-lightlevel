#pragma once

namespace levi_levels {

// forwards key events to Rust and cancels the ones Rust consumes
class KeyListener {
public:
    static void registerListener();
    static void unregisterListener();
};

} // namespace levi_levels
