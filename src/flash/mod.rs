use std::fmt::Display;

use bit_field::BitField;

pub mod py32f0xx;

#[derive(PartialEq, Clone, Copy)]
pub struct OptBytes {
    optr: u16,
    sdkr: u16,
    /// f002
    btcr: Option<u16>,
    wrpr: u16,
}

impl OptBytes {
    pub fn enable_rdp(&mut self) {
        self.optr.set_bits(0..8, 0x55);
    }

    pub fn disable_rdp(&mut self) {
        self.optr.set_bits(0..8, 0xAA);
    }

    pub fn is_rdp_enable(&self) -> bool {
        self.optr.get_bits(0..8) == 0x55
    }
}

impl Display for OptBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "OptBytes {{ optr: {:02X?}, sdkr: {:02X?}, btcr: {:02X?} wrpr: {:02X?} }}",
            self.optr.to_le_bytes(),
            self.sdkr.to_le_bytes(),
            self.btcr.map(|btcr| btcr.to_le_bytes()),
            self.wrpr.to_le_bytes()
        ))
    }
}

pub trait Opt {
    const OPT_BASE: u64;
    const OPT_SIZE: usize;

    fn parse_opt_bytes(bytes: &[u8]) -> anyhow::Result<OptBytes>;
}

pub trait Flash {
    fn lock(&mut self) -> anyhow::Result<()>;

    fn lock_ob(&mut self) -> anyhow::Result<()>;

    fn unlock(&mut self) -> anyhow::Result<()>;

    fn unlock_ob(&mut self) -> anyhow::Result<()>;

    fn is_locked(&mut self) -> anyhow::Result<bool>;

    fn is_ob_locked(&mut self) -> anyhow::Result<bool>;

    fn get_opt_bytes(&mut self) -> anyhow::Result<OptBytes>;

    fn set_opt_bytes(&mut self, bytes: OptBytes) -> anyhow::Result<()>;

    fn parse_opt_bytes(&self, bytes: &[u8]) -> anyhow::Result<OptBytes>;
}
