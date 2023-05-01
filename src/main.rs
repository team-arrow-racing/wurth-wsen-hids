#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32l4xx_hal as hal;

use defmt_rtt as _;
use defmt::println;
use hal::prelude::_stm32l4_hal_FlashExt;
use hal::pwr::PwrExt;
use hal::rcc::RccExt;
use panic_probe as _;

use crate::hal::prelude::*;

use crate::hal::i2c;
use crate::hal::i2c::I2c;

use cortex_m_rt::entry;

// Define the system clock
// const SYSCLK: u32 = 80_000_000;

#[entry]
fn not_main() -> ! {

    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let mut sc1 =
        gpioa
            .pa9
            .into_alternate_open_drain(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    sc1.internal_pull_up(&mut gpioa.pupdr, true);

    let mut sda =
        gpioa
            .pa10
            .into_alternate_open_drain(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    sda.internal_pull_up(&mut gpioa.pupdr, true);

    let mut i2c = I2c::i2c1(dp.I2C1,
        (sc1, sda),
        i2c::Config::new(100.kHz(), clocks),
        &mut rcc.apb1r1,
    );

    let mut total_buffer: [u8; 10] = [0; 10];

    const HUMIDITY_SLAVE_ADDRESS: u8 = 0x5F;


    loop {
        match i2c.read(HUMIDITY_SLAVE_ADDRESS, &mut total_buffer[..]) {
            Ok(_) => println!("Read OK."),
            Err(_) => panic!("Could not read I2C from device."),
        }

        for reading in total_buffer {
            defmt::debug!("Reading: {}", reading);
        }
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    loop {
        
    }
}