#![no_std]
#![no_main]

pub use lsm6dsox_driver::Lsm6dsox;
//use cortex_m_semihosting::{hprint, hprintln};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm, peripheral};
use cortex_m_rt::entry;
use core::cell::{Cell, RefCell};
use stm32f4xx_hal::{
    i2c::{self, I2c1},
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
        100.kHz(),
        &clocks,
    );

    let imu = Lsm6dsox::new(&mut i2c).unwrap();
    
    let id = imu.read_id(&mut i2c).unwrap();
    rprintln!("id is {:#b}: ", id);

    imu.configure_accel(&mut i2c).unwrap();
    imu.configure_gyro(&mut i2c).unwrap();
    
    let mut accel_data:[f32; 3] = [0.0, 0.0, 0.0];
    let mut gyro_data:[f32; 3] = [0.0, 0.0, 0.0];

    loop {

        accel_data = imu.read_accel(&mut i2c).unwrap();
        gyro_data = imu.read_gyro(&mut i2c).unwrap();

        rprintln!("Acceleration: {:?}", accel_data);
        rprintln!("Angular: {:?}", gyro_data);

    }

}


