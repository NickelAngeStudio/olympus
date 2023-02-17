use crate::kleio::display::screen::{KScreen, KScreenListError};

/// Private function that fetch Wayland display server screens.
pub(crate) fn get_wayland_screen() -> Result<Vec<KScreen>, KScreenListError>{
    Err(KScreenListError::FetchScreenListError)
}