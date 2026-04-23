//! Platform-agnostic Rust driver for the Texas Instruments INA4230 quad-channel
//! power and energy sense monitor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//!
//! For further details of the device architecture and operation, please refer
//! to the official [`Datasheet`].
//!
//! [`Datasheet`]: https://www.ti.com/lit/ds/symlink/ina4230.pdf

#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![allow(async_fn_in_trait)]

use embedded_sensors_hal_async::sensor;

#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(unsafe_code)]
#[allow(missing_docs)]
mod device;

pub use crate::device::*;

// ── I²C address ───────────────────────────────────────────────────────────────

/// Default 7-bit I²C address for the INA4230 (A0=GND, A1=GND → 0x40).
pub const INA4230_ADDR: u8 = 0x40;

/// Maximum register data size in bytes (energy registers are 32-bit = 4 bytes).
const LARGEST_REG_SIZE_BYTES: usize = 4;

// ── Error type ────────────────────────────────────────────────────────────────

/// INA4230 driver error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Ina4230Error<I2cError> {
    /// An error occurred on the I²C bus.
    Bus(I2cError),
}

impl<E: embedded_hal_async::i2c::Error> sensor::Error for Ina4230Error<E> {
    fn kind(&self) -> sensor::ErrorKind {
        match self {
            Self::Bus(_) => sensor::ErrorKind::Peripheral,
        }
    }
}

// ── DeviceInterface ───────────────────────────────────────────────────────────

/// Async I²C interface adapter for the INA4230.
pub struct DeviceInterface<I2c: embedded_hal_async::i2c::I2c> {
    /// The underlying async I²C bus.
    pub i2c: I2c,
    /// 7-bit I²C address of this device instance (see [`INA4230_ADDR`]).
    pub address: u8,
}

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface
    for DeviceInterface<I2c>
{
    type Error = Ina4230Error<I2c::Error>;
    type AddressType = u8;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        debug_assert!(data.len() <= LARGEST_REG_SIZE_BYTES, "Register data too large");
        let mut buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES];
        buf[0] = address;
        buf[1..=data.len()].copy_from_slice(data);
        self.i2c
            .write(self.address, &buf[..=data.len()])
            .await
            .map_err(Ina4230Error::Bus)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(self.address, &[address], data)
            .await
            .map_err(Ina4230Error::Bus)
    }
}

// ── Channel ───────────────────────────────────────────────────────────────────

/// One of the four measurement channels on the INA4230.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Channel {
    /// Channel 1
    Ch1,
    /// Channel 2
    Ch2,
    /// Channel 3
    Ch3,
    /// Channel 4
    Ch4,
}

/// ADC full-scale input range for shunt voltage measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AdcRange {
    /// ±81.92 mV full scale, LSB = 2.5 µV (default)
    Range0,
    /// ±20.48 mV full scale, LSB = 625 nV. SHUNT_CAL divided by 4.
    Range1,
}

// ── Physical-unit type aliases ────────────────────────────────────────────────

/// Voltage in millivolts.
pub type MilliVolts = f32;
/// Current in milliamperes.
pub type MilliAmps = f32;
/// Power in milliwatts.
pub type MilliWatts = f32;
/// Energy in millijoules.
pub type MilliJoules = f32;

// ── Sensor traits ─────────────────────────────────────────────────────────────

/// Async voltage sensor — reads bus or shunt voltage per channel.
pub trait VoltageSensor: sensor::ErrorType {
    /// Read the bus voltage for the given channel, in millivolts (LSB = 1.6 mV).
    async fn bus_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error>;
    /// Read the shunt voltage for the given channel, in millivolts (LSB = 2.5 µV).
    async fn shunt_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error>;
}

impl<T: VoltageSensor + ?Sized> VoltageSensor for &mut T {
    async fn bus_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error> {
        T::bus_voltage(self, channel).await
    }
    async fn shunt_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error> {
        T::shunt_voltage(self, channel).await
    }
}

/// Async current sensor — reads calculated current per channel.
pub trait CurrentSensor: sensor::ErrorType {
    /// Read the calculated current for the given channel, in milliamperes.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn current(&mut self, channel: Channel) -> Result<MilliAmps, Self::Error>;
}

