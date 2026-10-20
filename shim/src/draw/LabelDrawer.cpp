#include "draw/LabelDrawer.h"

#include "bridge/RustBridge.h"
#include "draw/CameraReader.h"
#include "draw/LabelRing.h"
#include "draw/LabelRotation.h"
#include "draw/LabelText.h"
#include "draw/NameTagList.h"
#include "mod/LevelsMod.h"
#include "scan/FrameStatusReader.h"
#include "scan/ScanDriver.h"

#include <mc/client/gui/screens/ScreenContext.h>
#include <mc/client/renderer/BaseActorRenderer.h>
#include <mc/deps/core/math/Color.h>
#include <mc/deps/core/math/Vec3.h>
#include <mc/deps/minecraft_renderer/objects/NameTagRenderObject.h>
#include <mc/deps/minecraft_renderer/objects/NameTagRenderObjectCollection.h>

namespace levi_levels {

namespace {

constexpr double MICROS_PER_SECOND = 1000000.0;

BaseActorRenderer* g_textRenderer = nullptr;
bool               g_layoutBroken = false;

void build(ScreenContext& screen, LvDrawView const& view, NameTagRenderObject* cursor) {
    LabelRotation           rotation(view.rotation);
    mce::MaterialPtr const* tagMat  = &*g_textRenderer->mDepthTestedNameTagMat;
    mce::MaterialPtr const* textMat = &*g_textRenderer->mDepthTestedNameTextMat;
    mce::Color const        noBackground(0.0f, 0.0f, 0.0f, 0.0f);
    for (uint32_t i = 0; i < view.count; ++i) {
        LvLabel const& label = view.labels[i];
        new (cursor + i) NameTagRenderObject(BaseActorRenderer::_extractRenderTextObject(
            screen.tessellator,
            LabelText::text(label.value),
            LabelText::widths(label.value),
            Vec3{label.x, label.y, label.z},
            mce::Color(label.r, label.g, label.b, label.a),
            tagMat,
            textMat,
            noBackground,
            view.scale,
            rotation.value(),
            0.0f
        ));
    }
}

// a list in a layout this build does not expect stops drawing for the session with one error line,
// instead of corrupting the game's list
std::optional<NameTagList> openList(NameTagRenderObjectCollection& tags) {
    auto list = NameTagList::open(tags);
    if (list) return list;
    g_layoutBroken = true;
    LevelsMod::getInstance().getSelf().getLogger().error(
        "levi-levels stops drawing: the name-tag list layout is not the one this build expects"
    );
    return std::nullopt;
}

void appendTo(ScreenContext& screen, NameTagList& list, LvDrawView const& view, ClientInstance& client) {
    if (view.count == 0) return;
    LabelText::prepare(client);
    if (!g_textRenderer) g_textRenderer = new BaseActorRenderer();
    NameTagRenderObject* storage = LabelRing::next(list.size() + view.count);
    list.moveTo(storage, view.count);
    build(screen, view, storage + list.size() - view.count);
}

} // namespace

void LabelDrawer::append(ScreenContext& screen, NameTagRenderObjectCollection& tags) {
    const LvHandle* handle = RustBridge::handle();
    ClientInstance* client = ScanDriver::client();
    if (!handle || !client || g_layoutBroken) return;
    double start = FrameStatusReader::nowSeconds();
    auto   list  = openList(tags);
    if (!list) return;
    LvCamera   camera{};
    LvCamera*  seen = CameraReader::read(camera) ? &camera : nullptr;
    LvDrawView view{};
    if (lv_draw_list(handle, static_cast<uint32_t>(list->size()), seen, &view) != LV_STATUS_OK) return;
    appendTo(screen, *list, view, *client);
    float drawMicros = static_cast<float>((FrameStatusReader::nowSeconds() - start) * MICROS_PER_SECOND);
    lv_report_timing(handle, ScanDriver::lastSampleMicros(), drawMicros, LabelRing::residentBytes());
}

} // namespace levi_levels
