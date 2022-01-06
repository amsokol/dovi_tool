use anyhow::{ensure, Result};
use bitvec_helpers::{bitvec_reader::BitVecReader, bitvec_writer::BitVecWriter};

#[cfg(feature = "serde_feature")]
use serde::{Deserialize, Serialize};

use super::{ExtMetadataBlock, ExtMetadataBlockInfo, MAX_12_BIT_VALUE};

/// Creative intent trim passes per target display peak brightness
/// For CM v4.0, L8 metadata only is present and used to compute L2
#[repr(C)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_feature", derive(Deserialize, Serialize))]
pub struct ExtMetadataBlockLevel8 {
    pub length: u64,
    pub target_display_index: u8,
    pub trim_slope: u16,
    pub trim_offset: u16,
    pub trim_power: u16,
    pub trim_chroma_weight: u16,
    pub trim_saturation_gain: u16,
    pub ms_weight: u16,
    pub target_mid_contrast: u16,
    pub clip_trim: u16,
}

impl ExtMetadataBlockLevel8 {
    pub fn parse(reader: &mut BitVecReader, length: u64) -> ExtMetadataBlock {
        let mut level8 = Self {
            length,
            target_display_index: reader.get_n(8),
            trim_slope: reader.get_n(12),
            trim_offset: reader.get_n(12),
            trim_power: reader.get_n(12),
            trim_chroma_weight: reader.get_n(12),
            trim_saturation_gain: reader.get_n(12),
            ms_weight: reader.get_n(12),
            ..Default::default()
        };

        if length >= 11 {
            level8.target_mid_contrast = reader.get_n(12);

            if length >= 13 {
                level8.clip_trim = reader.get_n(12);
            }
        }

        ExtMetadataBlock::Level8(level8)
    }

    pub fn write(&self, writer: &mut BitVecWriter) -> Result<()> {
        self.validate()?;

        writer.write_n(&self.target_display_index.to_be_bytes(), 8);
        writer.write_n(&self.trim_slope.to_be_bytes(), 12);
        writer.write_n(&self.trim_offset.to_be_bytes(), 12);
        writer.write_n(&self.trim_power.to_be_bytes(), 12);
        writer.write_n(&self.trim_chroma_weight.to_be_bytes(), 12);
        writer.write_n(&self.trim_saturation_gain.to_be_bytes(), 12);
        writer.write_n(&self.ms_weight.to_be_bytes(), 12);

        if self.length >= 11 {
            writer.write_n(&self.target_mid_contrast.to_be_bytes(), 12);

            if self.length >= 13 {
                writer.write_n(&self.clip_trim.to_be_bytes(), 12);
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        ensure!(self.trim_slope <= MAX_12_BIT_VALUE);
        ensure!(self.trim_offset <= MAX_12_BIT_VALUE);
        ensure!(self.trim_power <= MAX_12_BIT_VALUE);
        ensure!(self.trim_chroma_weight <= MAX_12_BIT_VALUE);
        ensure!(self.trim_saturation_gain <= MAX_12_BIT_VALUE);
        ensure!(self.ms_weight <= MAX_12_BIT_VALUE);
        ensure!(self.target_mid_contrast <= MAX_12_BIT_VALUE);
        ensure!(self.clip_trim <= MAX_12_BIT_VALUE);

        Ok(())
    }
}

impl ExtMetadataBlockInfo for ExtMetadataBlockLevel8 {
    fn level(&self) -> u8 {
        8
    }

    fn bytes_size(&self) -> u64 {
        self.length
    }

    fn required_bits(&self) -> u64 {
        if self.length == 13 {
            104
        } else if self.length == 12 {
            92
        } else {
            80
        }
    }

    fn sort_key(&self) -> (u8, u16) {
        (self.level(), self.target_display_index as u16)
    }
}

/// Target display: 1000-nit, P3, D65, ST.2084, Full (HOME)
impl Default for ExtMetadataBlockLevel8 {
    fn default() -> Self {
        Self {
            length: 10,
            target_display_index: 48,
            trim_slope: 2048,
            trim_offset: 2048,
            trim_power: 2048,
            trim_chroma_weight: 2048,
            trim_saturation_gain: 2048,
            ms_weight: 2048,
            target_mid_contrast: 0,
            clip_trim: 0,
        }
    }
}
