extern crate omniglot;

use omniglot::dictionary::parsers::simple as simple_parser;

fn main() {
    let dict_file = "src/assets/english-italian.txt";
    simple_parser::read(dict_file);
}