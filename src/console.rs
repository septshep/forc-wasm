use wasm_bindgen::JsValue;
use web_sys::console;

/// The `console.log()` function.
#[inline]
pub fn log(data: impl Into<JsValue>) {
    console::log_1(&data.into());
}

/// The `console.error()` function.
#[inline]
pub fn error(data: impl Into<JsValue>) {
    console::error_1(&data.into());
}

/// The `console.warn()` function.
#[inline]
#[allow(dead_code)]
pub fn warn(data: impl Into<JsValue>) {
    console::warn_1(&data.into());
}

/// The `console.info()` function.
#[inline]
#[allow(dead_code)]
pub fn info(data: impl Into<JsValue>) {
    console::info_1(&data.into());
}

/// The `console.debug()` function.
#[inline]
#[allow(dead_code)]
pub fn debug(data: impl Into<JsValue>) {
    console::debug_1(&data.into());
}

/// The `console.trace()` function.
#[inline]
#[allow(dead_code)]
pub fn trace(data: impl Into<JsValue>) {
    console::trace_1(&data.into());
}

/// The `console.clear()` function.
#[inline]
#[allow(dead_code)]
pub fn clear() {
    console::clear();
}

/// The `console.count()` function.
#[inline]
#[allow(dead_code)]
pub fn count() {
    console::count();
}
