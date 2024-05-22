#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Register {
    // ACCELEROMETER CONFIG REG
    CTRL1_XL = 0x10,

    // Accel rate X axis (X) output
    OUTX_L_A = 0x28,
    OUTX_H_A = 0x29,

    // Accel rate Y axis (Y) output
    OUTY_L_A = 0x2A,
    OUTY_H_A = 0x2B,

    // Accel rate Z axis (Z) output
    OUTZ_L_A = 0x2C,
    OUTZ_H_A = 0x2D,
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}