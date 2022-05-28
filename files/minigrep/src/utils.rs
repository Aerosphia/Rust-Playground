use std::process;

pub fn run_and_exit<F: Fn()>(func: F, code: i32) -> ! {
    func();
    process::exit(code);
}
