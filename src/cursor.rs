use ropey::Rope;

#[derive(Default)]
pub struct Cursor {
    pub row: usize,
    pub column: usize,
}

impl Cursor {
    pub fn char_index(&self, content: &Rope) -> usize {
        if self.row >= content.len_lines() {
            return content.len_chars();
        }

        let line_start = content.line_to_char(self.row);
        let line = content.line(self.row);
        let line_len = line.len_chars();
        let line_len_without_newline =
            line_len.saturating_sub(usize::from(line_len > 0 && line.char(line_len - 1) == '\n'));

        line_start + self.column.min(line_len_without_newline)
    }

    pub fn move_to_char_index(&mut self, content: &Rope, char_index: usize) {
        let char_index = char_index.min(content.len_chars());
        let row = content.char_to_line(char_index);
        let line_start = content.line_to_char(row);

        self.column = char_index - line_start;
    }
}
