#![no_std]
#![no_main]

pub use lsm6dsox_driver::Lsm6dsox;
//use cortex_m_semihosting::{hprint, hprintln};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm, peripheral::{self, dwt, dcb, DWT}};
use cortex_m_rt::entry;

use core::{cell::{Cell, RefCell}, convert::TryInto, f32::consts::PI};
use stm32f4xx_hal::{
    i2c::{self, I2c1, Mode},
    pac::{self, I2C1},
    prelude::*, gpio::alt::i2c1, dwt::{MonoTimer},
};
use rtt_target::{rprintln, rprint, rtt_init_print};
use libm::{atanf, sqrtf};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.hclk(8.MHz()).freeze();

    let gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_open_drain_output();
    let sda = gpiob.pb7.into_open_drain_output();
 
    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard { frequency: 200.kHz() },
        &clocks,
    );

    let imu = Lsm6dsox::new(&mut i2c).unwrap();
    
    let id = imu.read_id(&mut i2c).unwrap();
    rprintln!("id is {:#b}: ", id);

    imu.configure_accel(&mut i2c).unwrap();
    imu.configure_gyro(&mut i2c).unwrap();
    
    let mut accel_data:[f32; 3] = [0.0, 0.0, 0.0];
    let mut gyro_data:[f32; 3] = [0.0, 0.0, 0.0];

    let mono_timer = MonoTimer::new(cp.DWT, cp.DCB, &clocks);
    let start_time = mono_timer.now();
    let mut prev_time:u32 = 0;

    let mut x_gyro: f32 = 0.0;
    let mut y_gyro: f32 = 0.0;
    let mut z_gyro: f32 = 0.0;

    let mut x_accel: f32 = 0.0;
    let mut y_accel: f32 = 0.0;

    loop {
        let time = start_time.elapsed();
        let delta_time = time - prev_time;
        let delta_sec = delta_time as f32/ clocks.sysclk().raw() as f32 * 2.0;

        accel_data = imu.read_accel(&mut i2c).unwrap();
        gyro_data = imu.read_gyro(&mut i2c).unwrap();
        //rprintln!("Time: {:?}", delta_sec);
        //rprintln!("Accelerometer: {:?}", accel_data);
        //rprintln!("Angular: {:?}", gyro_data);

        x_gyro += delta_sec * gyro_data[0];
        y_gyro += delta_sec * gyro_data[1];
        z_gyro += delta_sec * gyro_data[2];

        y_accel = atanf(accel_data[0] / sqrtf(accel_data[1] * accel_data[1] + accel_data[2] * accel_data[2]) ) * 180.0 / PI;
        x_accel = atanf(accel_data[1] / sqrtf(accel_data[0] * accel_data[0] + accel_data[2] * accel_data[2]) ) * 180.0 / PI;


        rprintln!("Gyroscope angles; x: {:?}, y: {:?}, z: {:?} ", x_gyro, y_gyro, z_gyro);
        rprintln!("Accelerom angles: x: {:?}, y: {:?} ", x_accel, y_accel);

        prev_time = time;

    }

}


