use crate::{cmd::CommandLine, print, println};
use embassy_stm32::{mode::Async, usart::UartRx};
use heapless::Vec;
use log::info;

#[embassy_executor::task]
pub async fn cmd_task(mut rx: UartRx<'static, Async>) {
    info!("hello CMD task!");
    let mut cmd = init_command_line();
    let mut buffer = [0x00; 1];
    let mut vec_buffer: Vec<u8, 50> = Vec::new();
    while let Ok(_) = rx.read(&mut buffer).await {
        // Ignore null bytes
        if buffer[0] == 0x00 {
            continue;
        }
        // Newline or carriage return signifies the end of a command
        if buffer[0] == b'\n' || buffer[0] == b'\r' {
            print!("\n");
            if !vec_buffer.is_empty() {
                let command_str = core::str::from_utf8(&vec_buffer).unwrap();
                if let Some(command) = cmd.check_command(command_str) {
                    cmd.execute(command, command_str);
                } else {
                    println!("Invalid command");
                    cmd.help_command();
                }
            }
            print!("\x1b[1;32mnuen-embassy >\x1b[0m ");
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
            let _ = vec_buffer.push(buffer[0]);
        }
    }
}

// init the command and callback function for it
fn init_command_line() -> CommandLine<'static> {
    let mut command_line = CommandLine::new();
    command_line.add_command("hello", "Say hello", |_| println!("xin chao ban"));
    command_line.add_command("bye", "Say goodbye", say_good_bye);
    command_line.add_command("reset", "reset the board", |_| {
        println!("reset the board immediately !!!");
        cortex_m::peripheral::SCB::sys_reset();
    });
    command_line
}

fn say_good_bye(args: &[&str]) {
    println!("tam biet ban!");
}
