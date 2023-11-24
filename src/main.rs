#![no_std]
#![no_main]
#![allow(unused_imports)]


//use cortex_m_semihosting::{hprint, hprintln};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm, peripheral};
use cortex_m_rt::entry;
use generic_array::{ArrayLength, GenericArray};
use core::cell::{Cell, RefCell};
use stm32f4xx_hal::{
    i2c::{self},
    pac::{self, I2C1},
    prelude::*, gpio::alt::i2c1,
};
use rtt_target::{rprintln, rprint, rtt_init_print};


#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.hclk(8.MHz()).freeze();

    let gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_push_pull_output();
    let sda = gpiob.pb7;
 
    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        400.kHz(),
        &clocks,
    );

    let mut buffer: [u8; 2] = [0; 2];
    let mut word: i16;

    /* 
    match i2c.write_read(SLAVE_ADDRESS, &[WHO_AM_I], &mut buffer) {
        Ok(_) => rprintln!("The chip's id is: {:#b}", buffer[0]),
        Err(_) => rprintln!("Failed to read"),
    }*/

    /* 
    Program the peripheral input clock in I2C_CR2 Register in order to generate correct timings
    • Configure the clock control registers
    • Configure the rise time register
    • Program the I2C_CR1 register to enable the peripheral
    • Set the START bit in the I2C_CR1 register to generate a Start condition
    */
    /* 
    // Configures gyroscope
    i2c.write(SLAVE_ADDRESS, &[CTRL2_G, 0x4C]).unwrap(); 

    // Configures Accelerometer
    // ODR_XL[7:4] = 0100; sets accelerometer to work at 104 Hz
    // FS[3:2] = 10; sets accelerometer full-scale selection to 4g
    // LPF2_XL_EN[1] = 1; output from first stage digital filtering
    // 0100 10 1 0
    i2c.write(SLAVE_ADDRESS, &[CTRL1_XL, 0x4A]).unwrap(); 
    */
    let mut accel_data:[f32; 3] = [0.0, 0.0, 0.0];
    let mut gyro_data:[f32; 3] = [0.0, 0.0, 0.0];

    loop {


        /*
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTX_H_A], &mut buffer).unwrap();
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTX_L_A], &mut buffer).unwrap();
        word |= buffer[0] as i16;

        accel_data[0] = (word as f32) * 4.0/ 32768.0;

        i2c.write_read(SLAVE_ADDRESS, &[OUTY_H_A], &mut buffer).unwrap();
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTY_L_A], &mut buffer).unwrap();
        word |= buffer[0] as i16;

        accel_data[1] = (word as f32) * 4.0/ 32768.0;

        i2c.write_read(SLAVE_ADDRESS, &[OUTZ_H_A], &mut buffer).unwrap();
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTZ_L_A], &mut buffer).unwrap();
        word |= buffer[0] as i16;

        accel_data[2] = (word as f32) * 4.0/ 32768.0;

        // rprint!("X: {}", accel_data[0]);
        // rprint!(", Y: {}", accel_data[1]);
        // rprintln!(", Z: {}", accel_data[2]);

        i2c.write_read(SLAVE_ADDRESS, &[OUTX_H_G], &mut buffer).unwrap();
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTX_L_G], &mut buffer).unwrap();
        word |= buffer[0] as i16;

        gyro_data[0] = (word as f32) * 2000.0/ 32768.0;

        i2c.write_read(SLAVE_ADDRESS, &[OUTY_H_G], &mut buffer).unwrap();
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTY_L_G], &mut buffer).unwrap();
        word |= buffer[0] as i16;

        gyro_data[1] = (word as f32) * 2000.0/ 32768.0;

        i2c.write_read(SLAVE_ADDRESS, &[OUTZ_H_G], &mut buffer).unwrap();
        word = (buffer[0]  as i16) << 8;
    
        i2c.write_read(SLAVE_ADDRESS, &[OUTZ_L_G], &mut buffer).unwrap();
        word |= buffer[0] as i16;

        gyro_data[2] = (word as f32) * 2000.0/ 32768.0;

        // rprint!("X: {}", gyro_data[0]);
        // rprint!(", Y: {}", gyro_data[1]);
        // rprintln!(", Z: {}", gyro_data[2]);
        */

    }

 


}


