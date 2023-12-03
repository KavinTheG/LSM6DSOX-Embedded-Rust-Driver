# LSM6DSOX Rust Driver

<div align="center">
    <img src="https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324"/> 
    <img src="https://img.shields.io/badge/license-MIT-blue" (https://github.co/KavinTheG/LSM6DSOX-Embedded-Rust-Driver/blob/main/LICENSE-MIT) />
</div>

This is a rust driver to retrieve accelerometer + gyroscope data from the LSM6DSOX 6-axis IMU using [`embedded-hal`](https://github.com/japaric/embedded-hal) traits. 

Tested on the STM32F411E Discovery Board.

### Installation 

Ensure the line shown below is added to the Cargo.toml file

```
[dependencies]
    ...
lsm6dsox_driver = {git = "https://github.com/KavinTheG/LSM6DSOX-Embedded-Rust-Driver.git"} \
```

## Status
- [x] I2C Support 
    - [x] Basic sensor readings
    - [ ] Calibration
- [ ] SPI Support
- [ ] Documentation
