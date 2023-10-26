use rs_xterm_js::Terminal;
use std::io::Result;

#[allow(unused)]
pub(crate) fn is_raw_mode_enabled() -> bool {
    true
}

/// Returns the terminal size `(columns, rows)`.
///
/// The top left cell is represented `(1, 1)`.
pub fn size(term: &Terminal) -> Result<(u16, u16)> {
    Ok((term.cols(), term.rows()))
}
