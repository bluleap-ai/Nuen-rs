#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::{Executor, Spawner};
use embassy_stm32::bind_interrupts;
use embassy_stm32::can::{
    Can, CanRx, CanTx, Frame, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
    StandardId, TxInterruptHandler,
};
use embassy_stm32::peripherals::CAN1;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel};
use log::info;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

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

type ScreenBox = Channel<NoopRawMutex, ScreenRequest, 16>;
type SimulinkBox = Channel<NoopRawMutex, SimulinkType, 16>;

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::task]
async fn state_machine_task(channel0: &'static ScreenBox, channel1: &'static SimulinkBox) {
    match channel1.receive().await {
        SimulinkType::KeyFob(state) => {
            info!("Receive keyfob state {}", state);
        }
        SimulinkType::Can(frame) => {
            info!("Receive Can Frame {:?}", frame);
            channel0.send(ScreenRequest::LeftIndicator).await;
        }
    }
}

#[embassy_executor::task]
async fn can_tx_task(mut tx: CanTx<'static>, channel: &'static ScreenBox) {
    let mut i: u8 = 0;
    loop {
        match channel.receive().await {
            ScreenRequest::LeftIndicator => {
                info!("send LeftIndicator to screen");
                if let Some(can_id) = StandardId::new(i as _) {
                    if let Ok(tx_frame) = Frame::new_data(can_id, &[i]) {
                        let status = tx.write(&tx_frame).await;
                        info!(
                            "Transmit OK - dequeue_frame: {:?} - MB: {:?}",
                            status.dequeued_frame(),
                            status.mailbox()
                        );

                        i = i.wrapping_add(1);
                    } else {
                        error!("Failed to parse Can Frame");
                    }
                } else {
                    error!("Failed to parse Can ID");
                }
            }
            ScreenRequest::RightIndicator => {
                info!("send LeftIndicator to screen");
            }
            ScreenRequest::Speed(speed) => {
                info!("send Speed {} to screen", speed);
            }
            ScreenRequest::Soc(soc) => {
                info!("send SOC {} to screen", soc);
            }
            ScreenRequest::Abs(abs) => {
                info!("send ABS {} to screen", abs);
            }
            ScreenRequest::HeadLight(on) => {
                info!("send HeadLight {} to screen", on);
            }
        }
    }
}

#[embassy_executor::task]
async fn can_rx_task(mut rx: CanRx<'static>, channel: &'static SimulinkBox) {
    match rx.read().await {
        Ok(evelope) => {
            info!("Receive CAN Frame {:?}", evelope);
            channel.send(SimulinkType::Can(evelope.frame)).await;
        }
        Err(e) => {
            error!("Failed to receive CAN Frame: {}", e);
        }
    }
}

#[embassy_executor::task]
async fn keyfob_task(_channel: &'static SimulinkBox) {}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    // let mut executor1: Executor = Executor::new();
    let executor = EXECUTOR.init(Executor::new());
    static CHANNEL0: StaticCell<SimulinkBox> = StaticCell::new();
    let channel0 = &*CHANNEL0.init(Channel::new());
    static CHANNEL1: StaticCell<ScreenBox> = StaticCell::new();
    let channel1 = &*CHANNEL1.init(Channel::new());
    let mut can = Can::new(p.CAN1, p.PA11, p.PA12, Irqs);
    can.modify_config().set_bitrate(500_000);
    can.enable().await;
    let (can_tx, can_rx) = can.split();
    spawner
        .spawn(state_machine_task(channel1, channel0))
        .unwrap();
    executor.run(|spawn| {
        spawn.spawn(can_tx_task(can_tx, channel1)).unwrap();
        spawn.spawn(can_rx_task(can_rx, channel0)).unwrap();
        spawn.spawn(keyfob_task(channel0)).unwrap();
    });
}
