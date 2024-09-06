//! Data collection for network usage/IO.

pub mod sysinfo;
pub use self::sysinfo::*;

#[derive(Default, Clone, Debug)]
/// All units in bits.
pub struct NetworkHarvest {
    pub rx: u64,
    pub tx: u64,
    pub total_rx: u64,
    pub total_tx: u64,
    pub rx_pkts: u64,
    pub tx_pkts: u64,
    pub total_rx_pkts: u64,
    pub total_tx_pkts: u64,

}

impl NetworkHarvest {
    pub fn first_run_cleanup(&mut self) {
        self.rx = 0;
        self.tx = 0;
        self.rx_pkts = 0;
        self.tx_pkts = 0;
    }
}
