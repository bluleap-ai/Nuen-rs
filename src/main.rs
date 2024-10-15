#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::{Executor, InterruptExecutor, Spawner};
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
mod io;
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
unsafe fn SPI1() {
    EXECUTOR_HIGH.on_interrupt()
}

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

    let sw_input = SwitchGearInput {
        kill_sw: Input::new(p.PA4, Pull::Up),
        mode_sw: Input::new(p.PA6, Pull::Up),
        side_stand_sw: Input::new(p.PA7, Pull::Up),
        reverse_sw: Input::new(p.PA8, Pull::Up),
        horn_sw: Input::new(p.PA9, Pull::Up),
        pha_cos_pw_sw: Input::new(p.PA10, Pull::Up),
        pha_cos_sw: Input::new(p.PB8, Pull::Up),
        left_braker_sw: Input::new(p.PB9, Pull::Up),
        right_braker_sw: Input::new(p.PB1, Pull::Up),
        keyfob_a_sw: Input::new(p.PB12, Pull::Up),
        keyfob_b_sw: Input::new(p.PB13, Pull::Up),
        keyfob_c_sw: Input::new(p.PB10, Pull::Up),
        keyfob_d_sw: Input::new(p.PC15, Pull::Up),
        turn_right_sw: Input::new(p.PA15, Pull::Up),
        turn_left_sw: Input::new(p.PB11, Pull::Up),
    };

    let bike_output = BikeOutput {
        seat_lock: Output::new(p.PA2, Level::High, Speed::Low),
        tank_lock: Output::new(p.PA3, Level::High, Speed::Low),
        sound_engine: Output::new(p.PA5, Level::High, Speed::Low),
        braker_lamp: Output::new(p.PB0, Level::High, Speed::Low),
        turn_right_lamp: Output::new(p.PB3, Level::High, Speed::Low),
        pha_lamp: Output::new(p.PB4, Level::High, Speed::Low),
        cos_lamp: Output::new(p.PB5, Level::High, Speed::Low),
        license_lamp: Output::new(p.PB14, Level::High, Speed::Low),
        horn: Output::new(p.PB15, Level::High, Speed::Low),
        tail_lamp: Output::new(p.PC13, Level::High, Speed::Low),
        turn_left_lamp: Output::new(p.PC14, Level::High, Speed::Low),
    };

    // High-priority executor: SPI5, priority level 6
    interrupt::SPI1.set_priority(Priority::P6);
    let high_prio_spawner = EXECUTOR_HIGH.start(interrupt::SPI1);

    // Init 2 embassy channels to transfer data.
    static CHANNEL0: StaticCell<SimulinkBox> = StaticCell::new();
    static CHANNEL1: StaticCell<ScreenBox> = StaticCell::new();
    let channel0 = &*CHANNEL0.init(Channel::new());
    let channel1 = &*CHANNEL1.init(Channel::new());

    // Initialize the CAN bus
    let can = Can::new(p.CAN1, p.PA11, p.PA12, Irqs);

    // spawn state machine task on main executor.
    high_prio_spawner
        .spawn(tasks::state_machine_task(
            sw_input,
            bike_output,
            channel1,
            channel0,
        ))
        .unwrap();
    info!("start executor");
    // spawn CANTx and CANRx task on lower priority executor.
    interrupt::SPI2.set_priority(Priority::P6);
    let low_prio_spawner = EXECUTOR_LOW.init(Executor::new());
    low_prio_spawner.run(|spawner| {
        spawner
            .spawn(init_executor(spawner, can, channel0, channel1))
            .unwrap();
    });
}
#[embassy_executor::task]
async fn init_executor(
    spawner: Spawner,
    mut can: Can<'static>,
    channel0: &'static SimulinkBox,
    channel1: &'static ScreenBox,
) {
    info!("hello executor");
    can.modify_config()
        .set_automatic_retransmit(true)
        .set_bitrate(250_000);
    can.enable().await;
    let (can_tx, can_rx) = can.split();
    spawner.spawn(tasks::can_tx_task(can_tx, channel1)).unwrap();
    spawner.spawn(tasks::can_rx_task(can_rx, channel0)).unwrap();
}
