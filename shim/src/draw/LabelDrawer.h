#pragma once

class ScreenContext;
struct NameTagRenderObjectCollection;

namespace levi_levels {

// appends Rust's draw list to the game's name-tag list as flat, depth-tested text
class LabelDrawer {
public:
    // runs on the render thread right after the game built its own tags
    static void append(ScreenContext& screen, NameTagRenderObjectCollection& tags);
};

} // namespace levi_levels