impl<T: CurrentSensor + ?Sized> CurrentSensor for &mut T {
    async fn current(&mut self, channel: Channel) -> Result<MilliAmps, Self::Error> {
        T::current(self, channel).await
    }
}

/// Async power sensor — reads calculated power per channel.
pub trait PowerSensor: sensor::ErrorType {
    /// Read the calculated power for the given channel, in milliwatts.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn power(&mut self, channel: Channel) -> Result<MilliWatts, Self::Error>;
}

impl<T: PowerSensor + ?Sized> PowerSensor for &mut T {
    async fn power(&mut self, channel: Channel) -> Result<MilliWatts, Self::Error> {
        T::power(self, channel).await
    }
}

/// Async energy sensor — reads accumulated energy per channel.
pub trait EnergySensor: sensor::ErrorType {
    /// Read the accumulated energy for the given channel, in millijoules.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn energy(&mut self, channel: Channel) -> Result<MilliJoules, Self::Error>;
}

impl<T: EnergySensor + ?Sized> EnergySensor for &mut T {
    async fn energy(&mut self, channel: Channel) -> Result<MilliJoules, Self::Error> {
        T::energy(self, channel).await
    }
}

// ── Ina4230 driver struct ─────────────────────────────────────────────────────

/// High-level driver for the INA4230 quad-channel power and energy monitor.
pub struct Ina4230<I2c: embedded_hal_async::i2c::I2c> {
    /// The generated low-level register accessor.
    pub device: Device<DeviceInterface<I2c>>,
    /// Stored CURRENT_LSB in A/LSB, set by `calibrate*`. Used for unit conversion.
    current_lsb_a: f32,
    /// ADC input range, set during calibration. Used for shunt voltage LSB selection.
    adc_range: AdcRange,
}

impl<I2c: embedded_hal_async::i2c::I2c> Ina4230<I2c> {
    /// Create a new driver instance.
    ///
    /// Pass [`INA4230_ADDR`] for the default address (A0=GND, A1=GND → 0x40).
    pub fn new(i2c: I2c, address: u8) -> Self {
        Self {
            device: Device::new(DeviceInterface { i2c, address }),
            current_lsb_a: 0.0,
            adc_range: AdcRange::Range0,
        }
    }

    /// Release the underlying I²C bus.
    pub fn release(self) -> I2c {
        self.device.interface.i2c
    }

    // ── Device management ─────────────────────────────────────────────────

