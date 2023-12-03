# LSM6DSOX Rust Driver

<div align="center">
    <img src="https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324"/> 
    <img src="https://img.shields.io/badge/license-MIT-blue" (https://github.co/KavinTheG/LSM6DSOX-Embedded-Rust-Driver/blob/main/LICENSE-MIT) />
</div>

This is a rust driver to retrieve accelerometer + gyroscope data from the LSM6DSOX 6-axis IMU using [`embedded-hal`](https://github.com/japaric/embedded-hal) traits. 

Tested on the STM32F411E Discovery Board.

<br />

## Status
- [x] I2C Support 
    - [x] Basic sensor readings
    - [ ] Calibration
- [ ] SPI Support
- [ ] Documentation

<br />

### Installation 

Ensure the line shown below is added to the Cargo.toml file

```
[dependencies]

    ...

lsm6dsox_driver = {git = "https://github.com/KavinTheG/LSM6DSOX-Embedded-Rust-Driver.git"} \
```

<br />

### Usage

Shown below is a sample code to retrieve acceloremeter data

```
...

use lsm6dsox_driver::Lsm6dsox;

...

fn main() -> ! {

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

    // Instantiate Lsm6dsox imu object
    let imu = Lsm6dsox::new(&mut i2c).unwrap();

    // variable to store data
    let mut accel_data:[f32; 3] = [0.0, 0.0, 0.0];


    ...

    loop {

        // read acceleration data
        accel_data = imu.read_accel(&mut i2c).unwrap();

        ...

    }

}

```

### Datasheet

https://www.st.com/resource/en/datasheet/lsm6dsox.pdf
