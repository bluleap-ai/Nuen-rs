use crate::{
    display::{CanMessage, SegLcd},
    tasks::CAN_TX_CYCLE,
    ScreenBox, ScreenRequest,
};
use embassy_stm32::can::{filter::Mask32, Can, CanTx, Fifo, Frame};
use embassy_time::{Instant, Timer};
use log::{info, warn};

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

    can.enable().await;
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    tx.write(&display.get_status_1().into()).await;
    tx.write(&display.get_status_2().into()).await;
    tx.write(&display.get_status_3().into()).await;
    info!("Started CANTX Task !!!");
    loop {
        let start = Instant::now();
        match channel.receive().await {
            ScreenRequest::Power(en) => {
                info!("send LeftIndicator to screen");
                if en {
                    tx.write(&display.lcd_on().into()).await;
                } else {
                    tx.write(&display.lcd_off().into()).await;
                }
            }
            ScreenRequest::Ready => {
                tx.write(&display.rdy_on().into()).await;
            }
            ScreenRequest::LeftIndicator => {
                info!("send LeftIndicator to screen");
                tx.write(&display.left_ind_on().into()).await;
            }
            ScreenRequest::RightIndicator => {
                info!("send LeftIndicator to screen");
                tx.write(&display.right_ind_on().into()).await;
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

        let ms = Instant::now().duration_since(start).as_millis();
        if ms > CAN_TX_CYCLE {
            warn!("CanTx task done after {ms}ms > {CAN_TX_CYCLE}ms");
        } else {
            Timer::after_millis(CAN_TX_CYCLE - ms).await;
        }
    }
}
