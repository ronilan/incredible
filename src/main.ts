import init, { main, content_columns, content_rows } from "../pkg/incredible.js";

/**
 * Dynamically adjusts the mobile viewport meta tag scale factor to force 
 * the terminal grid layout to zoom out and fit smaller device widths.
 */
function syncViewportBounds(): void {
  const meta = document.getElementById('viewport-meta') as HTMLMetaElement | null;
  if (!meta) return;

  const columns = content_columns();
  const rows = content_rows();
  if (columns === undefined && rows === undefined) return;

  const LINE_HEIGHT = 18;
  const JETBRAINS_MONO_RATIO = 600 / 1000;
  const CHAR_WIDTH = LINE_HEIGHT * JETBRAINS_MONO_RATIO;

  let scale = 1;

  if (columns !== undefined) {
    const content_width = columns * CHAR_WIDTH;

    // Track the actual physical screen dimensions independent of active browser layout zoom
    const isLandscape = window.matchMedia("(orientation: landscape)").matches;
    const device_width = isLandscape
      ? Math.max(window.screen.width, window.screen.height)
      : Math.min(window.screen.width, window.screen.height);

    // Evaluate scale using the immutable physical device width boundary
    if (content_width > device_width) {
      scale = Math.min(scale, device_width / content_width);
    }
  }

  if (rows !== undefined) {
    const content_height = rows * LINE_HEIGHT;
    const device_height = window.screen.height;

    // Evaluate scale using the immutable physical device height boundary
    if (content_height > device_height) {
      scale = Math.min(scale, device_height / content_height);
    }
  }

  // Force the browser layout engine to optically zoom out to match the content matrix
  meta.setAttribute(
    'content',
    `width=device-width, initial-scale=${scale}, minimum-scale=${scale}, maximum-scale=${scale}, user-scalable=no`
  );
}

/**
 * Some WASM event handlers call `preventDefault()` on synthetic or emulated
 * events (e.g. touch gestures that fire mouse events). The browser marks these
 * as `cancelable: false`, causing `preventDefault()` to throw. This patch
 * silently skips the call when the event is not cancelable.
 */
function patchWasmCancelableInterventions() {
    // Save the original native preventDefault method
    const originalPreventDefault = Event.prototype.preventDefault;

    // Globally override preventDefault for ALL event types
    Event.prototype.preventDefault = function() {
      // If the browser says this specific event instance cannot be canceled
      // (e.g. an emulated mouse/pointer event while a touch gesture is locked),
      // we do absolutely nothing and return early.
      if (this.cancelable === false) {
        return;
      }
      
      // Otherwise, let the browser handle it normally
      return originalPreventDefault.call(this);
    };
  }

await init();

// Attach listeners for orientation flips and window adjustments
window.addEventListener('resize', syncViewportBounds);
window.addEventListener('orientationchange', syncViewportBounds);

patchWasmCancelableInterventions();

main();
syncViewportBounds();
