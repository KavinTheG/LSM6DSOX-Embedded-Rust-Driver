#![allow(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

use core::marker::PhantomData;


// Slave address 
const SLAVE_ADDRESS: u8 = 0x6A; // LSB is 1 if SDO/SA0 is connect to usplly voltage, 0 otherwise

// WHO AM I Register
const WHO_AM_I: u8 = 0x0F;

// 0th bit: Temp Data available (1 if available)
// 1st bit: Gyro data availability
// 2nd bit: Accel data available
const STATUS_REG: u8 = 0b0001_1110;

// ANGULAR RATE OUT REGISTERS
// The following are addresses for 8 bit registers
// That are concantanted for a 16bit output value
// Ex: OUTX_H_G + OUTX_L_G

// Gyro Configuration Register
const CTRL2_G: u8 = 0x11;

// Angular rate pitch axis (X) output
const OUTX_L_G: u8 = 0x22;
const OUTX_H_G: u8 = 0x23;

// Angular rate roll axis (Y) output
const OUTY_L_G: u8 = 0x24;
const OUTY_H_G: u8 = 0x25;

// Angular rate yaw axis (Z) output
const OUTZ_L_G: u8 = 0x26;
const OUTZ_H_G: u8 = 0x27;

// ACCELEROMETER CONFIG REG
const CTRL1_XL: u8 = 0x10;


// ACCELEROMETER RATE OUT REGISTERS

// Accel rate X axis (X) output
const OUTX_L_A: u8 = 0x28;
const OUTX_H_A: u8 = 0x29;

// Accel rate Y axis (Y) output
const OUTY_L_A: u8 = 0x2A;
const OUTY_H_A: u8 = 0x2B;

// Accel rate Z axis (Z) output
const OUTZ_L_A: u8 = 0x2C;
const OUTZ_H_A: u8 = 0x2D;


// lsm6dsox driver
pub struct Lsm6dsox<I2C> {
    i2c: PhantomData<I2C>,
}

impl<I2C, E> Lsm6dsox<I2C> 
    where 
        I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub fn new(_i2c: &I2C) -> Result<Self, E> {
        let lsm6dsox = Lsm6dsox {
            i2c: PhantomData,
        };

        Ok(lsm6dsox)
    }

    pub fn read_id(&self, i2c: &mut I2C) -> Result<u8, E> {

        let mut buffer = [0u8, 1];
        match i2c.write_read(SLAVE_ADDRESS, &[WHO_AM_I], &mut buffer) {
            Ok(_) => Ok(buffer[0]),
            Err(e) => Err(e),
        }
    }

    // Configures Accelerometer
    pub fn configure_accel(&self, i2c: &mut I2C) -> Result<(), E>{
        // ODR_XL[7:4] = 0100; sets accelerometer to work at 104 Hz
        // FS[3:2] = 10; sets accelerometer full-scale selection to 4g
        // LPF2_XL_EN[1] = 1; output from first stage digital filtering
        // 0100 10 1 0
        let configuration: u8 = 0x4A;
        i2c.write(SLAVE_ADDRESS, &[CTRL1_XL, configuration]);
        match i2c.write(SLAVE_ADDRESS, &[CTRL1_XL, configuration]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // Configures Gyroscope
    pub fn configure_gyro(&self, i2c: &mut I2C) -> Result<(), E> {
        // ODR_G[3:0] = 0100; sets gyroscope to work at 104 Hz
        // FS[1:0] = 11; sets full-scale selcetion to 2000dps
        // FS_125 = 0;
        let configuration: u8 = 0x4A;
        i2c.write(SLAVE_ADDRESS, &[CTRL2_G, configuration]);
        match i2c.write(SLAVE_ADDRESS, &[CTRL2_G, configuration]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // Read Accelerometer Data
    pub fn read_accel(&self, i2c: &mut I2C) -> Result<[f32; 3], E> {

        let mut accel_data:[f32; 3] = [0.0, 0.0, 0.0];
        let mut buffer: [u8; 2] = [0; 2];
        let mut word: i16;

        let _ = i2c.write_read(SLAVE_ADDRESS, &[OUTX_H_A], &mut buffer);
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTX_L_A], &mut buffer);
        word |= buffer[0] as i16;

        accel_data[0] = (word as f32) * 4.0/ 32768.0;

        i2c.write_read(SLAVE_ADDRESS, &[OUTY_H_A], &mut buffer);
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTY_L_A], &mut buffer);
        word |= buffer[0] as i16;

        accel_data[1] = (word as f32) * 4.0/ 32768.0;

        i2c.write_read(SLAVE_ADDRESS, &[OUTZ_H_A], &mut buffer);
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTZ_L_A], &mut buffer);
        word |= buffer[0] as i16;

        accel_data[2] = (word as f32) * 4.0/ 32768.0;

        Ok(accel_data)
        
    }

    // Read Gyroscope Data
    pub fn read_gyro(&self, i2c: &mut I2C) -> Result<[f32; 3], E> {

        let mut gyro_data:[f32; 3] = [0.0, 0.0, 0.0];
        let mut buffer: [u8; 2] = [0; 2];
        let mut word: i16;

        i2c.write_read(SLAVE_ADDRESS, &[OUTX_H_A], &mut buffer);
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTX_L_A], &mut buffer);
        word |= buffer[0] as i16;

        gyro_data[0] = (word as f32) * 4.0/ 32768.0;

        i2c.write_read(SLAVE_ADDRESS, &[OUTY_H_A], &mut buffer);
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTY_L_A], &mut buffer);
        word |= buffer[0] as i16;

        gyro_data[1] = (word as f32) * 4.0/ 32768.0;

        i2c.write_read(SLAVE_ADDRESS, &[OUTZ_H_A], &mut buffer);
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTZ_L_A], &mut buffer);
        word |= buffer[0] as i16;

        gyro_data[2] = (word as f32) * 4.0/ 32768.0;

        Ok(gyro_data)
    }

}
