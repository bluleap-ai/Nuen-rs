use super::PrinterImpl;

static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

#[defmt::global_logger]
pub struct Logger;
unsafe impl defmt::Logger for Logger {
    fn acquire() {
        // If not disabled, write a non-UTF8 sequence to indicate the start of a defmt
        // frame. We need this to distinguish defmt frames from other data that
        // might be written to the printer.
        do_write(&[0xFF, 0x00]);

        // safety: accessing the `static mut` is OK because we have acquired a critical
        // section.
        unsafe { ENCODER.start_frame(do_write) }
    }

    unsafe fn release() {
        // safety: accessing the `static mut` is OK because we have acquired a critical
        // section.
        ENCODER.end_frame(do_write);

        Self::flush();
    }

    unsafe fn flush() {
        PrinterImpl::flush();
    }

    unsafe fn write(bytes: &[u8]) {
        // safety: accessing the `static mut` is OK because we have acquired a critical
        // section.
        ENCODER.write(bytes, do_write);
    }
}

fn do_write(bytes: &[u8]) {
    PrinterImpl::write_bytes_assume_cs(bytes)
}
