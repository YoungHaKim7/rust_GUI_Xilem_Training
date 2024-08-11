use winit::{dpi::LogicalSize, error::EventLoopError, window::Window};
use xilem::{
    view::{
        button, flex, label, sized_box, Axis, CrossAxisAlignment, Flex, FlexExt, FlexSequence,
        FlexSpacer, MainAxisAlignment,
    },
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

#[derive(Copy, Clone)]
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

    fn on_delete(&mut self) {
        if self.result.is_some() {
            // Delete does not do anything with the result. Invalid action.
            return;
        }
        let mut num = self.get_current_number();
        if !num.is_empty() {
            num.remove(num.len() - 1);
            self.set_current_number(num);
        } // else, invalid action
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

fn centered_flex_row<State, Seq: FlexSequence<State>>(sequence: Seq) -> Flex<Seq, State> {
    flex(sequence)
        .direction(Axis::Horizontal)
        .cross_axis_alignment(CrossAxisAlignment::Center)
}

fn flex_row<State, Seq: FlexSequence<State>>(sequence: Seq) -> Flex<Seq, State> {
    flex(sequence)
        .direction(Axis::Horizontal)
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
        .gap(GRID_GAP)
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

fn operator_button(math_operator: MathOperator) -> impl WidgetView<Calculator> {
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
    flex((
        // Display
        centered_flex_row((
            FlexSpacer::Flex(0.1),
            display_label(data.numbers[0].as_ref()),
            // data.operation
            //     .as_ref()
            //     .map(|operation| display_label(operation.as_str())),
            // display_label(data.numbers[1].as_ref()),
            // data.result.is_some().then(|| display_label("=")),
            // data.result
            //     .as_ref()
            //     .map(|result| display_label(result.as_ref())),
            // FlexSpacer::Flex(0.1),
        ))
        .flex(1.0),
        FlexSpacer::Fixed(0.1),
        // Top row
        flex_row((
            expanded_button("CE", Calculator::clear_entry).flex(1.),
            // expanded_button("C", Calculator::clear_all).flex(1.),
            // expanded_button("DEL", Calculator::on_delete).flex(1.),
            // operator_button(MathOperator::Divide).flex(1.),
        )),
    ))
    .gap(GRID_GAP)
    .cross_axis_alignment(CrossAxisAlignment::Fill)
    .main_axis_alignment(MainAxisAlignment::End)
    .must_fill_major_axis(true)
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
