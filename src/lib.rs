mod big_num;
mod common;
mod frac;
mod parser;

use std::error::Error;
pub fn eval_to_string(input: String) -> Result<String, Box<dyn Error>> {
    parser::eval_to_string(input)
}
