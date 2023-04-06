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
// const WHITE: &str = "\x1b[37m";

/*
enum Color {
    black: ("\x1b[30m"),
    red : "\x1b[31m",
    green : "\x1b[32m",
    yellow : "\x1b[33m",
    blue : "\x1b[34m",
    magenta : "\x1b[35m",
    cyan : "\x1b[36m",
    white : "\x1b[37m",
    gray : "\x1b[90m",

    reset : "\x1b[0m",
    bright : "\x1b[1m",
    dim : "\x1b[2m",
    underscore : "\x1b[4m",
    blink : "\x1b[5m",
    reverse : "\x1b[7m",
    hidden : "\x1b[8m",
}
*/
