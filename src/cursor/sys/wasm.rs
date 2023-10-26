use rs_xterm_js::Terminal;
use std::io;

/// Returns the cursor position (column, row).
///
/// The top left cell is represented `(0, 0)`.
pub fn position(term: &Terminal) -> io::Result<(u16, u16)> {
    let buf = term.buffer().active();
    Ok((buf.cursor_x(), buf.cursor_y()))
}
