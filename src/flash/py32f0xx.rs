use std::marker::PhantomData;

use anyhow::Ok;
use bit_field::BitField;
use probe_rs::{Core, MemoryInterface};

use crate::flash::{Flash, Opt, OptBytes};

pub struct PY32F0xx<'c, 's, OPT: Opt>
where
    's: 'c,
{
    core: &'c mut Core<'s>,
    _opt: PhantomData<OPT>,
}

const UNLOCK_KEY0: u32 = 0x45670123;
const UNLOCK_KEY1: u32 = 0xCDEF89AB;

const OPT_UNLOCK_KEY0: u32 = 0x0819_2A3B;
const OPT_UNLOCK_KEY1: u32 = 0x4C5D_6E7F;

const FLASH_BASE: u64 = 0x4002_2000;
const CR_ADDR: u64 = FLASH_BASE + 0x14;
const SR_ADDR: u64 = FLASH_BASE + 0x10;
const OPTR_ADDR: u64 = FLASH_BASE + 0x20;
const SDKR_ADDR: u64 = FLASH_BASE + 0x24;
/// F002
const BCTR_ADDR: u64 = FLASH_BASE + 0x28;
const WRPR_ADDR: u64 = FLASH_BASE + 0x2c;
const KEYR_ADDR: u64 = FLASH_BASE + 0x08;
const OPT_KEYR_ADDR: u64 = FLASH_BASE + 0x0c;

const OPT_BYTES_TRIGGER_ADDR: u64 = 0x4002_2080;

pub struct F07x;
pub struct F030;
pub type F003 = F030;
pub struct F002;

impl Opt for F07x {
    const OPT_BASE: u64 = 0x1fff_3100;
    const OPT_SIZE: usize = 0x20;

