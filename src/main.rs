extern crate tokenizer;

use tokenizer::lexer;

fn main() {
    let res = lexer::tokenize("\tif (i == j)\n\t\tz = 1234;\n\telse\n\t\tz = 145;");
}
