/// Contains tests functions for an implementation of KWindowManager
pub mod tests;

/// Contains tests for KWindowManagerWayland
#[cfg(test)]
#[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
pub mod wayland;

/// Contains tests for KWindowManagerX11
#[cfg(test)]
#[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
pub mod x11;