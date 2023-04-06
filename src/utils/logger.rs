pub trait Logger<'a> {
    const NAMESPACE: &'a str;

    fn log(message: &str, color: &str) {
        println!("{color}[{}] {message}{RESET}", Self::NAMESPACE);
    }

    fn info(message: &str) {
        Self::log(message, BLUE);
    }

    fn debug(message: &str) {
        Self::log(message, GRAY);
    }

    fn success(message: &str) {
        Self::log(message, GREEN);
    }

    fn warning(message: &str) {
        Self::log(message, YELLOW);
    }

    fn error(message: &str) {
        Self::log(message, RED);
    }
}

const RED: &str = "\x1b[30m";
const YELLOW: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const GRAY: &str = "\x1b[90m";
const RESET: &str = "\x1b[0m";
