# NUEN Embassy OS

**NUEN Embassy OS** is a high-performance embedded operating system built using the [Embassy](https://github.com/embassy-rs/embassy) async framework for Rust. It is designed to enable real-time, asynchronous task scheduling and communication in resource-constrained devices, offering a modern alternative for embedded development with Rust.

## Features

- **Real-time Async Execution**: Powered by Embassy's async capabilities, NUEN Embassy OS manages tasks and I/O efficiently in embedded systems.
- **Multitasking Support**: The system supports multiple executors with various priority levels for different tasks.
- **Interrupt-Driven Architecture**: Integrates with hardware interrupts, allowing tasks to run based on external events, making the system highly responsive.
- **Embassy Time Driver**: Implements custom time drivers for handling tasks such as alarms and timers in embedded systems.
- **Embassy Channels**: Enables safe communication between tasks using embassy's async `Channel` API.

## Architect

![image](https://github.com/user-attachments/assets/caf8195a-60a3-43ed-9f54-aa3678a25b61)


## Getting Started

To get started with NUEN Embassy OS, you'll need to have Rust installed along with Embassy.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Embassy async framework (`embassy-rs`)
- [Probe-rs](https://github.com/probe-rs/probe-rs)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/virust-ai/Nuen-rs.git
   cd Nuen-rs
  
2. Set up your target toolchain:

   ```bash
   rustup target add thumbv7em-none-eabihf

3. Set up your target toolchain:

   ```bash
   cargo build --release
   cargo run --release

