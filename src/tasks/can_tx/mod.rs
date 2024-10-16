use crate::{ScreenBox, ScreenRequest};
use defmt::*;
use embassy_stm32::can::{filter::Mask32, Can, CanTx, Fifo, Frame, StandardId};

#[embassy_executor::task]
pub async fn can_tx_task(
    mut can: Can<'static>,
    mut tx: CanTx<'static>,
    channel: &'static ScreenBox,
) {
    can.enable().await;
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    let mut i: u8 = 0;
    info!("hello can tx!");
    loop {
        match channel.receive().await {
            ScreenRequest::LeftIndicator => {
                info!("send LeftIndicator to screen");
                if tx.is_idle() {
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
