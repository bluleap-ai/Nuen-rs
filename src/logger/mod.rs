mod defmt;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        {
            use core::fmt::Write;
            write!($crate::Printer, $($arg)*).ok();
        }
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        {
            use core::fmt::Write;
            writeln!($crate::Printer, $($arg)*).ok();
        }
    }};
}

/// Prints and returns the value of a given expression for quick and dirty
/// debugging.
// implementation adapted from `std::dbg`
#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `println!`
    // will be malformed.
    () => {
        $crate::println!("[{}:{}]", ::core::file!(), ::core::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

/// The printer that is used by the `print!` and `println!` macros.
pub struct Printer;

impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Printer::write_bytes(s.as_bytes());
        Ok(())
    }
}

impl Printer {
    /// Writes a byte slice to the configured output.
    pub fn write_bytes(bytes: &[u8]) {
        with(|| {
            PrinterImpl::write_bytes_assume_cs(bytes);
            PrinterImpl::flush();
        })
    }
}

type PrinterImpl = uart_printer::Printer;

mod uart_printer {
    use crate::{uart_flush, uart_tx};

    pub struct Printer;
    impl Printer {
        pub fn write_bytes_assume_cs(bytes: &[u8]) {
            uart_tx(bytes, bytes.len());
        }

        pub fn flush() {
            uart_flush();
        }
    }
}

#[inline]
fn with<R>(f: impl FnOnce() -> R) -> R {
    return critical_section::with(|_| f());
}

pub fn init_logger(level: log::LevelFilter) {
    unsafe {
        log::set_logger_racy(&Printer).unwrap();
        log::set_max_level_racy(level);
    }
}

impl log::Log for Printer {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    #[allow(unused)]
    fn log(&self, record: &log::Record) {
        if !self.enabled(&record.metadata()) {
            return;
        }

        const RESET: &str = "\u{001B}[0m";
        const RED: &str = "\u{001B}[31m";
        const GREEN: &str = "\u{001B}[32m";
        const YELLOW: &str = "\u{001B}[33m";
        const BLUE: &str = "\u{001B}[34m";
        const CYAN: &str = "\u{001B}[35m";

        let color = match record.level() {
            log::Level::Error => RED,
            log::Level::Warn => YELLOW,
            log::Level::Info => GREEN,
            log::Level::Debug => BLUE,
            log::Level::Trace => CYAN,
        };

        let reset = RESET;

        println!("{}{} - {}{}", color, record.level(), record.args(), reset);
    }

    fn flush(&self) {
        PrinterImpl::flush();
    }
}
