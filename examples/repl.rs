use gon::*;

fn main() -> Result<(), String> {
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input);
        let value_res = parse(input.trim());
        match value_res {
            Ok(value) => {
                dbg!(value);
            }
            Err(e) => eprintln!("{e}"),
        }
        input.clear();
    }
}
