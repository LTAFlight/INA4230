/// Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u8,
}
impl<I> Device<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self {
            interface,
            base_address: 0,
        }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.config_1().read()?;
        callback(32 + 0 * 0, "config_1", reg.into());
        let reg = self.config_2().read()?;
        callback(33 + 0 * 0, "config_2", reg.into());
        let reg = self.calibration_ch_1().read()?;
        callback(5 + 0 * 0, "calibration_ch_1", reg.into());
        let reg = self.calibration_ch_2().read()?;
        callback(13 + 0 * 0, "calibration_ch_2", reg.into());
        let reg = self.calibration_ch_3().read()?;
        callback(21 + 0 * 0, "calibration_ch_3", reg.into());
        let reg = self.calibration_ch_4().read()?;
        callback(29 + 0 * 0, "calibration_ch_4", reg.into());
        let reg = self.alert_config_1().read()?;
        callback(7 + 0 * 0, "alert_config_1", reg.into());
        let reg = self.alert_config_2().read()?;
        callback(15 + 0 * 0, "alert_config_2", reg.into());
        let reg = self.alert_config_3().read()?;
        callback(23 + 0 * 0, "alert_config_3", reg.into());
        let reg = self.alert_config_4().read()?;
        callback(31 + 0 * 0, "alert_config_4", reg.into());
        let reg = self.alert_limit_1().read()?;
        callback(6 + 0 * 0, "alert_limit_1", reg.into());
        let reg = self.alert_limit_2().read()?;
        callback(14 + 0 * 0, "alert_limit_2", reg.into());
        let reg = self.alert_limit_3().read()?;
        callback(22 + 0 * 0, "alert_limit_3", reg.into());
        let reg = self.alert_limit_4().read()?;
        callback(30 + 0 * 0, "alert_limit_4", reg.into());
        let reg = self.shunt_voltage_ch_1().read()?;
        callback(0 + 0 * 0, "shunt_voltage_ch_1", reg.into());
        let reg = self.shunt_voltage_ch_2().read()?;
        callback(8 + 0 * 0, "shunt_voltage_ch_2", reg.into());
        let reg = self.shunt_voltage_ch_3().read()?;
        callback(16 + 0 * 0, "shunt_voltage_ch_3", reg.into());
        let reg = self.shunt_voltage_ch_4().read()?;
        callback(24 + 0 * 0, "shunt_voltage_ch_4", reg.into());
        let reg = self.bus_voltage_ch_1().read()?;
        callback(1 + 0 * 0, "bus_voltage_ch_1", reg.into());
        let reg = self.bus_voltage_ch_2().read()?;
        callback(9 + 0 * 0, "bus_voltage_ch_2", reg.into());
        let reg = self.bus_voltage_ch_3().read()?;
        callback(17 + 0 * 0, "bus_voltage_ch_3", reg.into());
        let reg = self.bus_voltage_ch_4().read()?;
        callback(25 + 0 * 0, "bus_voltage_ch_4", reg.into());
        let reg = self.current_ch_1().read()?;
        callback(2 + 0 * 0, "current_ch_1", reg.into());
        let reg = self.current_ch_2().read()?;
        callback(10 + 0 * 0, "current_ch_2", reg.into());
        let reg = self.current_ch_3().read()?;
        callback(18 + 0 * 0, "current_ch_3", reg.into());
        let reg = self.current_ch_4().read()?;
        callback(26 + 0 * 0, "current_ch_4", reg.into());
        let reg = self.power_ch_1().read()?;
        callback(3 + 0 * 0, "power_ch_1", reg.into());
        let reg = self.power_ch_2().read()?;
        callback(11 + 0 * 0, "power_ch_2", reg.into());
        let reg = self.power_ch_3().read()?;
        callback(19 + 0 * 0, "power_ch_3", reg.into());
        let reg = self.power_ch_4().read()?;
        callback(27 + 0 * 0, "power_ch_4", reg.into());
        let reg = self.energy_ch_1().read()?;
        callback(4 + 0 * 0, "energy_ch_1", reg.into());
        let reg = self.energy_ch_2().read()?;
        callback(12 + 0 * 0, "energy_ch_2", reg.into());
        let reg = self.energy_ch_3().read()?;
        callback(20 + 0 * 0, "energy_ch_3", reg.into());
        let reg = self.energy_ch_4().read()?;
        callback(28 + 0 * 0, "energy_ch_4", reg.into());
        let reg = self.flags().read()?;
        callback(34 + 0 * 0, "flags", reg.into());
        let reg = self.manufacturer_id().read()?;
        callback(126 + 0 * 0, "manufacturer_id", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.config_1().read_async().await?;
        callback(32 + 0 * 0, "config_1", reg.into());
        let reg = self.config_2().read_async().await?;
        callback(33 + 0 * 0, "config_2", reg.into());
        let reg = self.calibration_ch_1().read_async().await?;
        callback(5 + 0 * 0, "calibration_ch_1", reg.into());
        let reg = self.calibration_ch_2().read_async().await?;
        callback(13 + 0 * 0, "calibration_ch_2", reg.into());
        let reg = self.calibration_ch_3().read_async().await?;
        callback(21 + 0 * 0, "calibration_ch_3", reg.into());
        let reg = self.calibration_ch_4().read_async().await?;
        callback(29 + 0 * 0, "calibration_ch_4", reg.into());
        let reg = self.alert_config_1().read_async().await?;
        callback(7 + 0 * 0, "alert_config_1", reg.into());
        let reg = self.alert_config_2().read_async().await?;
        callback(15 + 0 * 0, "alert_config_2", reg.into());
        let reg = self.alert_config_3().read_async().await?;
        callback(23 + 0 * 0, "alert_config_3", reg.into());
        let reg = self.alert_config_4().read_async().await?;
        callback(31 + 0 * 0, "alert_config_4", reg.into());
        let reg = self.alert_limit_1().read_async().await?;
        callback(6 + 0 * 0, "alert_limit_1", reg.into());
        let reg = self.alert_limit_2().read_async().await?;
        callback(14 + 0 * 0, "alert_limit_2", reg.into());
        let reg = self.alert_limit_3().read_async().await?;
        callback(22 + 0 * 0, "alert_limit_3", reg.into());
        let reg = self.alert_limit_4().read_async().await?;
        callback(30 + 0 * 0, "alert_limit_4", reg.into());
        let reg = self.shunt_voltage_ch_1().read_async().await?;
        callback(0 + 0 * 0, "shunt_voltage_ch_1", reg.into());
        let reg = self.shunt_voltage_ch_2().read_async().await?;
        callback(8 + 0 * 0, "shunt_voltage_ch_2", reg.into());
        let reg = self.shunt_voltage_ch_3().read_async().await?;
        callback(16 + 0 * 0, "shunt_voltage_ch_3", reg.into());
        let reg = self.shunt_voltage_ch_4().read_async().await?;
        callback(24 + 0 * 0, "shunt_voltage_ch_4", reg.into());
        let reg = self.bus_voltage_ch_1().read_async().await?;
        callback(1 + 0 * 0, "bus_voltage_ch_1", reg.into());
        let reg = self.bus_voltage_ch_2().read_async().await?;
        callback(9 + 0 * 0, "bus_voltage_ch_2", reg.into());
        let reg = self.bus_voltage_ch_3().read_async().await?;
        callback(17 + 0 * 0, "bus_voltage_ch_3", reg.into());
        let reg = self.bus_voltage_ch_4().read_async().await?;
        callback(25 + 0 * 0, "bus_voltage_ch_4", reg.into());
        let reg = self.current_ch_1().read_async().await?;
        callback(2 + 0 * 0, "current_ch_1", reg.into());
        let reg = self.current_ch_2().read_async().await?;
        callback(10 + 0 * 0, "current_ch_2", reg.into());
        let reg = self.current_ch_3().read_async().await?;
        callback(18 + 0 * 0, "current_ch_3", reg.into());
        let reg = self.current_ch_4().read_async().await?;
        callback(26 + 0 * 0, "current_ch_4", reg.into());
        let reg = self.power_ch_1().read_async().await?;
        callback(3 + 0 * 0, "power_ch_1", reg.into());
        let reg = self.power_ch_2().read_async().await?;
        callback(11 + 0 * 0, "power_ch_2", reg.into());
        let reg = self.power_ch_3().read_async().await?;
        callback(19 + 0 * 0, "power_ch_3", reg.into());
        let reg = self.power_ch_4().read_async().await?;
        callback(27 + 0 * 0, "power_ch_4", reg.into());
        let reg = self.energy_ch_1().read_async().await?;
        callback(4 + 0 * 0, "energy_ch_1", reg.into());
        let reg = self.energy_ch_2().read_async().await?;
        callback(12 + 0 * 0, "energy_ch_2", reg.into());
        let reg = self.energy_ch_3().read_async().await?;
        callback(20 + 0 * 0, "energy_ch_3", reg.into());
        let reg = self.energy_ch_4().read_async().await?;
        callback(28 + 0 * 0, "energy_ch_4", reg.into());
        let reg = self.flags().read_async().await?;
        callback(34 + 0 * 0, "flags", reg.into());
        let reg = self.manufacturer_id().read_async().await?;
        callback(126 + 0 * 0, "manufacturer_id", reg.into());
        Ok(())
    }
    /// Configuration register 1
    pub fn config_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::Config1, ::device_driver::RW> {
        let address = self.base_address + 32;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::Config1, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::Config1::new,
        )
    }
    /// Configuration register 2
    pub fn config_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::Config2, ::device_driver::RW> {
        let address = self.base_address + 33;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::Config2, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::Config2::new,
        )
    }
    /// Calibration register channel 1
    pub fn calibration_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CalibrationCh1, ::device_driver::RW> {
        let address = self.base_address + 5;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CalibrationCh1, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::CalibrationCh1::new,
        )
    }
    /// Calibration register channel 2
    pub fn calibration_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CalibrationCh2, ::device_driver::RW> {
        let address = self.base_address + 13;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CalibrationCh2, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::CalibrationCh2::new,
        )
    }
    /// Calibration register channel 3
    pub fn calibration_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CalibrationCh3, ::device_driver::RW> {
        let address = self.base_address + 21;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CalibrationCh3, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::CalibrationCh3::new,
        )
    }
    /// Calibration register channel 4
    pub fn calibration_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CalibrationCh4, ::device_driver::RW> {
        let address = self.base_address + 29;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CalibrationCh4, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::CalibrationCh4::new,
        )
    }
    /// Alert configuration register 1
    pub fn alert_config_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AlertConfig1, ::device_driver::RW> {
        let address = self.base_address + 7;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AlertConfig1, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AlertConfig1::new,
        )
    }
    /// Alert configuration register 2
    pub fn alert_config_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AlertConfig2, ::device_driver::RW> {
        let address = self.base_address + 15;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AlertConfig2, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AlertConfig2::new,
        )
    }
    /// Alert configuration register 3
    pub fn alert_config_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AlertConfig3, ::device_driver::RW> {
        let address = self.base_address + 23;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AlertConfig3, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AlertConfig3::new,
        )
    }
    /// Alert configuration register 4
    pub fn alert_config_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AlertConfig4, ::device_driver::RW> {
        let address = self.base_address + 31;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AlertConfig4, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AlertConfig4::new,
        )
    }
    /// Alert limit register 1. Format matches corresponding result register.
    pub fn alert_limit_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AlertLimit1, ::device_driver::RW> {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AlertLimit1, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AlertLimit1::new,
        )
    }
    /// Alert limit register 2
    pub fn alert_limit_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AlertLimit2, ::device_driver::RW> {
        let address = self.base_address + 14;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AlertLimit2, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AlertLimit2::new,
        )
    }
    /// Alert limit register 3
    pub fn alert_limit_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AlertLimit3, ::device_driver::RW> {
        let address = self.base_address + 22;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AlertLimit3, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AlertLimit3::new,
        )
    }
    /// Alert limit register 4
    pub fn alert_limit_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AlertLimit4, ::device_driver::RW> {
        let address = self.base_address + 30;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AlertLimit4, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AlertLimit4::new,
        )
    }
    /// Shunt voltage channel 1. 2's complement. LSB = 2.5uV (range=0) or 625nV (range=1)
    pub fn shunt_voltage_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ShuntVoltageCh1, ::device_driver::RO> {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ShuntVoltageCh1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ShuntVoltageCh1::new,
        )
    }
    /// Shunt voltage channel 2
    pub fn shunt_voltage_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ShuntVoltageCh2, ::device_driver::RO> {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ShuntVoltageCh2, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ShuntVoltageCh2::new,
        )
    }
    /// Shunt voltage channel 3
    pub fn shunt_voltage_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ShuntVoltageCh3, ::device_driver::RO> {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ShuntVoltageCh3, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ShuntVoltageCh3::new,
        )
    }
    /// Shunt voltage channel 4
    pub fn shunt_voltage_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ShuntVoltageCh4, ::device_driver::RO> {
        let address = self.base_address + 24;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ShuntVoltageCh4, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ShuntVoltageCh4::new,
        )
    }
    /// Bus voltage channel 1. Always positive. LSB = 1.6mV
    pub fn bus_voltage_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::BusVoltageCh1, ::device_driver::RO> {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::BusVoltageCh1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::BusVoltageCh1::new,
        )
    }
    /// Bus voltage channel 2
    pub fn bus_voltage_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::BusVoltageCh2, ::device_driver::RO> {
        let address = self.base_address + 9;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::BusVoltageCh2, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::BusVoltageCh2::new,
        )
    }
    /// Bus voltage channel 3
    pub fn bus_voltage_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::BusVoltageCh3, ::device_driver::RO> {
        let address = self.base_address + 17;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::BusVoltageCh3, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::BusVoltageCh3::new,
        )
    }
    /// Bus voltage channel 4
    pub fn bus_voltage_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::BusVoltageCh4, ::device_driver::RO> {
        let address = self.base_address + 25;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::BusVoltageCh4, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::BusVoltageCh4::new,
        )
    }
    /// Current channel 1. Value [A] = CURRENT_LSB x register_value
    pub fn current_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CurrentCh1, ::device_driver::RO> {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CurrentCh1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CurrentCh1::new,
        )
    }
    /// Current channel 2
    pub fn current_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CurrentCh2, ::device_driver::RO> {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CurrentCh2, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CurrentCh2::new,
        )
    }
    /// Current channel 3
    pub fn current_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CurrentCh3, ::device_driver::RO> {
        let address = self.base_address + 18;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CurrentCh3, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CurrentCh3::new,
        )
    }
    /// Current channel 4
    pub fn current_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CurrentCh4, ::device_driver::RO> {
        let address = self.base_address + 26;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CurrentCh4, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CurrentCh4::new,
        )
    }
    /// Power channel 1. Value [W] = 32 x CURRENT_LSB x register_value. Unsigned.
    pub fn power_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::PowerCh1, ::device_driver::RO> {
        let address = self.base_address + 3;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::PowerCh1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::PowerCh1::new,
        )
    }
    /// Power channel 2
    pub fn power_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::PowerCh2, ::device_driver::RO> {
        let address = self.base_address + 11;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::PowerCh2, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::PowerCh2::new,
        )
    }
    /// Power channel 3
    pub fn power_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::PowerCh3, ::device_driver::RO> {
        let address = self.base_address + 19;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::PowerCh3, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::PowerCh3::new,
        )
    }
    /// Power channel 4
    pub fn power_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::PowerCh4, ::device_driver::RO> {
        let address = self.base_address + 27;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::PowerCh4, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::PowerCh4::new,
        )
    }
    /// Energy channel 1. Value [J] = 32 x CURRENT_LSB x register_value. Unsigned.
    pub fn energy_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::EnergyCh1, ::device_driver::RO> {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::EnergyCh1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::EnergyCh1::new,
        )
    }
    /// Energy channel 2
    pub fn energy_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::EnergyCh2, ::device_driver::RO> {
        let address = self.base_address + 12;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::EnergyCh2, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::EnergyCh2::new,
        )
    }
    /// Energy channel 3
    pub fn energy_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::EnergyCh3, ::device_driver::RO> {
        let address = self.base_address + 20;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::EnergyCh3, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::EnergyCh3::new,
        )
    }
    /// Energy channel 4
    pub fn energy_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::EnergyCh4, ::device_driver::RO> {
        let address = self.base_address + 28;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::EnergyCh4, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::EnergyCh4::new,
        )
    }
    /// Flags register
    pub fn flags(&mut self) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::Flags, ::device_driver::RO> {
        let address = self.base_address + 34;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::Flags, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::Flags::new,
        )
    }
    /// Manufacturer ID. Reads back 0x5449 ('TI' in ASCII)
    pub fn manufacturer_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ManufacturerId, ::device_driver::RO> {
        let address = self.base_address + 126;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ManufacturerId, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ManufacturerId::new,
        )
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    #[allow(unused_imports)]
    use super::*;
    /// Configuration register 1
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for Config1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Config1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [241, 39] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `active_channel` field of the register.
        ///
        /// Active channel enable bits. Bit15=CH4, Bit14=CH3, Bit13=CH2, Bit12=CH1
        pub fn active_channel(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 12, 16) };
            raw
        }
        ///Read the `avg` field of the register.
        ///
        /// Number of ADC conversion results to average
        pub fn avg(&self) -> super::Averaging {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 9, 12) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vbusct` field of the register.
        ///
        /// Bus voltage conversion time
        pub fn vbusct(&self) -> super::BusConversionTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 6, 9) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vshct` field of the register.
        ///
        /// Shunt voltage conversion time
        pub fn vshct(&self) -> super::ShuntConversionTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 3, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `mode` field of the register.
        ///
        /// Operating mode
        pub fn mode(&self) -> super::Mode {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 0, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `active_channel` field of the register.
        ///
        /// Active channel enable bits. Bit15=CH4, Bit14=CH3, Bit13=CH2, Bit12=CH1
        pub fn set_active_channel(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 12, 16, &mut self.bits) };
        }
        ///Write the `avg` field of the register.
        ///
        /// Number of ADC conversion results to average
        pub fn set_avg(&mut self, value: super::Averaging) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 9, 12, &mut self.bits) };
        }
        ///Write the `vbusct` field of the register.
        ///
        /// Bus voltage conversion time
        pub fn set_vbusct(&mut self, value: super::BusConversionTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 6, 9, &mut self.bits) };
        }
        ///Write the `vshct` field of the register.
        ///
        /// Shunt voltage conversion time
        pub fn set_vshct(&mut self, value: super::ShuntConversionTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 3, 6, &mut self.bits) };
        }
        ///Write the `mode` field of the register.
        ///
        /// Operating mode
        pub fn set_mode(&mut self, value: super::Mode) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 0, 3, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for Config1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<Config1> for [u8; 2] {
        fn from(val: Config1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Config1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Config1");
            d.field("active_channel", &self.active_channel());
            d.field("avg", &self.avg());
            d.field("vbusct", &self.vbusct());
            d.field("vshct", &self.vshct());
            d.field("mode", &self.mode());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Config1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Config1 {{ ");
            defmt::write!(f, "active_channel: {=u8}, ", &self.active_channel());
            defmt::write!(f, "avg: {}, ", &self.avg());
            defmt::write!(f, "vbusct: {}, ", &self.vbusct());
            defmt::write!(f, "vshct: {}, ", &self.vshct());
            defmt::write!(f, "mode: {}, ", &self.mode());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for Config1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Config1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Config1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Config1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Config1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Config1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Config1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Configuration register 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for Config2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Config2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `rst` field of the register.
        ///
        /// System reset. Set to 1 to reset all registers to default. Self-clears.
        pub fn rst(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Read the `acc_rst` field of the register.
        ///
        /// Energy accumulator reset per channel. Bit11=CH4, Bit10=CH3, Bit9=CH2, Bit8=CH1. Bits self-clear after write.
        pub fn acc_rst(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 8, 12) };
            raw
        }
        ///Read the `cnvr_mask` field of the register.
        ///
        /// Conversion ready flag on ALERT pin enable
        pub fn cnvr_mask(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `enof_mask` field of the register.
        ///
        /// Energy overflow alert enable
        pub fn enof_mask(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `alert_latch` field of the register.
        ///
        /// Alert pin latch enable. When set, alert latches until flags register is read.
        pub fn alert_latch(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `alert_pol` field of the register.
        ///
        /// Alert pin polarity
        pub fn alert_pol(&self) -> super::AlertPolarity {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 4, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `range` field of the register.
        ///
        /// Shunt full scale input range per channel. Bit3=CH4, Bit2=CH3, Bit1=CH2, Bit0=CH1. 0=±81.92mV, 1=±20.48mV
        pub fn range(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 0, 4) };
            raw
        }
        ///Write the `rst` field of the register.
        ///
        /// System reset. Set to 1 to reset all registers to default. Self-clears.
        pub fn set_rst(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 15, 16, &mut self.bits) };
        }
        ///Write the `acc_rst` field of the register.
        ///
        /// Energy accumulator reset per channel. Bit11=CH4, Bit10=CH3, Bit9=CH2, Bit8=CH1. Bits self-clear after write.
        pub fn set_acc_rst(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 8, 12, &mut self.bits) };
        }
        ///Write the `cnvr_mask` field of the register.
        ///
        /// Conversion ready flag on ALERT pin enable
        pub fn set_cnvr_mask(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `enof_mask` field of the register.
        ///
        /// Energy overflow alert enable
        pub fn set_enof_mask(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `alert_latch` field of the register.
        ///
        /// Alert pin latch enable. When set, alert latches until flags register is read.
        pub fn set_alert_latch(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `alert_pol` field of the register.
        ///
        /// Alert pin polarity
        pub fn set_alert_pol(&mut self, value: super::AlertPolarity) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `range` field of the register.
        ///
        /// Shunt full scale input range per channel. Bit3=CH4, Bit2=CH3, Bit1=CH2, Bit0=CH1. 0=±81.92mV, 1=±20.48mV
        pub fn set_range(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 0, 4, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for Config2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<Config2> for [u8; 2] {
        fn from(val: Config2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Config2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Config2");
            d.field("rst", &self.rst());
            d.field("acc_rst", &self.acc_rst());
            d.field("cnvr_mask", &self.cnvr_mask());
            d.field("enof_mask", &self.enof_mask());
            d.field("alert_latch", &self.alert_latch());
            d.field("alert_pol", &self.alert_pol());
            d.field("range", &self.range());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Config2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Config2 {{ ");
            defmt::write!(f, "rst: {=bool}, ", &self.rst());
            defmt::write!(f, "acc_rst: {=u8}, ", &self.acc_rst());
            defmt::write!(f, "cnvr_mask: {=bool}, ", &self.cnvr_mask());
            defmt::write!(f, "enof_mask: {=bool}, ", &self.enof_mask());
            defmt::write!(f, "alert_latch: {=bool}, ", &self.alert_latch());
            defmt::write!(f, "alert_pol: {}, ", &self.alert_pol());
            defmt::write!(f, "range: {=u8}, ", &self.range());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for Config2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Config2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Config2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Config2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Config2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Config2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Config2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Calibration register channel 1
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalibrationCh1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CalibrationCh1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CalibrationCh1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `shunt_cal` field of the register.
        ///
        /// Shunt calibration value for current conversion
        pub fn shunt_cal(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 15) };
            raw
        }
        ///Write the `shunt_cal` field of the register.
        ///
        /// Shunt calibration value for current conversion
        pub fn set_shunt_cal(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 15, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for CalibrationCh1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<CalibrationCh1> for [u8; 2] {
        fn from(val: CalibrationCh1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for CalibrationCh1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("CalibrationCh1");
            d.field("shunt_cal", &self.shunt_cal());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CalibrationCh1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CalibrationCh1 {{ ");
            defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for CalibrationCh1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for CalibrationCh1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for CalibrationCh1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for CalibrationCh1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for CalibrationCh1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for CalibrationCh1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for CalibrationCh1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Calibration register channel 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalibrationCh2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CalibrationCh2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CalibrationCh2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `shunt_cal` field of the register.
        ///
        /// Shunt calibration value for current conversion
        pub fn shunt_cal(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 15) };
            raw
        }
        ///Write the `shunt_cal` field of the register.
        ///
        /// Shunt calibration value for current conversion
        pub fn set_shunt_cal(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 15, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for CalibrationCh2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<CalibrationCh2> for [u8; 2] {
        fn from(val: CalibrationCh2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for CalibrationCh2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("CalibrationCh2");
            d.field("shunt_cal", &self.shunt_cal());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CalibrationCh2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CalibrationCh2 {{ ");
            defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for CalibrationCh2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for CalibrationCh2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for CalibrationCh2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for CalibrationCh2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for CalibrationCh2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for CalibrationCh2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for CalibrationCh2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Calibration register channel 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalibrationCh3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CalibrationCh3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CalibrationCh3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `shunt_cal` field of the register.
        ///
        /// Shunt calibration value for current conversion
        pub fn shunt_cal(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 15) };
            raw
        }
        ///Write the `shunt_cal` field of the register.
        ///
        /// Shunt calibration value for current conversion
        pub fn set_shunt_cal(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 15, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for CalibrationCh3 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<CalibrationCh3> for [u8; 2] {
        fn from(val: CalibrationCh3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for CalibrationCh3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("CalibrationCh3");
            d.field("shunt_cal", &self.shunt_cal());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CalibrationCh3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CalibrationCh3 {{ ");
            defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for CalibrationCh3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for CalibrationCh3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for CalibrationCh3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for CalibrationCh3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for CalibrationCh3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for CalibrationCh3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for CalibrationCh3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Calibration register channel 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalibrationCh4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CalibrationCh4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CalibrationCh4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `shunt_cal` field of the register.
        ///
        /// Shunt calibration value for current conversion
        pub fn shunt_cal(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 15) };
            raw
        }
        ///Write the `shunt_cal` field of the register.
        ///
        /// Shunt calibration value for current conversion
        pub fn set_shunt_cal(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 15, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for CalibrationCh4 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<CalibrationCh4> for [u8; 2] {
        fn from(val: CalibrationCh4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for CalibrationCh4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("CalibrationCh4");
            d.field("shunt_cal", &self.shunt_cal());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CalibrationCh4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CalibrationCh4 {{ ");
            defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for CalibrationCh4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for CalibrationCh4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for CalibrationCh4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for CalibrationCh4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for CalibrationCh4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for CalibrationCh4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for CalibrationCh4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Alert configuration register 1
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlertConfig1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AlertConfig1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AlertConfig1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `channel` field of the register.
        ///
        /// Channel assignment for this alert
        pub fn channel(&self) -> super::AlertChannel {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 3, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `alert_mask` field of the register.
        ///
        /// Active alert function selection
        pub fn alert_mask(&self) -> super::AlertFunction {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 0, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `channel` field of the register.
        ///
        /// Channel assignment for this alert
        pub fn set_channel(&mut self, value: super::AlertChannel) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `alert_mask` field of the register.
        ///
        /// Active alert function selection
        pub fn set_alert_mask(&mut self, value: super::AlertFunction) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 0, 3, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AlertConfig1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AlertConfig1> for [u8; 2] {
        fn from(val: AlertConfig1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AlertConfig1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AlertConfig1");
            d.field("channel", &self.channel());
            d.field("alert_mask", &self.alert_mask());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlertConfig1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlertConfig1 {{ ");
            defmt::write!(f, "channel: {}, ", &self.channel());
            defmt::write!(f, "alert_mask: {}, ", &self.alert_mask());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AlertConfig1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AlertConfig1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AlertConfig1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AlertConfig1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AlertConfig1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AlertConfig1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AlertConfig1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Alert configuration register 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlertConfig2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AlertConfig2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AlertConfig2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `channel` field of the register.
        ///
        /// Channel assignment for this alert
        pub fn channel(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 3, 5) };
            raw
        }
        ///Read the `alert_mask` field of the register.
        ///
        /// Active alert function selection
        pub fn alert_mask(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 0, 3) };
            raw
        }
        ///Write the `channel` field of the register.
        ///
        /// Channel assignment for this alert
        pub fn set_channel(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `alert_mask` field of the register.
        ///
        /// Active alert function selection
        pub fn set_alert_mask(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 0, 3, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AlertConfig2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AlertConfig2> for [u8; 2] {
        fn from(val: AlertConfig2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AlertConfig2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AlertConfig2");
            d.field("channel", &self.channel());
            d.field("alert_mask", &self.alert_mask());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlertConfig2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlertConfig2 {{ ");
            defmt::write!(f, "channel: {=u8}, ", &self.channel());
            defmt::write!(f, "alert_mask: {=u8}, ", &self.alert_mask());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AlertConfig2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AlertConfig2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AlertConfig2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AlertConfig2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AlertConfig2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AlertConfig2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AlertConfig2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Alert configuration register 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlertConfig3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AlertConfig3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AlertConfig3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `channel` field of the register.
        ///
        /// Channel assignment for this alert
        pub fn channel(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 3, 5) };
            raw
        }
        ///Read the `alert_mask` field of the register.
        ///
        /// Active alert function selection
        pub fn alert_mask(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 0, 3) };
            raw
        }
        ///Write the `channel` field of the register.
        ///
        /// Channel assignment for this alert
        pub fn set_channel(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `alert_mask` field of the register.
        ///
        /// Active alert function selection
        pub fn set_alert_mask(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 0, 3, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AlertConfig3 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AlertConfig3> for [u8; 2] {
        fn from(val: AlertConfig3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AlertConfig3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AlertConfig3");
            d.field("channel", &self.channel());
            d.field("alert_mask", &self.alert_mask());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlertConfig3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlertConfig3 {{ ");
            defmt::write!(f, "channel: {=u8}, ", &self.channel());
            defmt::write!(f, "alert_mask: {=u8}, ", &self.alert_mask());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AlertConfig3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AlertConfig3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AlertConfig3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AlertConfig3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AlertConfig3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AlertConfig3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AlertConfig3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Alert configuration register 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlertConfig4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AlertConfig4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AlertConfig4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `channel` field of the register.
        ///
        /// Channel assignment for this alert
        pub fn channel(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 3, 5) };
            raw
        }
        ///Read the `alert_mask` field of the register.
        ///
        /// Active alert function selection
        pub fn alert_mask(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 0, 3) };
            raw
        }
        ///Write the `channel` field of the register.
        ///
        /// Channel assignment for this alert
        pub fn set_channel(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `alert_mask` field of the register.
        ///
        /// Active alert function selection
        pub fn set_alert_mask(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 0, 3, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AlertConfig4 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AlertConfig4> for [u8; 2] {
        fn from(val: AlertConfig4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AlertConfig4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AlertConfig4");
            d.field("channel", &self.channel());
            d.field("alert_mask", &self.alert_mask());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlertConfig4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlertConfig4 {{ ");
            defmt::write!(f, "channel: {=u8}, ", &self.channel());
            defmt::write!(f, "alert_mask: {=u8}, ", &self.alert_mask());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AlertConfig4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AlertConfig4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AlertConfig4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AlertConfig4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AlertConfig4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AlertConfig4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AlertConfig4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Alert limit register 1. Format matches corresponding result register.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlertLimit1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AlertLimit1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AlertLimit1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `limit` field of the register.
        ///
        /// Alert threshold. Shunt=signed 16-bit, Bus=unsigned 15-bit, Power=unsigned 16-bit
        pub fn limit(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `limit` field of the register.
        ///
        /// Alert threshold. Shunt=signed 16-bit, Bus=unsigned 15-bit, Power=unsigned 16-bit
        pub fn set_limit(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AlertLimit1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AlertLimit1> for [u8; 2] {
        fn from(val: AlertLimit1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AlertLimit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AlertLimit1");
            d.field("limit", &self.limit());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlertLimit1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlertLimit1 {{ ");
            defmt::write!(f, "limit: {=u16}, ", &self.limit());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AlertLimit1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AlertLimit1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AlertLimit1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AlertLimit1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AlertLimit1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AlertLimit1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AlertLimit1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Alert limit register 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlertLimit2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AlertLimit2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AlertLimit2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `limit` field of the register.
        ///
        /// Alert threshold
        pub fn limit(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `limit` field of the register.
        ///
        /// Alert threshold
        pub fn set_limit(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AlertLimit2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AlertLimit2> for [u8; 2] {
        fn from(val: AlertLimit2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AlertLimit2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AlertLimit2");
            d.field("limit", &self.limit());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlertLimit2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlertLimit2 {{ ");
            defmt::write!(f, "limit: {=u16}, ", &self.limit());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AlertLimit2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AlertLimit2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AlertLimit2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AlertLimit2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AlertLimit2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AlertLimit2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AlertLimit2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Alert limit register 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlertLimit3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AlertLimit3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AlertLimit3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `limit` field of the register.
        ///
        /// Alert threshold
        pub fn limit(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `limit` field of the register.
        ///
        /// Alert threshold
        pub fn set_limit(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AlertLimit3 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AlertLimit3> for [u8; 2] {
        fn from(val: AlertLimit3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AlertLimit3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AlertLimit3");
            d.field("limit", &self.limit());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlertLimit3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlertLimit3 {{ ");
            defmt::write!(f, "limit: {=u16}, ", &self.limit());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AlertLimit3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AlertLimit3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AlertLimit3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AlertLimit3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AlertLimit3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AlertLimit3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AlertLimit3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Alert limit register 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlertLimit4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AlertLimit4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AlertLimit4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `limit` field of the register.
        ///
        /// Alert threshold
        pub fn limit(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `limit` field of the register.
        ///
        /// Alert threshold
        pub fn set_limit(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AlertLimit4 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AlertLimit4> for [u8; 2] {
        fn from(val: AlertLimit4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AlertLimit4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AlertLimit4");
            d.field("limit", &self.limit());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlertLimit4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlertLimit4 {{ ");
            defmt::write!(f, "limit: {=u16}, ", &self.limit());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AlertLimit4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AlertLimit4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AlertLimit4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AlertLimit4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AlertLimit4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AlertLimit4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AlertLimit4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Shunt voltage channel 1. 2's complement. LSB = 2.5uV (range=0) or 625nV (range=1)
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShuntVoltageCh1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ShuntVoltageCh1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ShuntVoltageCh1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vshunt` field of the register.
        ///
        /// Differential shunt voltage, 2's complement
        pub fn vshunt(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `vshunt` field of the register.
        ///
        /// Differential shunt voltage, 2's complement
        pub fn set_vshunt(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ShuntVoltageCh1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ShuntVoltageCh1> for [u8; 2] {
        fn from(val: ShuntVoltageCh1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ShuntVoltageCh1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ShuntVoltageCh1");
            d.field("vshunt", &self.vshunt());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ShuntVoltageCh1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ShuntVoltageCh1 {{ ");
            defmt::write!(f, "vshunt: {=u16}, ", &self.vshunt());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ShuntVoltageCh1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ShuntVoltageCh1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ShuntVoltageCh1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ShuntVoltageCh1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ShuntVoltageCh1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ShuntVoltageCh1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ShuntVoltageCh1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Shunt voltage channel 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShuntVoltageCh2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ShuntVoltageCh2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ShuntVoltageCh2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vshunt` field of the register.
        ///
        /// Differential shunt voltage, 2's complement
        pub fn vshunt(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `vshunt` field of the register.
        ///
        /// Differential shunt voltage, 2's complement
        pub fn set_vshunt(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ShuntVoltageCh2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ShuntVoltageCh2> for [u8; 2] {
        fn from(val: ShuntVoltageCh2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ShuntVoltageCh2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ShuntVoltageCh2");
            d.field("vshunt", &self.vshunt());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ShuntVoltageCh2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ShuntVoltageCh2 {{ ");
            defmt::write!(f, "vshunt: {=u16}, ", &self.vshunt());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ShuntVoltageCh2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ShuntVoltageCh2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ShuntVoltageCh2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ShuntVoltageCh2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ShuntVoltageCh2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ShuntVoltageCh2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ShuntVoltageCh2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Shunt voltage channel 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShuntVoltageCh3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ShuntVoltageCh3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ShuntVoltageCh3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vshunt` field of the register.
        ///
        /// Differential shunt voltage, 2's complement
        pub fn vshunt(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `vshunt` field of the register.
        ///
        /// Differential shunt voltage, 2's complement
        pub fn set_vshunt(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ShuntVoltageCh3 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ShuntVoltageCh3> for [u8; 2] {
        fn from(val: ShuntVoltageCh3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ShuntVoltageCh3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ShuntVoltageCh3");
            d.field("vshunt", &self.vshunt());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ShuntVoltageCh3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ShuntVoltageCh3 {{ ");
            defmt::write!(f, "vshunt: {=u16}, ", &self.vshunt());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ShuntVoltageCh3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ShuntVoltageCh3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ShuntVoltageCh3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ShuntVoltageCh3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ShuntVoltageCh3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ShuntVoltageCh3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ShuntVoltageCh3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Shunt voltage channel 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShuntVoltageCh4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ShuntVoltageCh4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ShuntVoltageCh4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vshunt` field of the register.
        ///
        /// Differential shunt voltage, 2's complement
        pub fn vshunt(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `vshunt` field of the register.
        ///
        /// Differential shunt voltage, 2's complement
        pub fn set_vshunt(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ShuntVoltageCh4 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ShuntVoltageCh4> for [u8; 2] {
        fn from(val: ShuntVoltageCh4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ShuntVoltageCh4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ShuntVoltageCh4");
            d.field("vshunt", &self.vshunt());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ShuntVoltageCh4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ShuntVoltageCh4 {{ ");
            defmt::write!(f, "vshunt: {=u16}, ", &self.vshunt());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ShuntVoltageCh4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ShuntVoltageCh4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ShuntVoltageCh4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ShuntVoltageCh4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ShuntVoltageCh4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ShuntVoltageCh4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ShuntVoltageCh4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Bus voltage channel 1. Always positive. LSB = 1.6mV
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusVoltageCh1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for BusVoltageCh1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BusVoltageCh1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vbus` field of the register.
        ///
        /// Bus voltage, always positive, 2's complement format
        pub fn vbus(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `vbus` field of the register.
        ///
        /// Bus voltage, always positive, 2's complement format
        pub fn set_vbus(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for BusVoltageCh1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<BusVoltageCh1> for [u8; 2] {
        fn from(val: BusVoltageCh1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BusVoltageCh1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BusVoltageCh1");
            d.field("vbus", &self.vbus());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BusVoltageCh1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BusVoltageCh1 {{ ");
            defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for BusVoltageCh1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BusVoltageCh1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BusVoltageCh1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BusVoltageCh1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BusVoltageCh1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BusVoltageCh1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BusVoltageCh1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Bus voltage channel 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusVoltageCh2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for BusVoltageCh2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BusVoltageCh2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vbus` field of the register.
        ///
        /// Bus voltage
        pub fn vbus(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `vbus` field of the register.
        ///
        /// Bus voltage
        pub fn set_vbus(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for BusVoltageCh2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<BusVoltageCh2> for [u8; 2] {
        fn from(val: BusVoltageCh2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BusVoltageCh2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BusVoltageCh2");
            d.field("vbus", &self.vbus());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BusVoltageCh2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BusVoltageCh2 {{ ");
            defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for BusVoltageCh2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BusVoltageCh2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BusVoltageCh2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BusVoltageCh2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BusVoltageCh2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BusVoltageCh2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BusVoltageCh2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Bus voltage channel 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusVoltageCh3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for BusVoltageCh3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BusVoltageCh3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vbus` field of the register.
        ///
        /// Bus voltage
        pub fn vbus(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `vbus` field of the register.
        ///
        /// Bus voltage
        pub fn set_vbus(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for BusVoltageCh3 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<BusVoltageCh3> for [u8; 2] {
        fn from(val: BusVoltageCh3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BusVoltageCh3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BusVoltageCh3");
            d.field("vbus", &self.vbus());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BusVoltageCh3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BusVoltageCh3 {{ ");
            defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for BusVoltageCh3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BusVoltageCh3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BusVoltageCh3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BusVoltageCh3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BusVoltageCh3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BusVoltageCh3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BusVoltageCh3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Bus voltage channel 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusVoltageCh4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for BusVoltageCh4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BusVoltageCh4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vbus` field of the register.
        ///
        /// Bus voltage
        pub fn vbus(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `vbus` field of the register.
        ///
        /// Bus voltage
        pub fn set_vbus(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for BusVoltageCh4 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<BusVoltageCh4> for [u8; 2] {
        fn from(val: BusVoltageCh4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BusVoltageCh4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BusVoltageCh4");
            d.field("vbus", &self.vbus());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BusVoltageCh4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BusVoltageCh4 {{ ");
            defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for BusVoltageCh4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BusVoltageCh4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BusVoltageCh4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BusVoltageCh4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BusVoltageCh4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BusVoltageCh4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BusVoltageCh4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Current channel 1. Value [A] = CURRENT_LSB x register_value
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurrentCh1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CurrentCh1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CurrentCh1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `current` field of the register.
        ///
        /// Calculated current in amperes, 2's complement
        pub fn current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `current` field of the register.
        ///
        /// Calculated current in amperes, 2's complement
        pub fn set_current(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for CurrentCh1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<CurrentCh1> for [u8; 2] {
        fn from(val: CurrentCh1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for CurrentCh1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("CurrentCh1");
            d.field("current", &self.current());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CurrentCh1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CurrentCh1 {{ ");
            defmt::write!(f, "current: {=u16}, ", &self.current());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for CurrentCh1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for CurrentCh1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for CurrentCh1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for CurrentCh1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for CurrentCh1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for CurrentCh1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for CurrentCh1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Current channel 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurrentCh2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CurrentCh2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CurrentCh2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `current` field of the register.
        ///
        /// Calculated current in amperes, 2's complement
        pub fn current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `current` field of the register.
        ///
        /// Calculated current in amperes, 2's complement
        pub fn set_current(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for CurrentCh2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<CurrentCh2> for [u8; 2] {
        fn from(val: CurrentCh2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for CurrentCh2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("CurrentCh2");
            d.field("current", &self.current());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CurrentCh2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CurrentCh2 {{ ");
            defmt::write!(f, "current: {=u16}, ", &self.current());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for CurrentCh2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for CurrentCh2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for CurrentCh2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for CurrentCh2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for CurrentCh2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for CurrentCh2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for CurrentCh2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Current channel 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurrentCh3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CurrentCh3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CurrentCh3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `current` field of the register.
        ///
        /// Calculated current in amperes, 2's complement
        pub fn current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `current` field of the register.
        ///
        /// Calculated current in amperes, 2's complement
        pub fn set_current(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for CurrentCh3 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<CurrentCh3> for [u8; 2] {
        fn from(val: CurrentCh3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for CurrentCh3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("CurrentCh3");
            d.field("current", &self.current());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CurrentCh3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CurrentCh3 {{ ");
            defmt::write!(f, "current: {=u16}, ", &self.current());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for CurrentCh3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for CurrentCh3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for CurrentCh3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for CurrentCh3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for CurrentCh3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for CurrentCh3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for CurrentCh3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Current channel 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurrentCh4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CurrentCh4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CurrentCh4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `current` field of the register.
        ///
        /// Calculated current in amperes, 2's complement
        pub fn current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `current` field of the register.
        ///
        /// Calculated current in amperes, 2's complement
        pub fn set_current(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for CurrentCh4 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<CurrentCh4> for [u8; 2] {
        fn from(val: CurrentCh4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for CurrentCh4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("CurrentCh4");
            d.field("current", &self.current());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CurrentCh4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CurrentCh4 {{ ");
            defmt::write!(f, "current: {=u16}, ", &self.current());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for CurrentCh4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for CurrentCh4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for CurrentCh4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for CurrentCh4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for CurrentCh4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for CurrentCh4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for CurrentCh4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Power channel 1. Value [W] = 32 x CURRENT_LSB x register_value. Unsigned.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerCh1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for PowerCh1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PowerCh1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `power` field of the register.
        ///
        /// Calculated power in watts, unsigned
        pub fn power(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `power` field of the register.
        ///
        /// Calculated power in watts, unsigned
        pub fn set_power(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for PowerCh1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<PowerCh1> for [u8; 2] {
        fn from(val: PowerCh1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for PowerCh1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("PowerCh1");
            d.field("power", &self.power());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PowerCh1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PowerCh1 {{ ");
            defmt::write!(f, "power: {=u16}, ", &self.power());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for PowerCh1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for PowerCh1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for PowerCh1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for PowerCh1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for PowerCh1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for PowerCh1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for PowerCh1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Power channel 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerCh2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for PowerCh2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PowerCh2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `power` field of the register.
        ///
        /// Calculated power in watts, unsigned
        pub fn power(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `power` field of the register.
        ///
        /// Calculated power in watts, unsigned
        pub fn set_power(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for PowerCh2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<PowerCh2> for [u8; 2] {
        fn from(val: PowerCh2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for PowerCh2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("PowerCh2");
            d.field("power", &self.power());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PowerCh2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PowerCh2 {{ ");
            defmt::write!(f, "power: {=u16}, ", &self.power());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for PowerCh2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for PowerCh2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for PowerCh2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for PowerCh2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for PowerCh2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for PowerCh2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for PowerCh2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Power channel 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerCh3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for PowerCh3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PowerCh3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `power` field of the register.
        ///
        /// Calculated power in watts, unsigned
        pub fn power(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `power` field of the register.
        ///
        /// Calculated power in watts, unsigned
        pub fn set_power(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for PowerCh3 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<PowerCh3> for [u8; 2] {
        fn from(val: PowerCh3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for PowerCh3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("PowerCh3");
            d.field("power", &self.power());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PowerCh3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PowerCh3 {{ ");
            defmt::write!(f, "power: {=u16}, ", &self.power());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for PowerCh3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for PowerCh3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for PowerCh3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for PowerCh3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for PowerCh3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for PowerCh3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for PowerCh3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Power channel 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerCh4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for PowerCh4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PowerCh4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `power` field of the register.
        ///
        /// Calculated power in watts, unsigned
        pub fn power(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `power` field of the register.
        ///
        /// Calculated power in watts, unsigned
        pub fn set_power(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for PowerCh4 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<PowerCh4> for [u8; 2] {
        fn from(val: PowerCh4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for PowerCh4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("PowerCh4");
            d.field("power", &self.power());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PowerCh4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PowerCh4 {{ ");
            defmt::write!(f, "power: {=u16}, ", &self.power());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for PowerCh4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for PowerCh4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for PowerCh4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for PowerCh4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for PowerCh4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for PowerCh4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for PowerCh4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Energy channel 1. Value [J] = 32 x CURRENT_LSB x register_value. Unsigned.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EnergyCh1 {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for EnergyCh1 {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl EnergyCh1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `energy` field of the register.
        ///
        /// Accumulated energy in joules, unsigned
        pub fn energy(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::BE>(&self.bits, 0, 32) };
            raw
        }
        ///Write the `energy` field of the register.
        ///
        /// Accumulated energy in joules, unsigned
        pub fn set_energy(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::BE>(raw, 0, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for EnergyCh1 {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<EnergyCh1> for [u8; 4] {
        fn from(val: EnergyCh1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for EnergyCh1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("EnergyCh1");
            d.field("energy", &self.energy());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EnergyCh1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EnergyCh1 {{ ");
            defmt::write!(f, "energy: {=u32}, ", &self.energy());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for EnergyCh1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for EnergyCh1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for EnergyCh1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for EnergyCh1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for EnergyCh1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for EnergyCh1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for EnergyCh1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Energy channel 2
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EnergyCh2 {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for EnergyCh2 {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl EnergyCh2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `energy` field of the register.
        ///
        /// Accumulated energy in joules, unsigned
        pub fn energy(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::BE>(&self.bits, 0, 32) };
            raw
        }
        ///Write the `energy` field of the register.
        ///
        /// Accumulated energy in joules, unsigned
        pub fn set_energy(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::BE>(raw, 0, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for EnergyCh2 {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<EnergyCh2> for [u8; 4] {
        fn from(val: EnergyCh2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for EnergyCh2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("EnergyCh2");
            d.field("energy", &self.energy());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EnergyCh2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EnergyCh2 {{ ");
            defmt::write!(f, "energy: {=u32}, ", &self.energy());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for EnergyCh2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for EnergyCh2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for EnergyCh2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for EnergyCh2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for EnergyCh2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for EnergyCh2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for EnergyCh2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Energy channel 3
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EnergyCh3 {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for EnergyCh3 {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl EnergyCh3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `energy` field of the register.
        ///
        /// Accumulated energy in joules, unsigned
        pub fn energy(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::BE>(&self.bits, 0, 32) };
            raw
        }
        ///Write the `energy` field of the register.
        ///
        /// Accumulated energy in joules, unsigned
        pub fn set_energy(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::BE>(raw, 0, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for EnergyCh3 {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<EnergyCh3> for [u8; 4] {
        fn from(val: EnergyCh3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for EnergyCh3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("EnergyCh3");
            d.field("energy", &self.energy());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EnergyCh3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EnergyCh3 {{ ");
            defmt::write!(f, "energy: {=u32}, ", &self.energy());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for EnergyCh3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for EnergyCh3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for EnergyCh3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for EnergyCh3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for EnergyCh3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for EnergyCh3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for EnergyCh3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Energy channel 4
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EnergyCh4 {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for EnergyCh4 {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl EnergyCh4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `energy` field of the register.
        ///
        /// Accumulated energy in joules, unsigned
        pub fn energy(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::BE>(&self.bits, 0, 32) };
            raw
        }
        ///Write the `energy` field of the register.
        ///
        /// Accumulated energy in joules, unsigned
        pub fn set_energy(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::BE>(raw, 0, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for EnergyCh4 {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<EnergyCh4> for [u8; 4] {
        fn from(val: EnergyCh4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for EnergyCh4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("EnergyCh4");
            d.field("energy", &self.energy());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EnergyCh4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EnergyCh4 {{ ");
            defmt::write!(f, "energy: {=u32}, ", &self.energy());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for EnergyCh4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for EnergyCh4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for EnergyCh4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for EnergyCh4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for EnergyCh4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for EnergyCh4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for EnergyCh4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Flags register
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flags {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for Flags {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Flags {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `limit4_alert` field of the register.
        ///
        /// Alert limit 4 exceeded
        pub fn limit4_alert(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Read the `limit3_alert` field of the register.
        ///
        /// Alert limit 3 exceeded
        pub fn limit3_alert(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `limit2_alert` field of the register.
        ///
        /// Alert limit 2 exceeded
        pub fn limit2_alert(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `limit1_alert` field of the register.
        ///
        /// Alert limit 1 exceeded
        pub fn limit1_alert(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `energyof_ch4` field of the register.
        ///
        /// Energy register overflow channel 4
        pub fn energyof_ch4(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `energyof_ch3` field of the register.
        ///
        /// Energy register overflow channel 3
        pub fn energyof_ch3(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `energyof_ch2` field of the register.
        ///
        /// Energy register overflow channel 2
        pub fn energyof_ch2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `energyof_ch1` field of the register.
        ///
        /// Energy register overflow channel 1
        pub fn energyof_ch1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `cvrf` field of the register.
        ///
        /// Conversion ready flag. Set when all conversions and averaging complete. Cleared by writing CONFIG1 or reading FLAGS.
        pub fn cvrf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `ovf` field of the register.
        ///
        /// Math overflow flag. Indicates current and power data may be invalid.
        pub fn ovf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::BE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Write the `limit4_alert` field of the register.
        ///
        /// Alert limit 4 exceeded
        pub fn set_limit4_alert(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 15, 16, &mut self.bits) };
        }
        ///Write the `limit3_alert` field of the register.
        ///
        /// Alert limit 3 exceeded
        pub fn set_limit3_alert(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `limit2_alert` field of the register.
        ///
        /// Alert limit 2 exceeded
        pub fn set_limit2_alert(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `limit1_alert` field of the register.
        ///
        /// Alert limit 1 exceeded
        pub fn set_limit1_alert(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `energyof_ch4` field of the register.
        ///
        /// Energy register overflow channel 4
        pub fn set_energyof_ch4(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `energyof_ch3` field of the register.
        ///
        /// Energy register overflow channel 3
        pub fn set_energyof_ch3(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `energyof_ch2` field of the register.
        ///
        /// Energy register overflow channel 2
        pub fn set_energyof_ch2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `energyof_ch1` field of the register.
        ///
        /// Energy register overflow channel 1
        pub fn set_energyof_ch1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `cvrf` field of the register.
        ///
        /// Conversion ready flag. Set when all conversions and averaging complete. Cleared by writing CONFIG1 or reading FLAGS.
        pub fn set_cvrf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `ovf` field of the register.
        ///
        /// Math overflow flag. Indicates current and power data may be invalid.
        pub fn set_ovf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::BE>(raw, 6, 7, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for Flags {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<Flags> for [u8; 2] {
        fn from(val: Flags) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Flags {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Flags");
            d.field("limit4_alert", &self.limit4_alert());
            d.field("limit3_alert", &self.limit3_alert());
            d.field("limit2_alert", &self.limit2_alert());
            d.field("limit1_alert", &self.limit1_alert());
            d.field("energyof_ch4", &self.energyof_ch4());
            d.field("energyof_ch3", &self.energyof_ch3());
            d.field("energyof_ch2", &self.energyof_ch2());
            d.field("energyof_ch1", &self.energyof_ch1());
            d.field("cvrf", &self.cvrf());
            d.field("ovf", &self.ovf());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flags {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Flags {{ ");
            defmt::write!(f, "limit4_alert: {=bool}, ", &self.limit4_alert());
            defmt::write!(f, "limit3_alert: {=bool}, ", &self.limit3_alert());
            defmt::write!(f, "limit2_alert: {=bool}, ", &self.limit2_alert());
            defmt::write!(f, "limit1_alert: {=bool}, ", &self.limit1_alert());
            defmt::write!(f, "energyof_ch4: {=bool}, ", &self.energyof_ch4());
            defmt::write!(f, "energyof_ch3: {=bool}, ", &self.energyof_ch3());
            defmt::write!(f, "energyof_ch2: {=bool}, ", &self.energyof_ch2());
            defmt::write!(f, "energyof_ch1: {=bool}, ", &self.energyof_ch1());
            defmt::write!(f, "cvrf: {=bool}, ", &self.cvrf());
            defmt::write!(f, "ovf: {=bool}, ", &self.ovf());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for Flags {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Flags {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Flags {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Flags {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Flags {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Flags {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Flags {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Manufacturer ID. Reads back 0x5449 ('TI' in ASCII)
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ManufacturerId {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ManufacturerId {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ManufacturerId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [84, 73] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `id` field of the register.
        ///
        /// Manufacturer ID
        pub fn id(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::BE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `id` field of the register.
        ///
        /// Manufacturer ID
        pub fn set_id(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::BE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ManufacturerId {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ManufacturerId> for [u8; 2] {
        fn from(val: ManufacturerId) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ManufacturerId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ManufacturerId");
            d.field("id", &self.id());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ManufacturerId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ManufacturerId {{ ");
            defmt::write!(f, "id: {=u16}, ", &self.id());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ManufacturerId {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ManufacturerId {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ManufacturerId {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ManufacturerId {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ManufacturerId {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ManufacturerId {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ManufacturerId {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        /// Configuration register 1
        Config1(Config1),
        /// Configuration register 2
        Config2(Config2),
        /// Calibration register channel 1
        CalibrationCh1(CalibrationCh1),
        /// Calibration register channel 2
        CalibrationCh2(CalibrationCh2),
        /// Calibration register channel 3
        CalibrationCh3(CalibrationCh3),
        /// Calibration register channel 4
        CalibrationCh4(CalibrationCh4),
        /// Alert configuration register 1
        AlertConfig1(AlertConfig1),
        /// Alert configuration register 2
        AlertConfig2(AlertConfig2),
        /// Alert configuration register 3
        AlertConfig3(AlertConfig3),
        /// Alert configuration register 4
        AlertConfig4(AlertConfig4),
        /// Alert limit register 1. Format matches corresponding result register.
        AlertLimit1(AlertLimit1),
        /// Alert limit register 2
        AlertLimit2(AlertLimit2),
        /// Alert limit register 3
        AlertLimit3(AlertLimit3),
        /// Alert limit register 4
        AlertLimit4(AlertLimit4),
        /// Shunt voltage channel 1. 2's complement. LSB = 2.5uV (range=0) or 625nV (range=1)
        ShuntVoltageCh1(ShuntVoltageCh1),
        /// Shunt voltage channel 2
        ShuntVoltageCh2(ShuntVoltageCh2),
        /// Shunt voltage channel 3
        ShuntVoltageCh3(ShuntVoltageCh3),
        /// Shunt voltage channel 4
        ShuntVoltageCh4(ShuntVoltageCh4),
        /// Bus voltage channel 1. Always positive. LSB = 1.6mV
        BusVoltageCh1(BusVoltageCh1),
        /// Bus voltage channel 2
        BusVoltageCh2(BusVoltageCh2),
        /// Bus voltage channel 3
        BusVoltageCh3(BusVoltageCh3),
        /// Bus voltage channel 4
        BusVoltageCh4(BusVoltageCh4),
        /// Current channel 1. Value [A] = CURRENT_LSB x register_value
        CurrentCh1(CurrentCh1),
        /// Current channel 2
        CurrentCh2(CurrentCh2),
        /// Current channel 3
        CurrentCh3(CurrentCh3),
        /// Current channel 4
        CurrentCh4(CurrentCh4),
        /// Power channel 1. Value [W] = 32 x CURRENT_LSB x register_value. Unsigned.
        PowerCh1(PowerCh1),
        /// Power channel 2
        PowerCh2(PowerCh2),
        /// Power channel 3
        PowerCh3(PowerCh3),
        /// Power channel 4
        PowerCh4(PowerCh4),
        /// Energy channel 1. Value [J] = 32 x CURRENT_LSB x register_value. Unsigned.
        EnergyCh1(EnergyCh1),
        /// Energy channel 2
        EnergyCh2(EnergyCh2),
        /// Energy channel 3
        EnergyCh3(EnergyCh3),
        /// Energy channel 4
        EnergyCh4(EnergyCh4),
        /// Flags register
        Flags(Flags),
        /// Manufacturer ID. Reads back 0x5449 ('TI' in ASCII)
        ManufacturerId(ManufacturerId),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Config1(val) => core::fmt::Debug::fmt(val, f),
                Self::Config2(val) => core::fmt::Debug::fmt(val, f),
                Self::CalibrationCh1(val) => core::fmt::Debug::fmt(val, f),
                Self::CalibrationCh2(val) => core::fmt::Debug::fmt(val, f),
                Self::CalibrationCh3(val) => core::fmt::Debug::fmt(val, f),
                Self::CalibrationCh4(val) => core::fmt::Debug::fmt(val, f),
                Self::AlertConfig1(val) => core::fmt::Debug::fmt(val, f),
                Self::AlertConfig2(val) => core::fmt::Debug::fmt(val, f),
                Self::AlertConfig3(val) => core::fmt::Debug::fmt(val, f),
                Self::AlertConfig4(val) => core::fmt::Debug::fmt(val, f),
                Self::AlertLimit1(val) => core::fmt::Debug::fmt(val, f),
                Self::AlertLimit2(val) => core::fmt::Debug::fmt(val, f),
                Self::AlertLimit3(val) => core::fmt::Debug::fmt(val, f),
                Self::AlertLimit4(val) => core::fmt::Debug::fmt(val, f),
                Self::ShuntVoltageCh1(val) => core::fmt::Debug::fmt(val, f),
                Self::ShuntVoltageCh2(val) => core::fmt::Debug::fmt(val, f),
                Self::ShuntVoltageCh3(val) => core::fmt::Debug::fmt(val, f),
                Self::ShuntVoltageCh4(val) => core::fmt::Debug::fmt(val, f),
                Self::BusVoltageCh1(val) => core::fmt::Debug::fmt(val, f),
                Self::BusVoltageCh2(val) => core::fmt::Debug::fmt(val, f),
                Self::BusVoltageCh3(val) => core::fmt::Debug::fmt(val, f),
                Self::BusVoltageCh4(val) => core::fmt::Debug::fmt(val, f),
                Self::CurrentCh1(val) => core::fmt::Debug::fmt(val, f),
                Self::CurrentCh2(val) => core::fmt::Debug::fmt(val, f),
                Self::CurrentCh3(val) => core::fmt::Debug::fmt(val, f),
                Self::CurrentCh4(val) => core::fmt::Debug::fmt(val, f),
                Self::PowerCh1(val) => core::fmt::Debug::fmt(val, f),
                Self::PowerCh2(val) => core::fmt::Debug::fmt(val, f),
                Self::PowerCh3(val) => core::fmt::Debug::fmt(val, f),
                Self::PowerCh4(val) => core::fmt::Debug::fmt(val, f),
                Self::EnergyCh1(val) => core::fmt::Debug::fmt(val, f),
                Self::EnergyCh2(val) => core::fmt::Debug::fmt(val, f),
                Self::EnergyCh3(val) => core::fmt::Debug::fmt(val, f),
                Self::EnergyCh4(val) => core::fmt::Debug::fmt(val, f),
                Self::Flags(val) => core::fmt::Debug::fmt(val, f),
                Self::ManufacturerId(val) => core::fmt::Debug::fmt(val, f),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::Config1(val) => defmt::Format::format(val, f),
                Self::Config2(val) => defmt::Format::format(val, f),
                Self::CalibrationCh1(val) => defmt::Format::format(val, f),
                Self::CalibrationCh2(val) => defmt::Format::format(val, f),
                Self::CalibrationCh3(val) => defmt::Format::format(val, f),
                Self::CalibrationCh4(val) => defmt::Format::format(val, f),
                Self::AlertConfig1(val) => defmt::Format::format(val, f),
                Self::AlertConfig2(val) => defmt::Format::format(val, f),
                Self::AlertConfig3(val) => defmt::Format::format(val, f),
                Self::AlertConfig4(val) => defmt::Format::format(val, f),
                Self::AlertLimit1(val) => defmt::Format::format(val, f),
                Self::AlertLimit2(val) => defmt::Format::format(val, f),
                Self::AlertLimit3(val) => defmt::Format::format(val, f),
                Self::AlertLimit4(val) => defmt::Format::format(val, f),
                Self::ShuntVoltageCh1(val) => defmt::Format::format(val, f),
                Self::ShuntVoltageCh2(val) => defmt::Format::format(val, f),
                Self::ShuntVoltageCh3(val) => defmt::Format::format(val, f),
                Self::ShuntVoltageCh4(val) => defmt::Format::format(val, f),
                Self::BusVoltageCh1(val) => defmt::Format::format(val, f),
                Self::BusVoltageCh2(val) => defmt::Format::format(val, f),
                Self::BusVoltageCh3(val) => defmt::Format::format(val, f),
                Self::BusVoltageCh4(val) => defmt::Format::format(val, f),
                Self::CurrentCh1(val) => defmt::Format::format(val, f),
                Self::CurrentCh2(val) => defmt::Format::format(val, f),
                Self::CurrentCh3(val) => defmt::Format::format(val, f),
                Self::CurrentCh4(val) => defmt::Format::format(val, f),
                Self::PowerCh1(val) => defmt::Format::format(val, f),
                Self::PowerCh2(val) => defmt::Format::format(val, f),
                Self::PowerCh3(val) => defmt::Format::format(val, f),
                Self::PowerCh4(val) => defmt::Format::format(val, f),
                Self::EnergyCh1(val) => defmt::Format::format(val, f),
                Self::EnergyCh2(val) => defmt::Format::format(val, f),
                Self::EnergyCh3(val) => defmt::Format::format(val, f),
                Self::EnergyCh4(val) => defmt::Format::format(val, f),
                Self::Flags(val) => defmt::Format::format(val, f),
                Self::ManufacturerId(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<Config1> for FieldSetValue {
        fn from(val: Config1) -> Self {
            Self::Config1(val)
        }
    }
    impl From<Config2> for FieldSetValue {
        fn from(val: Config2) -> Self {
            Self::Config2(val)
        }
    }
    impl From<CalibrationCh1> for FieldSetValue {
        fn from(val: CalibrationCh1) -> Self {
            Self::CalibrationCh1(val)
        }
    }
    impl From<CalibrationCh2> for FieldSetValue {
        fn from(val: CalibrationCh2) -> Self {
            Self::CalibrationCh2(val)
        }
    }
    impl From<CalibrationCh3> for FieldSetValue {
        fn from(val: CalibrationCh3) -> Self {
            Self::CalibrationCh3(val)
        }
    }
    impl From<CalibrationCh4> for FieldSetValue {
        fn from(val: CalibrationCh4) -> Self {
            Self::CalibrationCh4(val)
        }
    }
    impl From<AlertConfig1> for FieldSetValue {
        fn from(val: AlertConfig1) -> Self {
            Self::AlertConfig1(val)
        }
    }
    impl From<AlertConfig2> for FieldSetValue {
        fn from(val: AlertConfig2) -> Self {
            Self::AlertConfig2(val)
        }
    }
    impl From<AlertConfig3> for FieldSetValue {
        fn from(val: AlertConfig3) -> Self {
            Self::AlertConfig3(val)
        }
    }
    impl From<AlertConfig4> for FieldSetValue {
        fn from(val: AlertConfig4) -> Self {
            Self::AlertConfig4(val)
        }
    }
    impl From<AlertLimit1> for FieldSetValue {
        fn from(val: AlertLimit1) -> Self {
            Self::AlertLimit1(val)
        }
    }
    impl From<AlertLimit2> for FieldSetValue {
        fn from(val: AlertLimit2) -> Self {
            Self::AlertLimit2(val)
        }
    }
    impl From<AlertLimit3> for FieldSetValue {
        fn from(val: AlertLimit3) -> Self {
            Self::AlertLimit3(val)
        }
    }
    impl From<AlertLimit4> for FieldSetValue {
        fn from(val: AlertLimit4) -> Self {
            Self::AlertLimit4(val)
        }
    }
    impl From<ShuntVoltageCh1> for FieldSetValue {
        fn from(val: ShuntVoltageCh1) -> Self {
            Self::ShuntVoltageCh1(val)
        }
    }
    impl From<ShuntVoltageCh2> for FieldSetValue {
        fn from(val: ShuntVoltageCh2) -> Self {
            Self::ShuntVoltageCh2(val)
        }
    }
    impl From<ShuntVoltageCh3> for FieldSetValue {
        fn from(val: ShuntVoltageCh3) -> Self {
            Self::ShuntVoltageCh3(val)
        }
    }
    impl From<ShuntVoltageCh4> for FieldSetValue {
        fn from(val: ShuntVoltageCh4) -> Self {
            Self::ShuntVoltageCh4(val)
        }
    }
    impl From<BusVoltageCh1> for FieldSetValue {
        fn from(val: BusVoltageCh1) -> Self {
            Self::BusVoltageCh1(val)
        }
    }
    impl From<BusVoltageCh2> for FieldSetValue {
        fn from(val: BusVoltageCh2) -> Self {
            Self::BusVoltageCh2(val)
        }
    }
    impl From<BusVoltageCh3> for FieldSetValue {
        fn from(val: BusVoltageCh3) -> Self {
            Self::BusVoltageCh3(val)
        }
    }
    impl From<BusVoltageCh4> for FieldSetValue {
        fn from(val: BusVoltageCh4) -> Self {
            Self::BusVoltageCh4(val)
        }
    }
    impl From<CurrentCh1> for FieldSetValue {
        fn from(val: CurrentCh1) -> Self {
            Self::CurrentCh1(val)
        }
    }
    impl From<CurrentCh2> for FieldSetValue {
        fn from(val: CurrentCh2) -> Self {
            Self::CurrentCh2(val)
        }
    }
    impl From<CurrentCh3> for FieldSetValue {
        fn from(val: CurrentCh3) -> Self {
            Self::CurrentCh3(val)
        }
    }
    impl From<CurrentCh4> for FieldSetValue {
        fn from(val: CurrentCh4) -> Self {
            Self::CurrentCh4(val)
        }
    }
    impl From<PowerCh1> for FieldSetValue {
        fn from(val: PowerCh1) -> Self {
            Self::PowerCh1(val)
        }
    }
    impl From<PowerCh2> for FieldSetValue {
        fn from(val: PowerCh2) -> Self {
            Self::PowerCh2(val)
        }
    }
    impl From<PowerCh3> for FieldSetValue {
        fn from(val: PowerCh3) -> Self {
            Self::PowerCh3(val)
        }
    }
    impl From<PowerCh4> for FieldSetValue {
        fn from(val: PowerCh4) -> Self {
            Self::PowerCh4(val)
        }
    }
    impl From<EnergyCh1> for FieldSetValue {
        fn from(val: EnergyCh1) -> Self {
            Self::EnergyCh1(val)
        }
    }
    impl From<EnergyCh2> for FieldSetValue {
        fn from(val: EnergyCh2) -> Self {
            Self::EnergyCh2(val)
        }
    }
    impl From<EnergyCh3> for FieldSetValue {
        fn from(val: EnergyCh3) -> Self {
            Self::EnergyCh3(val)
        }
    }
    impl From<EnergyCh4> for FieldSetValue {
        fn from(val: EnergyCh4) -> Self {
            Self::EnergyCh4(val)
        }
    }
    impl From<Flags> for FieldSetValue {
        fn from(val: Flags) -> Self {
            Self::Flags(val)
        }
    }
    impl From<ManufacturerId> for FieldSetValue {
        fn from(val: ManufacturerId) -> Self {
            Self::ManufacturerId(val)
        }
    }
}
/// Averaging count
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Averaging {
    _1 = 0,
    _4 = 1,
    _16 = 2,
    _64 = 3,
    _128 = 4,
    _256 = 5,
    _512 = 6,
    _1024 = 7,
}
impl core::convert::TryFrom<u8> for Averaging {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::_1),
            1 => Ok(Self::_4),
            2 => Ok(Self::_16),
            3 => Ok(Self::_64),
            4 => Ok(Self::_128),
            5 => Ok(Self::_256),
            6 => Ok(Self::_512),
            7 => Ok(Self::_1024),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Averaging",
            }),
        }
    }
}
impl From<Averaging> for u8 {
    fn from(val: Averaging) -> Self {
        match val {
            Averaging::_1 => 0,
            Averaging::_4 => 1,
            Averaging::_16 => 2,
            Averaging::_64 => 3,
            Averaging::_128 => 4,
            Averaging::_256 => 5,
            Averaging::_512 => 6,
            Averaging::_1024 => 7,
        }
    }
}
/// Bus voltage conversion time
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusConversionTime {
    _140Us = 0,
    _204Us = 1,
    _332Us = 2,
    _588Us = 3,
    _1100Us = 4,
    _2116Us = 5,
    _4156Us = 6,
    _8244Us = 7,
}
impl core::convert::TryFrom<u8> for BusConversionTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::_140Us),
            1 => Ok(Self::_204Us),
            2 => Ok(Self::_332Us),
            3 => Ok(Self::_588Us),
            4 => Ok(Self::_1100Us),
            5 => Ok(Self::_2116Us),
            6 => Ok(Self::_4156Us),
            7 => Ok(Self::_8244Us),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "BusConversionTime",
            }),
        }
    }
}
impl From<BusConversionTime> for u8 {
    fn from(val: BusConversionTime) -> Self {
        match val {
            BusConversionTime::_140Us => 0,
            BusConversionTime::_204Us => 1,
            BusConversionTime::_332Us => 2,
            BusConversionTime::_588Us => 3,
            BusConversionTime::_1100Us => 4,
            BusConversionTime::_2116Us => 5,
            BusConversionTime::_4156Us => 6,
            BusConversionTime::_8244Us => 7,
        }
    }
}
/// Shunt voltage conversion time
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShuntConversionTime {
    _140Us = 0,
    _204Us = 1,
    _332Us = 2,
    _588Us = 3,
    _1100Us = 4,
    _2116Us = 5,
    _4156Us = 6,
    _8244Us = 7,
}
impl core::convert::TryFrom<u8> for ShuntConversionTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::_140Us),
            1 => Ok(Self::_204Us),
            2 => Ok(Self::_332Us),
            3 => Ok(Self::_588Us),
            4 => Ok(Self::_1100Us),
            5 => Ok(Self::_2116Us),
            6 => Ok(Self::_4156Us),
            7 => Ok(Self::_8244Us),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ShuntConversionTime",
            }),
        }
    }
}
impl From<ShuntConversionTime> for u8 {
    fn from(val: ShuntConversionTime) -> Self {
        match val {
            ShuntConversionTime::_140Us => 0,
            ShuntConversionTime::_204Us => 1,
            ShuntConversionTime::_332Us => 2,
            ShuntConversionTime::_588Us => 3,
            ShuntConversionTime::_1100Us => 4,
            ShuntConversionTime::_2116Us => 5,
            ShuntConversionTime::_4156Us => 6,
            ShuntConversionTime::_8244Us => 7,
        }
    }
}
/// Operating mode
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    Shutdown = 0,
    ShuntTriggered = 1,
    BusTriggered = 2,
    ShuntAndBusTriggered = 3,
    Shutdown2 = 4,
    ContinuousShunt = 5,
    ContinuousBus = 6,
    ContinuousShuntAndBus = 7,
}
impl core::convert::TryFrom<u8> for Mode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Shutdown),
            1 => Ok(Self::ShuntTriggered),
            2 => Ok(Self::BusTriggered),
            3 => Ok(Self::ShuntAndBusTriggered),
            4 => Ok(Self::Shutdown2),
            5 => Ok(Self::ContinuousShunt),
            6 => Ok(Self::ContinuousBus),
            7 => Ok(Self::ContinuousShuntAndBus),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Mode",
            }),
        }
    }
}
impl From<Mode> for u8 {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Shutdown => 0,
            Mode::ShuntTriggered => 1,
            Mode::BusTriggered => 2,
            Mode::ShuntAndBusTriggered => 3,
            Mode::Shutdown2 => 4,
            Mode::ContinuousShunt => 5,
            Mode::ContinuousBus => 6,
            Mode::ContinuousShuntAndBus => 7,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlertPolarity {
    ActiveLow = 0,
    ActiveHigh = 1,
}
impl core::convert::TryFrom<u8> for AlertPolarity {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ActiveLow),
            1 => Ok(Self::ActiveHigh),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AlertPolarity",
            }),
        }
    }
}
impl From<AlertPolarity> for u8 {
    fn from(val: AlertPolarity) -> Self {
        match val {
            AlertPolarity::ActiveLow => 0,
            AlertPolarity::ActiveHigh => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlertChannel {
    Ch1 = 0,
    Ch2 = 1,
    Ch3 = 2,
    Ch4 = 3,
}
impl core::convert::TryFrom<u8> for AlertChannel {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ch1),
            1 => Ok(Self::Ch2),
            2 => Ok(Self::Ch3),
            3 => Ok(Self::Ch4),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AlertChannel",
            }),
        }
    }
}
impl From<AlertChannel> for u8 {
    fn from(val: AlertChannel) -> Self {
        match val {
            AlertChannel::Ch1 => 0,
            AlertChannel::Ch2 => 1,
            AlertChannel::Ch3 => 2,
            AlertChannel::Ch4 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlertFunction {
    None = 0,
    ShuntOverLimit = 1,
    ShuntUnderLimit = 2,
    BusOverLimit = 3,
    BusUnderLimit = 4,
    PowerOverLimit = 5,
    Reserved6 = 6,
    Reserved7 = 7,
}
impl core::convert::TryFrom<u8> for AlertFunction {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::None),
            1 => Ok(Self::ShuntOverLimit),
            2 => Ok(Self::ShuntUnderLimit),
            3 => Ok(Self::BusOverLimit),
            4 => Ok(Self::BusUnderLimit),
            5 => Ok(Self::PowerOverLimit),
            6 => Ok(Self::Reserved6),
            7 => Ok(Self::Reserved7),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AlertFunction",
            }),
        }
    }
}
impl From<AlertFunction> for u8 {
    fn from(val: AlertFunction) -> Self {
        match val {
            AlertFunction::None => 0,
            AlertFunction::ShuntOverLimit => 1,
            AlertFunction::ShuntUnderLimit => 2,
            AlertFunction::BusOverLimit => 3,
            AlertFunction::BusUnderLimit => 4,
            AlertFunction::PowerOverLimit => 5,
            AlertFunction::Reserved6 => 6,
            AlertFunction::Reserved7 => 7,
        }
    }
}
