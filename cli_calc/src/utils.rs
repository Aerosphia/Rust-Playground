use std::process;

/// Runs the specified closure function and then exits with the code given.
///
/// # Examples
/// ```
/// utils::run_and_exit(|| eprintln!("Application error: {err}"), 1);
/// ```
#[allow(dead_code)]
pub fn run_and_exit<F>(func: F, code: i32) -> !
where
    F: Fn(),
{
    func();
    process::exit(code);
}
