use crate::terminal::Terminal;

const _SAMPLE_MAZE: &str = r#"
  * --- *         * --- *
 /       \       /       \
*         * --- *         *
 \       /       \       /
  * --- *         * --- *
         \       /
          * --- *
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Maze {
    cols: u16,
    rows: u16,
}

impl Maze {
    pub fn new(term: &Terminal) -> Self {
        let (term_width, term_height) = term.size();
        if term_width < 11 || term_height < 7 {
            return Self::empty();
        }

        Self {
            cols: (term_width - 3) / 8,
            rows: (term_height - 3) / 4,
        }
    }

    pub fn empty() -> Maze {
        Self { cols: 0, rows: 0 }
    }

    pub fn copy_from(&mut self, other: &Maze) {
        self.cols = other.cols;
        self.rows = other.rows;
    }

    pub fn draw(&self, term: &mut Terminal) {
        if self.cols == 0 || self.rows == 0 {
            return;
        }

        let height = self.rows * 4 + 3;
        for y in 0..height {
            let half_row = y / 2;
            if y % 2 == 0 {
                let indent = 2 - y % 4;
                term.goto(indent, y);

                for col in 0..self.cols {
                    if (col + half_row) % 2 == 0 {
                        term.write("* --- ");
                    } else {
                        term.write("*         ");
                    }
                }

                term.write("*");
            } else {
                term.goto(1, y);

                for col in 0..self.cols {
                    if (col + half_row) % 2 == 0 {
                        term.write("/       ");
                    } else {
                        term.write("\\       ");
                    }
                }

                if (self.cols + half_row) % 2 == 0 {
                    term.write("/");
                } else {
                    term.write("\\");
                }
            }
        }
    }
}
