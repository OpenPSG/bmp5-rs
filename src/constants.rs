// Register addresses
pub(crate) const BMP5_REG_CHIP_ID: u8 = 0x01;
pub(crate) const BMP5_REG_REV_ID: u8 = 0x02;
pub(crate) const BMP5_REG_CHIP_STATUS: u8 = 0x11;
pub(crate) const BMP5_REG_DRIVE_CONFIG: u8 = 0x13;
pub(crate) const BMP5_REG_INT_CONFIG: u8 = 0x14;
pub(crate) const BMP5_REG_INT_SOURCE: u8 = 0x15;
pub(crate) const BMP5_REG_FIFO_CONFIG: u8 = 0x16;
pub(crate) const BMP5_REG_FIFO_COUNT: u8 = 0x17;
pub(crate) const BMP5_REG_FIFO_SEL: u8 = 0x18;
pub(crate) const BMP5_REG_TEMP_DATA_XLSB: u8 = 0x1D;
pub(crate) const BMP5_REG_TEMP_DATA_LSB: u8 = 0x1E;
pub(crate) const BMP5_REG_TEMP_DATA_MSB: u8 = 0x1F;
pub(crate) const BMP5_REG_PRESS_DATA_XLSB: u8 = 0x20;
pub(crate) const BMP5_REG_PRESS_DATA_LSB: u8 = 0x21;
pub(crate) const BMP5_REG_PRESS_DATA_MSB: u8 = 0x22;
pub(crate) const BMP5_REG_INT_STATUS: u8 = 0x27;
pub(crate) const BMP5_REG_STATUS: u8 = 0x28;
pub(crate) const BMP5_REG_FIFO_DATA: u8 = 0x29;
pub(crate) const BMP5_REG_NVM_ADDR: u8 = 0x2B;
pub(crate) const BMP5_REG_NVM_DATA_LSB: u8 = 0x2C;
pub(crate) const BMP5_REG_NVM_DATA_MSB: u8 = 0x2D;
pub(crate) const BMP5_REG_DSP_CONFIG: u8 = 0x30;
pub(crate) const BMP5_REG_DSP_IIR: u8 = 0x31;
pub(crate) const BMP5_REG_OOR_THR_P_LSB: u8 = 0x32;
pub(crate) const BMP5_REG_OOR_THR_P_MSB: u8 = 0x33;
pub(crate) const BMP5_REG_OOR_RANGE: u8 = 0x34;
pub(crate) const BMP5_REG_OOR_CONFIG: u8 = 0x35;
pub(crate) const BMP5_REG_OSR_CONFIG: u8 = 0x36;
pub(crate) const BMP5_REG_ODR_CONFIG: u8 = 0x37;
pub(crate) const BMP5_REG_OSR_EFF: u8 = 0x38;
pub(crate) const BMP5_REG_CMD: u8 = 0x7E;

// Commands
pub(crate) const BMP5_CMD_NOOP: u8 = 0x00;
pub(crate) const BMP5_CMD_EXTMODE_SEQ_0: u8 = 0x73;
pub(crate) const BMP5_CMD_EXTMODE_SEQ_1: u8 = 0xB4;
pub(crate) const BMP5_CMD_EXTMODE_SEQ_2: u8 = 0x69;
pub(crate) const BMP5_CMD_NVM_OP_SEQ_0: u8 = 0x5D;
pub(crate) const BMP5_CMD_NVM_READ_SEQ_1: u8 = 0xA5;
pub(crate) const BMP5_CMD_NVM_WRITE_SEQ_1: u8 = 0xA0;
pub(crate) const BMP5_CMD_SOFT_RESET: u8 = 0xB6;

// Power mode selection
pub(crate) const BMP5_MODE_MASK: u8 = 0x03;
pub(crate) const BMP5_MODE_STANDBY: u8 = 0x00;
pub(crate) const BMP5_MODE_NORMAL: u8 = 0x01;
pub(crate) const BMP5_MODE_FORCED: u8 = 0x02;
pub(crate) const BMP5_MODE_CONTINOUS: u8 = 0x03;
pub(crate) const BMP5_ODR_DEEPSLEEP_DIS: u8 = 0x80;

