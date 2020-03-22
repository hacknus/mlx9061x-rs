use crate::{
    ic,
    register_access::mlx90615::{Register},
    Error, Mlx9061x, SlaveAddr,
};
use core::marker::PhantomData;
use embedded_hal::blocking::{delay, i2c};

impl<E, D, I2C> Mlx9061x<I2C, D, ic::Mlx90615>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
    D: delay::DelayMs<u8>,
{
    /// Create new instance of the MLX90615 device.
    ///
    /// The slave address must match the address stored in the device EEPROM.
    /// To change it you need to connect first and then change it with `set_address()`.
    /// An invalid alternative slave address will return `Error::InvalidInputData`.
    pub fn new_mlx90615(
        i2c: I2C,
        address: SlaveAddr,
        delay_ms: D,
        eeprom_write_delay_ms: u8,
    ) -> Result<Self, Error<E>> {
        let address = Self::get_address(address)?;
        Ok(Mlx9061x {
            i2c,
            eeprom_write_delay_ms,
            delay_ms,
            address,
            _ic: PhantomData,
        })
    }
}

impl<E, D, I2C> Mlx9061x<I2C, D, ic::Mlx90615>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
    D: delay::DelayMs<u8>,
{
    /// Read the ambient temperature in celsius degrees
    pub fn ambient_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TA)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the object temperature in celsius degrees
    pub fn object_temperature(&mut self) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TOBJ)?;
        let t = f32::from(t) * 0.02 - 273.15;
        Ok(t)
    }

    /// Read the raw IR data
    pub fn raw_ir(&mut self) -> Result<u16, Error<E>> {
        self.read_u16(Register::RAW_IR)
    }
}