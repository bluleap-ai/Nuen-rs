mod bms_handler;
mod can_rx;
mod can_tx;
mod cmd;
mod motor_handler;
mod obc_handler;
mod simulink;

const CAN_RX_CYCLE: u64 = 50; // in ms
const CAN_TX_CYCLE: u64 = 50; // in ms
const SIM_APP_CYCLE: u64 = 50; // in ms
const BMS_CYCLE: u64 = 50; // in ms
const MOTOR_CYCLE: u64 = 50; // in ms
const OBC_CYCLE: u64 = 50; // in ms

pub use bms_handler::bms_task;
pub use can_rx::can_rx_task;
pub use can_tx::can_tx_task;
pub use cmd::cmd_task;
pub use motor_handler::motor_task;
pub use obc_handler::obc_task;
pub use simulink::state_machine_task;
