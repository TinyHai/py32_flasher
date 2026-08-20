use std::fmt::Display;

use anyhow::anyhow;
use bit_field::BitField;

pub mod py32f0xx;

#[derive(PartialEq, Clone, Copy)]
pub struct OptBytes {
    optr: u16,
    sdkr: u16,
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

impl TryFrom<&[u8]> for OptBytes {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(anyhow!("Content length mismatch"));
        }
        let (optr, right) = value.split_at(4);
        let (sdkr, right) = right.split_at(4);
        let (_, wrpr) = right.split_at(4);
        let optr = {
            let (optr, complement) = optr.split_at(2);
            let pair = (
                u16::from_le_bytes(optr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            if pair.0 != !pair.1 {
                return Err(anyhow!("optr does not match its complement"));
            }
            pair.0
        };
        let sdkr = {
            let (sdkr, complement) = sdkr.split_at(2);
            let pair = (
                u16::from_le_bytes(sdkr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            if pair.0 != !pair.1 {
                return Err(anyhow!("sdkr does not match its complement"));
            }
            pair.0
        };
        let wrpr = {
            let (wrpr, complement) = wrpr.split_at(2);
            let pair = (
                u16::from_le_bytes(wrpr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            if pair.0 != !pair.1 {
                return Err(anyhow!("wrpr does not match its complement"));
            }
            pair.0
        };
        Ok(OptBytes { optr, sdkr, wrpr })
    }
}

impl Display for OptBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "OptBytes {{ optr: {:02X?}, sdkr: {:02X?}, wrpr: {:02X?} }}",
            self.optr.to_le_bytes(),
            self.sdkr.to_le_bytes(),
            self.wrpr.to_le_bytes()
        ))
    }
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
}
