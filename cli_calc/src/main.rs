use cli_calc::Input;

mod utils;

fn main() {
    let mut input_struct: Input = Input::default();

    input_struct = input_struct
        .get()
        .unwrap_or_else(|err| utils::run_and_exit(|| eprintln!("Error: {err}"), 1));

    match input_struct.calc() {
        Ok(value) => println!("Result: {value}"),
        Err(err) => eprintln!("Error: {err}"),
    }
}
