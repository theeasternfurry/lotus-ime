use vi::{VNI, TELEX, transform_buffer};

#[derive(Debug, PartialEq)]
pub enum InputMode {
    VNI,
    TELEX,
}

#[derive(Debug)]
pub struct Input {
    pub mode: InputMode,
}

impl Input {
    pub fn new() -> Self {
        Input { mode: InputMode::VNI }
    }

    pub fn return_text(&self) -> String {
        let inputs = vec![
            "viet65",
            "nam"
        ];
        
        let mut result = String::new();
        for input in inputs {
            if self.mode == InputMode::VNI {
                transform_buffer(&VNI, input.chars(), &mut result);
            } else {
                transform_buffer(&TELEX, input.chars(), &mut result);
            }
            result.push(' ');
        }

        return result
    } 
}
