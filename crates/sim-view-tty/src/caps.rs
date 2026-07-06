//! Terminal surface capability presets for the tty view/edit surface.
//!
//! A surface is described by its advertised [`SurfaceCaps`]; the projection
//! ranker reads those caps to fit a scene. The `cli` preset is keyboard-only
//! ANSI; the `tui` preset adds pointer input and a richer palette.

use sim_lib_view::SurfaceCaps;

/// Builds the `cli` surface capabilities with `client_id` set.
///
/// A `cli` surface is the baseline keyboard-only ANSI terminal. Panics only if
/// the built-in `cli` preset is missing, which is a build-time invariant of
/// [`sim_lib_view::surface`].
pub fn cli_caps(client_id: &str) -> SurfaceCaps {
    SurfaceCaps::from_preset("cli", client_id).expect("cli is a built-in surface preset")
}

/// Builds the `tui` surface capabilities with `client_id` set.
///
/// A `tui` surface extends the `cli` baseline with pointer input and a richer
/// (ansi256) palette. Panics only if the built-in `tui` preset is missing.
pub fn tui_caps(client_id: &str) -> SurfaceCaps {
    SurfaceCaps::from_preset("tui", client_id).expect("tui is a built-in surface preset")
}
