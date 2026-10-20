#include "draw/LabelText.h"

#include <mc/client/game/ClientInstance.h>
#include <mc/client/gui/Font.h>
#include <mc/client/gui/FontHandle.h>

#include <algorithm>
#include <array>

namespace levi_levels {

namespace {

constexpr size_t VALUES = 16;

std::array<std::string, VALUES>      g_texts;
std::array<std::vector<int>, VALUES> g_widths;
bool                                 g_ready = false;

size_t index(uint8_t value) { return std::min<size_t>(value, VALUES - 1); }

} // namespace

void LabelText::prepare(ClientInstance& client) {
    if (g_ready) return;
    FontHandle handle = client.getFontHandle();
    Font&      font   = handle.getFont();
    for (size_t i = 0; i < VALUES; ++i) {
        g_texts[i]  = std::to_string(i);
        g_widths[i] = {font.getLineLength(g_texts[i], 1.0f, false)};
    }
    g_ready = true;
}

std::string const& LabelText::text(uint8_t value) { return g_texts[index(value)]; }

std::vector<int> const& LabelText::widths(uint8_t value) { return g_widths[index(value)]; }

} // namespace levi_levels