    fn parse_opt_bytes(bytes: &[u8]) -> anyhow::Result<super::OptBytes> {
        anyhow::ensure!(bytes.len() == Self::OPT_SIZE);
        let (optr, right) = bytes.split_at(8);
        let (sdkr, right) = right.split_at(8);
        let (_, wrpr) = right.split_at(8);
        let optr = {
            let (optr, complement) = (&optr[0..2], &optr[2..4]);
            let pair = (
                u16::from_le_bytes(optr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        let sdkr = {
            let (sdkr, complement) = (&sdkr[0..2], &sdkr[2..4]);
            let pair = (
                u16::from_le_bytes(sdkr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        let wrpr = {
            let (wrpr, complement) = (&wrpr[0..2], &wrpr[2..4]);
            let pair = (
                u16::from_le_bytes(wrpr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        Ok(OptBytes {
            optr,
            sdkr,
            btcr: None,
            wrpr,
        })
    }
}

impl Opt for F030 {
    const OPT_BASE: u64 = 0x1fff_0e80;

    const OPT_SIZE: usize = 0x10;

    fn parse_opt_bytes(bytes: &[u8]) -> anyhow::Result<OptBytes> {
        anyhow::ensure!(bytes.len() == Self::OPT_SIZE);
        let (optr, right) = bytes.split_at(4);
        let (sdkr, right) = right.split_at(4);
        let (_, wrpr) = right.split_at(4);
        let optr = {
            let (optr, complement) = (&optr[0..2], &optr[2..4]);
            let pair = (
                u16::from_le_bytes(optr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        let sdkr = {
            let (sdkr, complement) = (&sdkr[0..2], &sdkr[2..4]);
            let pair = (
                u16::from_le_bytes(sdkr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        let wrpr = {
            let (wrpr, complement) = (&wrpr[0..2], &wrpr[2..4]);
            let pair = (
                u16::from_le_bytes(wrpr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        Ok(OptBytes {
            optr,
            sdkr,
            btcr: None,
            wrpr,
        })
    }
}

impl Opt for F002 {
    const OPT_BASE: u64 = 0x1fff_0080;

    const OPT_SIZE: usize = 0x10;

    fn parse_opt_bytes(bytes: &[u8]) -> anyhow::Result<OptBytes> {
        anyhow::ensure!(bytes.len() == Self::OPT_SIZE);
        let (optr, right) = bytes.split_at(4);
        let (sdkr, right) = right.split_at(4);
        let (btcr, wrpr) = right.split_at(4);
        let optr = {
            let (optr, complement) = (&optr[0..2], &optr[2..4]);
            let pair = (
                u16::from_le_bytes(optr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        let btcr = {
            let (btcr, complement) = (&btcr[0..2], &btcr[2..4]);
            let pair = (
                u16::from_le_bytes(btcr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        let sdkr = {
            let (sdkr, complement) = (&sdkr[0..2], &sdkr[2..4]);
            let pair = (
                u16::from_le_bytes(sdkr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        let wrpr = {
            let (wrpr, complement) = (&wrpr[0..2], &wrpr[2..4]);
            let pair = (
                u16::from_le_bytes(wrpr.try_into().unwrap()),
                u16::from_le_bytes(complement.try_into().unwrap()),
            );
            anyhow::ensure!(pair.0 == !pair.1);
            pair.0
        };
        Ok(OptBytes {
            optr,
            sdkr,
            btcr: Some(btcr),
            wrpr,
        })
    }
}

pub type PY32F07x<'c, 's> = PY32F0xx<'c, 's, F07x>;
pub type PY32F030<'c, 's> = PY32F0xx<'c, 's, F030>;
pub type PY32F003<'c, 's> = PY32F030<'c, 's>;
pub type PY32F002<'c, 's> = PY32F0xx<'c, 's, F002>;

impl<'c, 's, OPT: Opt> PY32F0xx<'c, 's, OPT> {
    pub fn new(core: &'c mut Core<'s>) -> Self {
        Self {
            core,
            _opt: PhantomData,
        }
    }
}

trait FlashRegs {
    fn cr(&mut self) -> anyhow::Result<u32>;
    fn set_cr(&mut self, cr: u32) -> anyhow::Result<()>;
    fn sr(&mut self) -> anyhow::Result<u32>;
    fn set_sr(&mut self, sr: u32) -> anyhow::Result<()>;
}

impl<'c, 's, OPT: Opt> FlashRegs for PY32F0xx<'c, 's, OPT>
where
    's: 'c,
{
    fn cr(&mut self) -> anyhow::Result<u32> {
        Ok(self.core.read_word_32(CR_ADDR)?)
    }

    fn set_cr(&mut self, cr: u32) -> anyhow::Result<()> {
        Ok(self.core.write_word_32(CR_ADDR, cr)?)
    }

    fn sr(&mut self) -> anyhow::Result<u32> {
        Ok(self.core.read_word_32(SR_ADDR)?)
    }

    fn set_sr(&mut self, sr: u32) -> anyhow::Result<()> {
        Ok(self.core.write_word_32(SR_ADDR, sr)?)
    }
}

impl<'c, 's, OPT: Opt> Flash for PY32F0xx<'c, 's, OPT>
where
    's: 'c,
{
    fn lock(&mut self) -> anyhow::Result<()> {
        if !self.is_locked()? {
            let mut cr = self.cr()?;
            cr.set_bit(31, true);
            self.set_cr(cr)?;
        }
        Ok(())
    }

    fn lock_ob(&mut self) -> anyhow::Result<()> {
        if !self.is_ob_locked()? {
            let mut cr = self.cr()?;
            cr.set_bit(30, true);
            self.set_cr(cr)?;
        }
        Ok(())
    }

    fn unlock(&mut self) -> anyhow::Result<()> {
        if self.is_locked()? {
            self.core.write_word_32(KEYR_ADDR, UNLOCK_KEY0)?;
            self.core.write_word_32(KEYR_ADDR, UNLOCK_KEY1)?;
        }
        while self.is_locked()? {}
        Ok(())
    }

    fn unlock_ob(&mut self) -> anyhow::Result<()> {
        if self.is_ob_locked()? {
            self.core.write_word_32(OPT_KEYR_ADDR, OPT_UNLOCK_KEY0)?;
            self.core.write_word_32(OPT_KEYR_ADDR, OPT_UNLOCK_KEY1)?;
        }
        while self.is_ob_locked()? {}
        Ok(())
    }

    fn is_locked(&mut self) -> anyhow::Result<bool> {
        Ok(self.cr()?.get_bit(31))
    }

    fn is_ob_locked(&mut self) -> anyhow::Result<bool> {
        Ok(self.cr()?.get_bit(30))
    }

    fn get_opt_bytes(&mut self) -> anyhow::Result<super::OptBytes> {
        let mut vec = vec![0u8; OPT::OPT_SIZE];
        self.core.read(OPT::OPT_BASE, &mut vec)?;
        Ok(OPT::parse_opt_bytes(&vec)?)
    }

    fn set_opt_bytes(&mut self, bytes: super::OptBytes) -> anyhow::Result<()> {
        let old_bytes = self.get_opt_bytes()?;
        if old_bytes == bytes {
            return Ok(());
        }
        let sr = self.sr()? & ((0b1 << 4) | (0b1 << 15));
        self.set_sr(sr)?;

        macro_rules! wait_ready {
            () => {
                while self.sr()?.get_bit(16) {}
            };
        }
        self.unlock()?;
        self.unlock_ob()?;

        wait_ready!();

        if old_bytes.optr != bytes.optr {
            self.core.write_word_32(OPTR_ADDR, bytes.optr as u32)?;
        }
        if old_bytes.sdkr != bytes.sdkr {
            self.core.write_word_32(SDKR_ADDR, bytes.sdkr as u32)?;
        }
        if let (Some(old_bctr), Some(new_bctr)) = (old_bytes.btcr, bytes.btcr) {
            if old_bctr != new_bctr {
                self.core.write_word_32(BCTR_ADDR, new_bctr as u32)?;
            }
        }

        if old_bytes.wrpr != bytes.wrpr {
            self.core.write_word_32(WRPR_ADDR, bytes.wrpr as u32)?;
        }

        let mut cr = self.cr()?;
        cr.set_bit(17, true);
        self.set_cr(cr)?;

        self.core.write_word_32(OPT_BYTES_TRIGGER_ADDR, 0xFF)?;

        wait_ready!();

        while self.get_opt_bytes()? != bytes {}

        // sleep(Duration::from_secs(1));

        let mut sr = self.sr()?;
        sr.set_bit(0, true);
        self.set_cr(sr)?;

        self.lock_ob()?;
        self.lock()?;

        let mut cr = self.cr()?;
        cr.set_bit(27, true);
        self.set_cr(cr)?;
        Ok(())
    }

    fn parse_opt_bytes(&self, bytes: &[u8]) -> anyhow::Result<OptBytes> {
        OPT::parse_opt_bytes(bytes)
    }
}
