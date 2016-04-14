extern crate tokenizer;

use tokenizer::lexer;

fn main() {
    let s = "\tif (i == 1)\n\t\tx = foo();\n\telse if (i == 5)\n\t\tx = i;";
    let res = lexer::tokenize(s);
    //let res = lexer::tokenize("\tif (i == j)\n\t\tz = 1234;\n\telse\n\t\tz = 145;");
    println!("tokenize - {:?}", res);
}
