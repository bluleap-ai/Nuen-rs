use crate::{
    display::{CanMessage, SegLcd},
    println, ScreenBox, ScreenRequest,
};
use embassy_stm32::can::{filter::Mask32, Can, CanTx, Fifo, Frame};

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
    println!("hello can tx!");
    loop {
        match channel.receive().await {
            ScreenRequest::Power(en) => {
                println!("send LeftIndicator to screen");
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
                println!("send LeftIndicator to screen");
                tx.write(&display.left_ind_on().into()).await;
            }
            ScreenRequest::RightIndicator => {
                println!("send LeftIndicator to screen");
                tx.write(&display.right_ind_on().into()).await;
            }
            ScreenRequest::Speed(speed) => {
                println!("send Speed {} to screen", speed);
            }
            ScreenRequest::Soc(soc) => {
                println!("send SOC {} to screen", soc);
            }
            ScreenRequest::Abs(abs) => {
                println!("send ABS {} to screen", abs);
            }
            ScreenRequest::HeadLight(on) => {
                println!("send HeadLight {} to screen", on);
            }
        }
    }
}
