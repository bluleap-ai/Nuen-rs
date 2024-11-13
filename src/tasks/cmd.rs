use crate::{print, println};
use embassy_stm32::{mode::Async, usart::UartRx};
use heapless::Vec;

#[embassy_executor::task]
pub async fn cmd_task(mut rx: UartRx<'static, Async>) {
    println!("hello CMD task!");
    let mut buffer = [0x00; 1];
    let mut vec_buffer: Vec<char, 50> = Vec::new();
    while let Ok(_) = rx.read(&mut buffer).await {
        // Ignore null bytes
        if buffer[0] == 0x00 {
            continue;
        }
        // Newline or carriage return signifies the end of a command
        if buffer[0] == b'\n' || buffer[0] == b'\r' {
            print!("\n");
            if !vec_buffer.is_empty() {
                println!("Received command: {:?}", vec_buffer);
            }
            print!("\x1b[32mnuen-embassy >\x1b[0m ");
            vec_buffer.clear(); // Clear the command string for the next command
        } else if buffer[0] == 0x08 || buffer[0] == 127 {
            // process backspace character
            if let Some(_) = vec_buffer.pop() {
                // Move cursor back, print a space to clear the character, then move back again
                print!("\x08 \x08");
            }
        } else {
            print!("{}", buffer[0] as char);
            // Append valid bytes to command string
            let _ = vec_buffer.push(buffer[0] as char);
        }
    }
}
