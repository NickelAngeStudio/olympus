/// # Re-export for Public API
#[doc(inline)]
pub use window::KWindow as KWindow;
pub use property::KWindowProperty as KWindowProperty;
pub use property::KCursorMode as KCursorMode;
pub use property::KCursorProperty as KCursorProperty;
pub use property::KWindowFullscreenMode as KWindowFullscreenMode;

/// Window and cursor properties.
#[doc(hidden)]
pub mod property;

/// [KWindow] definition.
#[doc(hidden)]
pub mod window;

/// Hardware screen details and supported resolutions.
pub mod screen;

/// Elements relatives to [KWindow] events and handling.
pub mod event;

/// Linux implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
#[doc(hidden)]
pub mod linux;

/// Windows shell implementations of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "windows"))]
#[doc(hidden)]
pub mod shell;

/// Android implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "android"))]
#[doc(hidden)]
pub mod android;

/// IOS implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "ios"))]
#[doc(hidden)]
pub mod ios;

/// MacOS implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "macos"))]
#[doc(hidden)]
pub mod macos;

/// Web assembly implementation of KWindow
#[cfg(target_family = "wasm")]
#[doc(hidden)]
pub mod wasm;