#include "input/KeyListener.h"

#include "bridge/RustBridge.h"

#include <ll/api/event/EventBus.h>
#include <ll/api/event/input/KeyInputEvent.h>

#include <windows.h>

namespace levi_levels {

namespace {

ll::event::ListenerPtr g_keyListener;

constexpr SHORT KEY_DOWN_BIT = static_cast<SHORT>(0x8000);

// the physical key state at the event, so a Ctrl release the game missed cannot stick, whatever thread
// delivers the event
uint8_t ctrlHeld() { return (GetAsyncKeyState(VK_CONTROL) & KEY_DOWN_BIT) ? 1 : 0; }

void onKey(ll::event::KeyInputEvent& event) {
    const LvHandle* handle = RustBridge::handle();
    if (!handle) return;
    uint8_t consume = 0;
    auto    code    = static_cast<uint32_t>(event.keyCode());
    if (lv_key(handle, code, event.isDown() ? 1 : 0, ctrlHeld(), &consume) != LV_STATUS_OK) return;
    if (consume) event.cancel();
}

} // namespace

void KeyListener::registerListener() {
    g_keyListener = ll::event::EventBus::getInstance().emplaceListener<ll::event::KeyInputEvent>(&onKey);
}

void KeyListener::unregisterListener() {
    if (!g_keyListener) return;
    ll::event::EventBus::getInstance().removeListener<ll::event::KeyInputEvent>(g_keyListener);
    g_keyListener = nullptr;
}

} // namespace levi_levels
