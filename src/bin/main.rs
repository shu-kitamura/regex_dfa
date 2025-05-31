use regex_dfa::parser::parse;

fn main() {
    let pattern = "abc(def|ghi)";
    match parse(pattern) {
        Ok(ast) => println!("Parsed AST: {:?}", ast),
        Err(e) => eprintln!("Error parsing pattern '{}': {}", pattern, e),
    }
}
