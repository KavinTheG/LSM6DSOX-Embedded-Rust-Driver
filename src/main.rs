#![no_std]
#![no_main]
#![allow(unused_imports)]


use cortex_m_semihosting::{hprint, hprintln};
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
    i2c::Mode,
    pac::{self},
    prelude::*,
};

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

// Angular rate pitch axis (X) output
const OUTX_L_G: u8 = 0b0010_0010;
const OUTX_H_G: u8 = 0b0010_0011;

// Angular rate roll axis (Y) output
const OUTY_L_G: u8 = 0b0010_0100;
const OUTY_H_G: u8 = 0b0010_0101;

// Angular rate yaw axis (Z) output
const OUTZ_L_G: u8 = 0b0010_0110;
const OUTZ_H_G: u8 = 0b0010_0111;

// ACCELEROMETER CONFIG REG
const CTRL1_XL: u8 = 0x10;


// ACCELEROMETER RATE OUT REGISTERS

// Accel rate X axis (X) output
const OUTX_L_A: u8 = 0b0010_1000;
const OUTX_H_A: u8 = 0b0010_1001;

// Accel rate Y axis (Y) output
const OUTY_L_A: u8 = 0b0010_1010;
const OUTY_H_A: u8 = 0b0010_1011;

// Accel rate Z axis (Z) output
const OUTZ_L_A: u8 = 0b0010_1100;
const OUTZ_H_A: u8 = 0b0010_1101;


#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.hclk(8.MHz()).freeze();

    let gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_push_pull_output();
    let sda = gpiob.pb7;
 
    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        100.kHz(),
        &clocks,
    );

    let mut buffer: [u8; 2] = [0; 2];


    match i2c.write_read(SLAVE_ADDRESS, &[WHO_AM_I], &mut buffer) {
        Ok(_) => hprintln!("The chip's id is: {:#b}", buffer[0]).unwrap(),
        Err(_) => hprintln!("Failed to read").unwrap(),
    }

    /* 
    Program the peripheral input clock in I2C_CR2 Register in order to generate correct timings
    • Configure the clock control registers
    • Configure the rise time register
    • Program the I2C_CR1 register to enable the peripheral
    • Set the START bit in the I2C_CR1 register to generate a Start condition
    */

    // Configures Accelerometer
    // ODR_XL[7:4] = 0100; sets accelerometer to work at 104 Hz
    // FS[3:2] = 10; sets accelerometer full-scale selection to 4g
    // LPF2_XL_EN = output from first stage digital filtering
    // 0100 11 0 0
    i2c.write(CTRL1_XL, &[0x4C]).unwrap(); 

    //let mut accel_buffer = [0u8; 1]; 

    loop {


    }
}
