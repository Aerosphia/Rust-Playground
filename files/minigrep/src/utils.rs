use std::process;

pub fn run_and_exit<F>(func: F, code: i32) -> !
where
    F: Fn(),
{
    func();
    process::exit(code);
}
