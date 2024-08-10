use winit::{dpi::LogicalSize, error::EventLoopError, window::Window};
use xilem::{
    view::{button, flex, label, sized_box, FlexExt, FlexSpacer},
    EventLoop, EventLoopBuilder, WidgetView, Xilem,
};

const DISPLAY_FONT_SIZE: f32 = 30.;
const GRID_GAP: f64 = 2.;

struct Calculator {
    current_num_index: usize,
    clear_current_on_input: bool,
    numbers: [String; 2],
    result: Option<String>,
    operation: Option<String>,
}

#[derive(Debug, Clone)]
enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl MathOperator {
    fn as_str(&self) -> &'static str {
        match self {
            MathOperator::Add => "+",
            MathOperator::Subtract => "-",
            MathOperator::Multiply => "x",
            MathOperator::Divide => "÷ ",
        }
    }
}

impl Calculator {
    fn clear_all(&mut self) {
        self.current_num_index = 0;
        self.result = None;
        self.operation = None;
        for num in self.numbers.iter_mut() {
            *num = "".into();
        }
    }

    fn clear_entry(&mut self) {
        self.clear_current_on_input = false;
        if self.result.is_some() {
            self.clear_all();
            return;
        }
    }

    fn on_equals(&mut self) {
        todo!();
    }

    fn on_entered_digit(&mut self, digit: &str) {
        if self.result.is_some() {
            self.clear_all();
        } else if self.clear_current_on_input {
            self.clear_entry();
        }

        let mut num = self.get_current_number();

        if digit == "." {
            if num.contains('.') {
                return;
            }
        } else {
            num += digit;
        }
        self.set_current_number(num)
    }

    fn get_current_number(&mut self) -> String {
        self.numbers[self.current_num_index].clone()
    }

    fn set_current_number(&mut self, new_num: String) {
        self.numbers[self.current_num_index] = new_num;
    }

    fn on_entered_operator(&mut self, operator: MathOperator) {
        // self.clear_current_on_input = false;
        // if self.operation.is_some() && !self.numbers[1].is_empty() {
        //     if self.result.is_none() {
        //         self.on_equals();
        //     }
        //     todo!();
        // }
        // self.operation = Some(operator);
        todo!();
    }
}

fn display_label(text: &str) -> impl WidgetView<Calculator> {
    label(text).text_size(DISPLAY_FONT_SIZE)
}

fn expanded_button(
    text: &str,
    callback: impl Fn(&mut Calculator) + Send + Sync + 'static,
) -> impl WidgetView<Calculator> + '_ {
    sized_box(button(text, callback)).expand()
}

fn digit_button(digit: &'static str) -> impl WidgetView<Calculator> {
    expanded_button(digit, |data: &mut Calculator| {
        data.on_entered_digit(digit);
    })
}

fn operator_button(math_operator: MathOperator) -> WidgetView<Calculator> {
    expanded_button(math_operator.as_str(), move |data: &mut Calculator| {
        data.on_entered_operator(math_operator);
    })
}

// impl statics dispatch
fn app_logic(data: &mut Calculator) -> impl WidgetView<Calculator> {
    let num_row = |nums: [&'static str; 3], operator| {
        flex_row((
            nums.map(|num| digit_button(num).flex(1.)),
            operator_button(operator).flex(1.),
        ))
    };
    flex(
        (
            // Display
            centered_flex_row((
                FlexSpacer::Flex(0.1),
                display_label(data.numbers[0].as_ref()),
                data.operation
                    .map(|operation| display_label(operation.as_str())),
            ))
        ),
    )
}

fn run(event_loop: EventLoopBuilder) -> Result<(), EventLoopError> {
    let data = Calculator {
        current_num_index: 0,
        clear_current_on_input: false,
        numbers: ["".into(), "".into()],
        result: None,
        operation: None,
    };

    let app = Xilem::new(data, app_logic);
    let min_window_size = LogicalSize::new(200.0, 200.);
    let window_size = LogicalSize::new(400., 500.);
    let windows_attributes = Window::default_attributes()
        .with_title("Calculator")
        .with_resizable(true)
        .with_min_inner_size(min_window_size)
        .with_inner_size(window_size);
    app.run_windowed_in(event_loop, windows_attributes)?;
    Ok(())
}

fn main() -> Result<(), EventLoopError> {
    run(EventLoop::with_user_event())
}
