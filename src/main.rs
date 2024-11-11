#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::{Executor, InterruptExecutor};
use embassy_stm32::{
    bind_interrupts,
    can::{
        Can, Frame, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
        TxInterruptHandler,
    },
    gpio::{Input, Level, Output, Pull, Speed},
    interrupt,
    interrupt::{InterruptExt, Priority},
    peripherals::CAN1,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};
mod display;
mod io;
mod state_machine;
mod tasks;
use io::{BikeOutput, SwitchGearInput};

pub enum SimulinkType {
    KeyFob(u8),
    Can(Frame),
}

pub enum ScreenRequest {
    LeftIndicator,
    RightIndicator,
    Speed(u8),
    Soc(u8),
    Abs(bool),
    HeadLight(bool),
}

type ScreenBox = Channel<CriticalSectionRawMutex, ScreenRequest, 16>;
type SimulinkBox = Channel<CriticalSectionRawMutex, SimulinkType, 16>;

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_LOW: StaticCell<Executor> = StaticCell::new();
#[interrupt]
unsafe fn USART3() {
    EXECUTOR_HIGH.on_interrupt()
}

#[interrupt]
unsafe fn USART2() {
    EXECUTOR_HIGH.on_interrupt()
}

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let sw_input = SwitchGearInput {
        kill_sw: Input::new(p.PA4, Pull::Up), // no kill sw
        mode_sw: Input::new(p.PA6, Pull::Up),
        side_stand_sw: Input::new(p.PA1, Pull::Up),
        reverse_sw: Input::new(p.PA7, Pull::Up),
        horn_sw: Input::new(p.PB0, Pull::Up),
        pha_cos_pw_sw: Input::new(p.PC4, Pull::Up),
        pha_cos_sw: Input::new(p.PC5, Pull::Up),
        left_braker_sw: Input::new(p.PA2, Pull::Up),
        right_braker_sw: Input::new(p.PA3, Pull::Up),
        keyfob_a_sw: Input::new(p.PC0, Pull::Up),
        keyfob_b_sw: Input::new(p.PC1, Pull::Up),
        keyfob_c_sw: Input::new(p.PC2, Pull::Up),
        keyfob_d_sw: Input::new(p.PC3, Pull::Up),
        turn_right_sw: Input::new(p.PA0, Pull::Up),
        turn_left_sw: Input::new(p.PB1, Pull::Up),
    };

    let bike_output = BikeOutput {
        seat_lock: Output::new(p.PD2, Level::High, Speed::Low),
        tank_lock: Output::new(p.PC12, Level::High, Speed::Low),
        sound_engine: Output::new(p.PB14, Level::High, Speed::Low),
        braker_lamp: Output::new(p.PB8, Level::High, Speed::Low),
        turn_right_lamp: Output::new(p.PC8, Level::High, Speed::Low),
        pha_lamp: Output::new(p.PB13, Level::High, Speed::Low),
        cos_lamp: Output::new(p.PC6, Level::High, Speed::Low),
        license_lamp: Output::new(p.PC7, Level::High, Speed::Low),
        horn: Output::new(p.PA8, Level::High, Speed::Low),
        tail_lamp: Output::new(p.PC9, Level::High, Speed::Low),
        turn_left_lamp: Output::new(p.PB15, Level::High, Speed::Low),
    };

    // High-priority executor: USART3, priority level 6
    interrupt::USART3.set_priority(Priority::P6);
    let high_prio_spawner = EXECUTOR_HIGH.start(interrupt::USART3);

    // Init 2 embassy channels to transfer data.
    static CHANNEL0: StaticCell<SimulinkBox> = StaticCell::new();
    static CHANNEL1: StaticCell<ScreenBox> = StaticCell::new();
    let channel0 = &*CHANNEL0.init(Channel::new());
    let channel1 = &*CHANNEL1.init(Channel::new());

    // Initialize the CAN bus
    let mut can = Can::new(p.CAN1, p.PA11, p.PA12, Irqs);
    can.modify_config().set_bitrate(500_000);
    let (can_tx, can_rx) = can.split();

    // spawn state machine task on high priority executor.
    info!("Start State Machine task");
    high_prio_spawner
        .spawn(tasks::state_machine_task(
            sw_input,
            bike_output,
            channel1,
            channel0,
        ))
        .unwrap();

    // spawn CANTx and CANRx tasks on low priority executor.
    info!("Start CANTx and CANRx task");
    let low_prio_spawner = EXECUTOR_LOW.init(Executor::new());
    low_prio_spawner.run(|spawner| {
        spawner
            .spawn(tasks::can_tx_task(can, can_tx, channel1))
            .unwrap();
        spawner.spawn(tasks::can_rx_task(can_rx, channel0)).unwrap();
    });
}
