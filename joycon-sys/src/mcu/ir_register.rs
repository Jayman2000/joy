#[repr(packed)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Register {
    page: u8,
    offset: u8,
    value: u8,
}

impl Register {
    fn new(address: Address, value: u8) -> Register {
        Register {
            page: address.address().0,
            offset: address.address().1,
            value,
        }
    }
    pub fn resolution(resolution: Resolution) -> Register {
        Register::new(Resolution, resolution as u8)
    }

    pub fn exposure(exposure: u16) -> [Register; 2] {
        [
            Register::new(ExposureLSB, (exposure & 0xff) as u8),
            Register::new(ExposureMSB, (exposure >> 8) as u8),
        ]
    }

    pub fn exposure_mode(mode: ExposureMode) -> Register {
        Register::new(ExposureMode, mode as u8)
    }

    pub fn digital_gain(gain: u16) -> [Register; 2] {
        [
            Register::new(DigitalGainLSB, ((gain & 0x0f) << 4) as u8),
            Register::new(DigitalGainMSB, ((gain & 0xf0) >> 4) as u8),
        ]
    }

    pub fn ir_leds(far: bool, near: bool) -> Register {
        //todo: strobe + flashlight
        //todo: bitmap
        Register::new(IRLeds, ((!far) as u8) << 5 | ((!near) as u8) << 6)
    }

    pub fn external_light_filter(filter: ExternalLightFilter) -> Register {
        Register::new(ExternalLightFilter, filter as u8)
    }

    pub fn white_pixel_threshold(threshold: u8) -> Register {
        Register::new(WhitePixelThreshold, threshold)
    }

    pub fn leds_intensity(l1: u8, l2: u8, l3: u8, l4: u8) -> [Register; 2] {
        assert_eq!(0, (l1 | l2 | l3 | l4) & 0xf0);
        [
            Register::new(IntensityLeds12, l1 << 4 | l2),
            Register::new(IntensityLeds34, l3 << 4 | l4),
        ]
    }

    pub fn flip(side: Flip) -> Register {
        Register::new(Flip, side as u8)
    }

    pub fn denoise(enabled: bool) -> Register {
        Register::new(Denoise, enabled as u8)
    }

    pub fn edge_smoothing_threshold(threshold: u8) -> Register {
        Register::new(EdgeSmoothingThreshold, threshold)
    }

    pub fn color_interpolation_threshold(threshold: u8) -> Register {
        Register::new(ColorInterpolationThreshold, threshold)
    }

    pub fn buffer_update_time(time: u8) -> Register {
        Register::new(BufferUpdateTimeLSB, time)
    }

    pub fn finish() -> Register {
        Register::new(Finish, 1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Address {
    Resolution,
    DigitalGainLSB,
    DigitalGainMSB,
    ExposureLSB,
    ExposureMSB,
    ExposureMode,
    ExternalLightFilter,
    WhitePixelThreshold,
    IntensityLeds12,
    IntensityLeds34,
    Flip,
    Denoise,
    EdgeSmoothingThreshold,
    ColorInterpolationThreshold,
    BufferUpdateTimeLSB,
    IRLeds,
    Finish,
}
use Address::*;

impl Address {
    /// page + offset
    fn address(self) -> (u8, u8) {
        match self {
            BufferUpdateTimeLSB => (0, 0x04),
            Finish => (0, 0x07),
            ExternalLightFilter => (0, 0x0e),
            IRLeds => (0, 0x10),
            IntensityLeds12 => (0, 0x11),
            IntensityLeds34 => (0, 0x12),
            Flip => (0, 0x2d),
            Resolution => (0, 0x2e),
            DigitalGainLSB => (1, 0x2e),
            DigitalGainMSB => (1, 0x2f),
            ExposureLSB => (1, 0x30),
            ExposureMSB => (1, 0x31),
            ExposureMode => (1, 0x32),
            WhitePixelThreshold => (1, 0x43),
            Denoise => (1, 0x67),
            EdgeSmoothingThreshold => (1, 0x68),
            ColorInterpolationThreshold => (1, 0x69),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Resolution {
    /// Full pixel array
    R320x240 = 0b0000_0000,
    /// Sensor Binning [2 X 2]
    R160x120 = 0b0101_0000,
    /// Sensor Binning [4 x 2] and Skipping [1 x 2]
    R80x60 = 0b0110_0100,
    /// Sensor Binning [4 x 2] and Skipping [2 x 4]
    R40x30 = 0b0110_1001,
}

impl Resolution {
    pub fn max_fragment_id(self) -> u8 {
        match self {
            Resolution::R320x240 => 0xff,
            Resolution::R160x120 => 0x0f,
            Resolution::R80x60 => 0x0f,
            Resolution::R40x30 => 0x03,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ExposureMode {
    Manual = 0,
    Max = 1,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ExternalLightFilter {
    Off = 0b00,
    X1 = 0b11,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Flip {
    Normal = 0,
    Vertically = 1,
    Horizontally = 2,
    Both = 3,
}
