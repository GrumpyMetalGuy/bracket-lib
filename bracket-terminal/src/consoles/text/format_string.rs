use bracket_color::prelude::*;

#[derive(Debug)]
pub struct ColoredTextSpans {
    pub length: usize,
    pub spans: Vec<(RGBA, String)>,
}

fn find_color(col_name: &str) -> RGBA {
    if let Some(palette) = palette_color(&col_name) {
        palette
    } else {
        RGBA::from_u8(255, 255, 255, 255)
    }
}

impl ColoredTextSpans {
    pub fn new(text: &str) -> Self {
        let mut result = Self {
            length: 0,
            spans: Vec::new(),
        };
        let mut color_stack = Vec::new();

        for color_span in text.to_owned().split("#[") {
            if color_span.is_empty() {
                continue;
            }
            let mut col_text = color_span.splitn(2, ']');
            let col_name = col_text.next().unwrap();
            match col_text.next() {
                Some(text_span) => {
                    if !col_name.is_empty() {
                        color_stack.push(find_color(col_name));
                    } else {
                        color_stack.pop();
                    }
                    result.spans.push((
                        *color_stack
                            .last()
                            .unwrap_or(&RGBA::from_u8(255, 255, 255, 255)),
                        text_span.to_string(),
                    ));
                    result.length += text_span.chars().count();
                }
                None => {
                    // No closing ']': the whole segment is plain text.
                    result.spans.push((
                        *color_stack
                            .last()
                            .unwrap_or(&RGBA::from_u8(255, 255, 255, 255)),
                        col_name.to_string(),
                    ));
                    result.length += col_name.chars().count();
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::ColoredTextSpans;

    #[test]
    fn plain_text_is_kept() {
        let spans = ColoredTextSpans::new("hello");
        assert_eq!(spans.length, 5);
        assert_eq!(spans.spans.len(), 1);
        assert_eq!(spans.spans[0].1, "hello");
    }

    #[test]
    fn colored_text_keeps_text() {
        let spans = ColoredTextSpans::new("#[red]hi#[] there");
        assert_eq!(spans.length, 8);
        assert_eq!(spans.spans[0].1, "hi");
        assert_eq!(spans.spans[1].1, " there");
    }
}
