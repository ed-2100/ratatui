use crate::prelude::*;

/// A widget to clear/reset a certain area to allow overdrawing (e.g. for popups).
///
/// This widget **cannot be used to clear the terminal on the first render** as `ratatui` assumes
/// the render area is empty. Use [`crate::Terminal::clear`] instead.
///
/// # Examples
///
/// ```
/// use ratatui::{prelude::*, widgets::*};
///
/// fn draw_on_clear(f: &mut Frame, area: Rect) {
///     let block = Block::bordered().title("Block");
///     f.render_widget(Clear, area); // <- this will clear/reset the area first
///     f.render_widget(block, area); // now render the block widget
/// }
/// ```
///
/// # Popup Example
///
/// For a more complete example how to utilize `Clear` to realize popups see
/// the example `examples/popup.rs`
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Clear;

impl Widget for Clear {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Clear {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y).reset();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_buffer_eq;

    #[test]
    fn render() {
        let mut buf = Buffer::with_lines(vec!["xxxxxxxxxxxxxxx"; 7]);
        let clear = Clear;
        clear.render(Rect::new(1, 2, 3, 4), &mut buf);
        assert_buffer_eq!(
            buf,
            Buffer::with_lines(vec![
                "xxxxxxxxxxxxxxx",
                "xxxxxxxxxxxxxxx",
                "x   xxxxxxxxxxx",
                "x   xxxxxxxxxxx",
                "x   xxxxxxxxxxx",
                "x   xxxxxxxxxxx",
                "xxxxxxxxxxxxxxx",
            ])
        );
    }
}
