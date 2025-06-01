use gpui::{prelude::*, *};

const BACKGROUND_COLOR: u32 = 0x1E2027;
const FOREGROUND_COLOR: u32 = 0xE6E6E6;
const BORDER_COLOR: u32 = 0x2D3039;
const BUTTON_BACKGROUND_COLOR: u32 = 0x3B82F6;
const BUTTON_FOREGROUND_COLOR: u32 = 0xFFFFFF;
const BUTTON_HOVER_COLOR: u32 = 0x60A5FA;

struct Calculator {
    expression: String,
    _solution: f64,
}

impl Calculator {
    fn render_calculator(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .justify_center()
            .items_center()
            .text_xl()
            .p_4()
            .border_2()
            .rounded_lg()
            .border_color(rgb(BORDER_COLOR))
            .w_80()
            .text_color(rgb(FOREGROUND_COLOR))
            .child(format!("Expression: {}", self.expression))
            .gap_5()
            .child(format!("Solution: {}", self._solution))
    }

    fn handle_number_click(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
        number: u8,
    ) {
        self.expression.push_str(&number.to_string());
        cx.notify();
    }

    fn handle_operator_click(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
        operator: &str,
    ) {
        if self.expression.is_empty() && operator != "-" {
            return;
        }

        if !self.expression.is_empty() {
            let last_char = self.expression.chars().last().unwrap();
            if (last_char == '+' || last_char == '-' || last_char == '*' || last_char == '/')
                && operator != "-"
            {
                return;
            }
        }

        self.expression.push_str(operator);
        cx.notify();
    }

    fn render_operator_buttons(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_wrap()
            .gap_4()
            .justify_center()
            .p_4()
            .w_80()
            .children(vec!["+", "-", "*", "/"].iter().map(|&operator| {
                let operator_owned = operator.to_string(); // Create an owned copy for the closure
                div()
                    .text_xl()
                    .border_2()
                    .p_4()
                    .rounded_lg()
                    .w_12()
                    .h_12()
                    .justify_center()
                    .items_center()
                    .text_center()
                    .cursor_pointer()
                    .border_color(rgb(BORDER_COLOR))
                    .text_color(rgb(BUTTON_FOREGROUND_COLOR))
                    .bg(rgb(BUTTON_BACKGROUND_COLOR))
                    .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, event, window, cx| {
                            this.handle_operator_click(event, window, cx, &operator_owned)
                        }),
                    )
                    .child(operator.to_string())
            }))
    }

    fn handle_equals_click(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.expression.is_empty() {
            return;
        }

        match self.evaluate_expression(&self.expression) {
            Ok(result) => {
                self._solution = result;
                cx.notify();
            }
            Err(_) => {
                cx.notify();
            }
        }
    }

    fn evaluate_expression(&self, expr: &str) -> Result<f64, String> {
        let tokens = self.tokenize(expr)?;
        self.parse_expression(&tokens, 0).map(|(result, _)| result)
    }

    fn tokenize(&self, expr: &str) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        let mut num_buffer = String::new();

        for c in expr.chars() {
            if c.is_digit(10) || c == '.' {
                num_buffer.push(c);
            } else if c == '+' || c == '-' || c == '*' || c == '/' {
                if !num_buffer.is_empty() {
                    tokens.push(num_buffer.clone());
                    num_buffer.clear();
                }
                tokens.push(c.to_string());
            } else if !c.is_whitespace() {
                return Err(format!("Invalid character in expression: {}", c));
            }
        }

        if !num_buffer.is_empty() {
            tokens.push(num_buffer);
        }

        Ok(tokens)
    }

    fn parse_expression(&self, tokens: &[String], pos: usize) -> Result<(f64, usize), String> {
        if pos >= tokens.len() {
            return Err("Unexpected end of expression".to_string());
        }

        let (mut left, mut current_pos) = self.parse_term(tokens, pos)?;

        while current_pos < tokens.len() {
            let op = &tokens[current_pos];
            if op != "+" && op != "-" {
                break;
            }

            current_pos += 1;
            let (right, new_pos) = self.parse_term(tokens, current_pos)?;
            current_pos = new_pos;

            if op == "+" {
                left += right;
            } else {
                left -= right;
            }
        }

        Ok((left, current_pos))
    }

    fn parse_term(&self, tokens: &[String], pos: usize) -> Result<(f64, usize), String> {
        if pos >= tokens.len() {
            return Err("Unexpected end of term".to_string());
        }

        let (mut left, mut current_pos) = self.parse_factor(tokens, pos)?;

        while current_pos < tokens.len() {
            let op = &tokens[current_pos];
            if op != "*" && op != "/" {
                break;
            }

            current_pos += 1;
            let (right, new_pos) = self.parse_factor(tokens, current_pos)?;
            current_pos = new_pos;

            if op == "*" {
                left *= right;
            } else {
                if right == 0.0 {
                    return Err("Division by zero".to_string());
                }
                left /= right;
            }
        }

        Ok((left, current_pos))
    }

    fn parse_factor(&self, tokens: &[String], pos: usize) -> Result<(f64, usize), String> {
        if pos >= tokens.len() {
            return Err("Unexpected end of factor".to_string());
        }

        let token = &tokens[pos];
        match token.parse::<f64>() {
            Ok(num) => Ok((num, pos + 1)),
            Err(_) => Err(format!("Invalid number: {}", token)),
        }
    }

    fn render_equals_sign(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xl()
            .border_2()
            .p_4()
            .rounded_lg()
            .w_12()
            .h_12()
            .justify_center()
            .items_center()
            .text_center()
            .cursor_pointer()
            .border_color(rgb(BORDER_COLOR))
            .text_color(rgb(BUTTON_FOREGROUND_COLOR))
            .bg(rgb(BUTTON_BACKGROUND_COLOR))
            .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event, window, cx| {
                    this.handle_equals_click(event, window, cx)
                }),
            )
            .child("=")
    }

    fn render_num_buttons(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_wrap()
            .gap_4()
            .justify_center()
            .p_4()
            .w_80()
            .children((0..10).map(|num| {
                div()
                    .text_xl()
                    .border_2()
                    .p_4()
                    .rounded_lg()
                    .w_12()
                    .h_12()
                    .justify_center()
                    .items_center()
                    .text_center()
                    .cursor_pointer()
                    .border_color(rgb(BORDER_COLOR))
                    .text_color(rgb(BUTTON_FOREGROUND_COLOR))
                    .bg(rgb(BUTTON_BACKGROUND_COLOR))
                    .hover(|style| style.bg(rgb(BUTTON_HOVER_COLOR)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, event, window, cx| {
                            this.handle_number_click(event, window, cx, num)
                        }),
                    )
                    .child(num.to_string())
            }))
    }
}

impl Render for Calculator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(BACKGROUND_COLOR))
            .size_full()
            .items_center()
            .justify_center()
            .gap_6()
            .child(self.render_calculator())
            .child(self.render_num_buttons(cx))
            .child(self.render_operator_buttons(cx))
            .child(self.render_equals_sign(cx))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_cx| Calculator {
                    expression: String::new(),
                    _solution: 0.0,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