// ODR settings
pub(crate) const BMP5_ODR_MASK: u8 = 0x7C;
pub(crate) const BMP5_ODR_240_HZ: u8 = 0x00;
pub(crate) const BMP5_ODR_218_5_HZ: u8 = 0x01;
pub(crate) const BMP5_ODR_199_1_HZ: u8 = 0x02;
pub(crate) const BMP5_ODR_179_2_HZ: u8 = 0x03;
pub(crate) const BMP5_ODR_160_HZ: u8 = 0x04;
pub(crate) const BMP5_ODR_149_3_HZ: u8 = 0x05;
pub(crate) const BMP5_ODR_140_HZ: u8 = 0x06;
pub(crate) const BMP5_ODR_129_8_HZ: u8 = 0x07;
pub(crate) const BMP5_ODR_120_HZ: u8 = 0x08;
pub(crate) const BMP5_ODR_110_1_HZ: u8 = 0x09;
pub(crate) const BMP5_ODR_100_2_HZ: u8 = 0x0A;
pub(crate) const BMP5_ODR_89_6_HZ: u8 = 0x0B;
pub(crate) const BMP5_ODR_80_HZ: u8 = 0x0C;
pub(crate) const BMP5_ODR_70_HZ: u8 = 0x0D;
pub(crate) const BMP5_ODR_60_HZ: u8 = 0x0E;
pub(crate) const BMP5_ODR_50_HZ: u8 = 0x0F;
pub(crate) const BMP5_ODR_45_HZ: u8 = 0x10;
pub(crate) const BMP5_ODR_40_HZ: u8 = 0x11;
pub(crate) const BMP5_ODR_35_HZ: u8 = 0x12;
pub(crate) const BMP5_ODR_30_HZ: u8 = 0x13;
pub(crate) const BMP5_ODR_25_HZ: u8 = 0x14;
pub(crate) const BMP5_ODR_20_HZ: u8 = 0x15;
pub(crate) const BMP5_ODR_15_HZ: u8 = 0x16;
pub(crate) const BMP5_ODR_10_HZ: u8 = 0x17;
pub(crate) const BMP5_ODR_05_HZ: u8 = 0x18;
pub(crate) const BMP5_ODR_04_HZ: u8 = 0x19;
pub(crate) const BMP5_ODR_03_HZ: u8 = 0x1A;
pub(crate) const BMP5_ODR_02_HZ: u8 = 0x1B;
pub(crate) const BMP5_ODR_01_HZ: u8 = 0x1C;
pub(crate) const BMP5_ODR_0_5_HZ: u8 = 0x1D;
pub(crate) const BMP5_ODR_0_250_HZ: u8 = 0x1E;
pub(crate) const BMP5_ODR_0_125_HZ: u8 = 0x1F;

// Oversampling for temperature and pressure
pub(crate) const BMP5_OSR_MASK: u8 = 0x3F;
pub(crate) const BMP5_OSR_1X: u8 = 0x00;
pub(crate) const BMP5_OSR_2X: u8 = 0x01;
pub(crate) const BMP5_OSR_4X: u8 = 0x02;
pub(crate) const BMP5_OSR_8X: u8 = 0x03;
pub(crate) const BMP5_OSR_16X: u8 = 0x04;
pub(crate) const BMP5_OSR_32X: u8 = 0x05;
pub(crate) const BMP5_OSR_64X: u8 = 0x06;
pub(crate) const BMP5_OSR_128X: u8 = 0x07;
pub(crate) const BMP5_OSR_PRESS_EN: u8 = 0x40;

pub(crate) const BMP5_OSR_EFF_VALID_ODR: u8 = 0x80;

// IIR filter for temperature and pressure
pub(crate) const BMP5_IIR_FILTER_BYPASS: u8 = 0x00;
pub(crate) const BMP5_IIR_FILTER_COEFF_1: u8 = 0x01;
pub(crate) const BMP5_IIR_FILTER_COEFF_3: u8 = 0x02;
pub(crate) const BMP5_IIR_FILTER_COEFF_7: u8 = 0x03;
pub(crate) const BMP5_IIR_FILTER_COEFF_15: u8 = 0x04;
pub(crate) const BMP5_IIR_FILTER_COEFF_31: u8 = 0x05;
pub(crate) const BMP5_IIR_FILTER_COEFF_63: u8 = 0x06;
pub(crate) const BMP5_IIR_FILTER_COEFF_127: u8 = 0x07;

// Chip id of BMP5x
pub(crate) const BMP5_CHIP_ID: u8 = 0x50;
pub(crate) const BMP5_CHIP_ID_ALT: u8 = 0x51;
