#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::{adc, stm32 ,prelude::*};

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::{OutputSwitch, ToggleableOutputSwitch};


#[entry]
fn main() -> ! {

    let mut device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // initialize user leds
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    let mut adc1 = adc::Adc::adc1(
        device_periphs.ADC1, // The ADC we are going to control
        // The following is only needed to make sure the clock signal for the ADC is set up
        // correctly.
        &mut device_periphs.ADC1_2,
        &mut reset_and_clock_control.ahb,
        adc::CKMODE::default(),
        clocks,
        // If everything is set up correctly, we'll get `Some(adc1)`. to access it, we need to `unwrap`
        // it. If there was an error, we'll get `None` and this unwrap will `panic!`.
    ).unwrap();

    // Set up pin PA0 as analog pin.
    // This pin is connected to the user button on the stm32f3discovery board.
    let mut gpio_a = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let mut adc1_in1_pin = gpio_a.pa1.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);

    loop {
        let adc1_in1_data: u16 = adc1.read(&mut adc1_in1_pin).expect("Error reading adc1.");

        if adc1_in1_data > 50 {
            leds.ld3.toggle().ok();
            delay.delay_ms(100u16);
            leds.ld3.toggle().ok();
            delay.delay_ms(100u16);
        }
        else {

            //explicit on/off
            leds.ld4.on().ok();
            delay.delay_ms(100u16);
            leds.ld4.off().ok();
            delay.delay_ms(100u16);
        }
    }
}
