use crate::{
    display::{CanMessage, SegLcd},
    ScreenBox, ScreenRequest,
};
use defmt::*;
use embassy_stm32::can::{filter::Mask32, Can, CanTx, ExtendedId, Fifo, Frame, StandardId};

impl From<CanMessage> for Frame {
    fn from(message: CanMessage) -> Self {
        Frame::new_extended(message.id, &message.data).unwrap()
    }
}

#[embassy_executor::task]
pub async fn can_tx_task(
    mut can: Can<'static>,
    mut tx: CanTx<'static>,
    channel: &'static ScreenBox,
) {
    let mut display = SegLcd::init();
    display.lcd_on();
    can.enable().await;
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    info!("hello can tx!");
    loop {
        match channel.receive().await {
            ScreenRequest::LeftIndicator => {
                info!("send LeftIndicator to screen");
                if tx.is_idle() {
                    tx.write(&display.left_ind_on().into()).await;
                } else {
                    warn!("The CAN bus is not idle");
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