    /// Issue a full device reset (`CONFIG2.RST = 1`). All registers return to
    /// power-on defaults. The bit self-clears.
    pub async fn reset(&mut self) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device.config_2().write_async(|w| w.set_rst(true)).await
    }

    /// Read the manufacturer ID register. Returns `0x5449` ("TI") on a healthy device.
    pub async fn manufacturer_id(&mut self) -> Result<u16, Ina4230Error<I2c::Error>> {
        Ok(self.device.manufacturer_id().read_async().await?.id())
    }

    /// Poll the Conversion Ready Flag (`FLAGS.CVRF`). Returns `true` when all
    /// enabled channels have completed conversion and averaging. Reading FLAGS clears CVRF.
    pub async fn conversion_ready(&mut self) -> Result<bool, Ina4230Error<I2c::Error>> {
        Ok(self.device.flags().read_async().await?.cvrf())
    }

    /// Read the full flags register in one call.
    pub async fn flags(&mut self) -> Result<field_sets::Flags, Ina4230Error<I2c::Error>> {
        self.device.flags().read_async().await
    }

    // ── Calibration ───────────────────────────────────────────────────────

    /// Write the calibration register for a single channel.
    ///
    /// - `current_lsb_a`: desired current resolution in A/LSB (e.g. `100e-6` for 100 µA/LSB)
    /// - `shunt_ohms`: shunt resistor value in Ω (e.g. `0.010` for 10 mΩ)
    /// - `adc_range`: ADC full-scale input range (use [`AdcRange::Range0`] for default ±81.92 mV)
    ///
    /// Formula (ADCRANGE = 0): `SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)`
    /// Formula (ADCRANGE = 1): `SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT) / 4`
    pub async fn calibrate(
        &mut self,
        channel: Channel,
        current_lsb_a: f32,
        shunt_ohms: f32,
        adc_range: AdcRange,
    ) -> Result<(), Ina4230Error<I2c::Error>> {
        self.current_lsb_a = current_lsb_a;
        self.adc_range = adc_range;
        let cal = Self::shunt_cal_value(current_lsb_a, shunt_ohms, adc_range);
        match channel {
            Channel::Ch1 => {
                self.device.calibration_ch_1().write_async(|w| w.set_shunt_cal(cal)).await
            }
            Channel::Ch2 => {
                self.device.calibration_ch_2().write_async(|w| w.set_shunt_cal(cal)).await
            }
            Channel::Ch3 => {
                self.device.calibration_ch_3().write_async(|w| w.set_shunt_cal(cal)).await
            }
            Channel::Ch4 => {
                self.device.calibration_ch_4().write_async(|w| w.set_shunt_cal(cal)).await
            }
        }
    }

    /// Write the calibration register for all four channels with identical parameters.
    pub async fn calibrate_all(
        &mut self,
        current_lsb_a: f32,
        shunt_ohms: f32,
        adc_range: AdcRange,
    ) -> Result<(), Ina4230Error<I2c::Error>> {
        for ch in [Channel::Ch1, Channel::Ch2, Channel::Ch3, Channel::Ch4] {
            self.calibrate(ch, current_lsb_a, shunt_ohms, adc_range).await?;
        }
        Ok(())
    }

    // ── Internal unit-conversion helpers ──────────────────────────────────

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn shunt_cal_value(current_lsb_a: f32, shunt_ohms: f32, adc_range: AdcRange) -> u16 {
        // INA4230 datasheet §8.1.2: SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)
        let val = 0.00512_f32 / (current_lsb_a * shunt_ohms);
        let val = match adc_range {
            AdcRange::Range0 => val,
            AdcRange::Range1 => val / 4.0,
        };
        (val as u32).min(u32::from(u16::MAX)) as u16
    }

    fn bus_mv(raw: u16) -> MilliVolts {
        f32::from(raw) * 1.6
    }

    #[allow(clippy::cast_possible_wrap)]
    fn shunt_mv(&self, raw: u16) -> MilliVolts {
        let signed = raw as i16;
        let lsb_mv = match self.adc_range {
            AdcRange::Range0 => 0.0025,    // 2.5 µV
            AdcRange::Range1 => 0.000625,  // 625 nV
        };
        f32::from(signed) * lsb_mv
    }

    #[allow(clippy::cast_possible_wrap)]
    fn current_ma(&self, raw: u16) -> MilliAmps {
        let signed = raw as i16;
        f32::from(signed) * self.current_lsb_a * 1000.0
    }

    fn power_mw(&self, raw: u16) -> MilliWatts {
        f32::from(raw) * 32.0 * self.current_lsb_a * 1000.0
    }

    fn energy_mj(&self, raw: u32) -> MilliJoules {
        raw as f32 * 32.0 * self.current_lsb_a * 1000.0
    }
}

// ── Trait implementations ─────────────────────────────────────────────────────

impl<I2c: embedded_hal_async::i2c::I2c> sensor::ErrorType for Ina4230<I2c> {
    type Error = Ina4230Error<I2c::Error>;
}

