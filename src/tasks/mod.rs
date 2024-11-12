mod can_rx;
mod can_tx;
mod simulink;

const CAN_RX_CYCLE: u64 = 50; // in ms
const CAN_TX_CYCLE: u64 = 50; // in ms
const SIM_APP_CYCLE: u64 = 50; // in ms

pub use can_rx::can_rx_task;
pub use can_tx::can_tx_task;
pub use simulink::state_machine_task;
