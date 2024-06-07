#![allow(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};
use core::mem::MaybeUninit;
use generic_array::GenericArray;
use generic_array::typenum::consts::U6;

// Slave address 
const SLAVE_ADDRESS: u8 = 0x6A; // LSB is 1 if SDO/SA0 is connect to usplly voltage, 0 otherwise

// WHO AM I Register
const WHO_AM_I: u8 = 0x0F;

mod accel;
mod gyro;

// lsm6dsox driver
pub struct Lsm6dsox<I2C> {
    i2c: I2C,
}

impl<I2C, E> Lsm6dsox<I2C> 
    where 
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {       
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let lsm6dsox = Lsm6dsox { i2c };
        Ok(lsm6dsox)
    }

    pub fn read_id(&mut self) -> Result<u8, E> {

        let mut buffer = [0u8, 1];
        self.i2c.write_read(SLAVE_ADDRESS, &[WHO_AM_I], &mut buffer)?;

        Ok(buffer[0])
    }

    // Configures Accelerometer
    pub fn configure_accel(&mut self) -> Result<(), E>{
        // ODR_XL[7:4] = 0100; sets accelerometer to work at 104 Hz
        // FS[3:2] = 10; sets accelerometer full-scale selection to 4g
        // LPF2_XL_EN[1] = 1; output from first stage digital filtering
        // 0100 10 1 0
        let configuration: u8 = 0x48;
        self.i2c.write(SLAVE_ADDRESS, &[accel::Register::CTRL1_XL.addr(), configuration])?;
        Ok(())
    }

    // Configures Gyroscope
    pub fn configure_gyro(&mut self) -> Result<(), E> {
        // ODR_G[3:0] = 0110; sets gyroscope to work at 416 Hz
        // FS[1:0] = 11; sets full-scale selcetion to 2000dps
        // FS_125 = 0;
        let configuration: u8 = 0x4C;
        self.i2c.write(SLAVE_ADDRESS, &[gyro::Register::CTRL2_G.addr(), configuration])?;
        Ok(())
    }

    // Read Accelerometer Data
    pub fn read_accel(&mut self) -> Result<[f32; 3], E> {
        
        let mut accel_data: [f32; 3] = [0.0, 0.0, 0.0];
        let mut buffer: GenericArray<u8, U6> = unsafe { MaybeUninit::uninit().assume_init() };

        { 
            let buffer: &mut [u8] = &mut buffer;
            self.i2c.write_read(SLAVE_ADDRESS, &[accel::Register::OUTX_L_A.addr()], buffer)?;
            accel_data[0] = ((buffer[1] as i16) << 8 | (buffer[0] as i16)) as f32 * 4.0 / 32768.0;
            accel_data[1] = ((buffer[3] as i16) << 8 | (buffer[2] as i16)) as f32 * 4.0 / 32768.0;
            accel_data[2] = ((buffer[5] as i16) << 8 | (buffer[4] as i16)) as f32 * 4.0 / 32768.0;
        }
        Ok(accel_data)
    }


    // Read Gyroscope Data
    pub fn read_gyro(&mut self) -> Result<[f32; 3], E> {

        let mut gyro_data:[f32; 3] = [0.0, 0.0, 0.0];
        let mut buffer: GenericArray<u8, U6> = unsafe { MaybeUninit::uninit().assume_init() };

        {
            let buffer: &mut [u8] = &mut buffer;
            self.i2c.write_read(SLAVE_ADDRESS, &[gyro::Register::OUTX_L_G.addr()], buffer)?;
            gyro_data[0] = ((buffer[1] as i16) << 8 | (buffer[0] as i16)) as f32 * 2000.0 / 32768.0;
            gyro_data[1] = ((buffer[3] as i16) << 8 | (buffer[2] as i16)) as f32 * 2000.0 / 32768.0;
            gyro_data[2] = ((buffer[5] as i16) << 8 | (buffer[4] as i16)) as f32 * 2000.0 / 32768.0;
        }
        Ok(gyro_data)
    }
}