impl<I2c: embedded_hal_async::i2c::I2c> VoltageSensor for Ina4230<I2c> {
    async fn bus_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.bus_voltage_ch_1().read_async().await?.vbus(),
            Channel::Ch2 => self.device.bus_voltage_ch_2().read_async().await?.vbus(),
            Channel::Ch3 => self.device.bus_voltage_ch_3().read_async().await?.vbus(),
            Channel::Ch4 => self.device.bus_voltage_ch_4().read_async().await?.vbus(),
        };
        Ok(Self::bus_mv(raw))
    }

    async fn shunt_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.shunt_voltage_ch_1().read_async().await?.vshunt(),
            Channel::Ch2 => self.device.shunt_voltage_ch_2().read_async().await?.vshunt(),
            Channel::Ch3 => self.device.shunt_voltage_ch_3().read_async().await?.vshunt(),
            Channel::Ch4 => self.device.shunt_voltage_ch_4().read_async().await?.vshunt(),
        };
        Ok(self.shunt_mv(raw))
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> CurrentSensor for Ina4230<I2c> {
    async fn current(&mut self, channel: Channel) -> Result<MilliAmps, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.current_ch_1().read_async().await?.current(),
            Channel::Ch2 => self.device.current_ch_2().read_async().await?.current(),
            Channel::Ch3 => self.device.current_ch_3().read_async().await?.current(),
            Channel::Ch4 => self.device.current_ch_4().read_async().await?.current(),
        };
        Ok(self.current_ma(raw))
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> PowerSensor for Ina4230<I2c> {
    async fn power(&mut self, channel: Channel) -> Result<MilliWatts, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.power_ch_1().read_async().await?.power(),
            Channel::Ch2 => self.device.power_ch_2().read_async().await?.power(),
            Channel::Ch3 => self.device.power_ch_3().read_async().await?.power(),
            Channel::Ch4 => self.device.power_ch_4().read_async().await?.power(),
        };
        Ok(self.power_mw(raw))
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> EnergySensor for Ina4230<I2c> {
    async fn energy(&mut self, channel: Channel) -> Result<MilliJoules, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.energy_ch_1().read_async().await?.energy(),
            Channel::Ch2 => self.device.energy_ch_2().read_async().await?.energy(),
            Channel::Ch3 => self.device.energy_ch_3().read_async().await?.energy(),
            Channel::Ch4 => self.device.energy_ch_4().read_async().await?.energy(),
        };
        Ok(self.energy_mj(raw))
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    use super::*;

    #[tokio::test]
    async fn read_manufacturer_id() {
        // ManufacturerId: address 0x7E, 2 bytes BE, resets to 0x5449 (TI in ASCII)
        let expectations = vec![Transaction::write_read(
            INA4230_ADDR,
            vec![0x7E],
            vec![0x54, 0x49],
        )];
        let i2c = Mock::new(&expectations);
        let mut dev = Device::new(DeviceInterface { i2c, address: INA4230_ADDR });
        let id = dev.manufacturer_id().read_async().await.unwrap();
        assert_eq!(id.id(), 0x5449);
        dev.interface.i2c.done();
    }

    #[tokio::test]
    async fn write_calibration_ch1() {
        // calibration_ch_1: address 0x05, 2 bytes BE
        // shunt_cal for 100µA/LSB, 10mΩ: 0.00512 / (100e-6 * 0.010) = 5120
        let cal: u16 = 5120;
        let [hi, lo] = cal.to_be_bytes();
        let expectations = vec![Transaction::write(INA4230_ADDR, vec![0x05, hi, lo])];
        let i2c = Mock::new(&expectations);
        let mut dev = Device::new(DeviceInterface { i2c, address: INA4230_ADDR });
        dev.calibration_ch_1()
            .write_async(|w| w.set_shunt_cal(cal))
            .await
            .unwrap();
        dev.interface.i2c.done();
    }

    #[tokio::test]
    async fn bus_voltage_ch1_trait() {
        // bus_voltage_ch_1: address 0x01, 2 bytes BE
        // raw = 5000 → 5000 * 1.6 mV = 8000.0 mV
        let raw: u16 = 5000;
        let [hi, lo] = raw.to_be_bytes();
        let expectations = vec![Transaction::write_read(
            INA4230_ADDR,
            vec![0x01],
            vec![hi, lo],
        )];
        let i2c = Mock::new(&expectations);
        let mut sensor = Ina4230::new(i2c, INA4230_ADDR);
        let mv = sensor.bus_voltage(Channel::Ch1).await.unwrap();
        assert!((mv - 8000.0).abs() < 0.1, "expected 8000.0 mV, got {mv}");
        sensor.device.interface.i2c.done();
    }

    #[tokio::test]
    async fn current_ch1_trait() {
        // current_ch_1: address 0x02, 2 bytes BE
        // raw = 1000, CURRENT_LSB = 100µA/LSB → 1000 * 100e-6 * 1000 = 100.0 mA
        let raw: u16 = 1000;
        let [hi, lo] = raw.to_be_bytes();
        let expectations = vec![Transaction::write_read(
            INA4230_ADDR,
            vec![0x02],
            vec![hi, lo],
        )];
        let i2c = Mock::new(&expectations);
        let mut sensor = Ina4230::new(i2c, INA4230_ADDR);
        sensor.current_lsb_a = 100e-6;
        let ma = sensor.current(Channel::Ch1).await.unwrap();
        assert!((ma - 100.0).abs() < 0.01, "expected 100.0 mA, got {ma}");
        sensor.device.interface.i2c.done();
    }
}
