#![no_main]
#![no_std]

use cortex_m::asm::delay;
use hal::{
    gpio::GpioExt,
    prelude::{_embedded_hal_digital_v2_InputPin, _embedded_hal_digital_v2_OutputPin},
};
use stm32f4xx_hal as hal;

use hello_f446 as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Started");

    // Take ownership of the whole board
    let board = hal::pac::Peripherals::take().unwrap();

    // Take ownership of the GPIO-A pins
    let gpioa = board.GPIOA.split();
    let gpioc = board.GPIOC.split();

    // This:
    //  board.GPIOA;
    // would cause a compiler error now since GPIOA was moved from the board.

    // take ownership of PA5 pin (the LED)
    let mut pa5 = gpioa.pa5.into_push_pull_output();

    // take ownership of the PC13 pin (the push button)
    let pc13 = gpioc.pc13.into_pull_down_input();

    // Now let it blink.  Pressing the blue button exits the loop.
    loop {
        defmt::info!("Switch");
        if pa5.is_high().unwrap() {
            pa5.set_low().unwrap();
        } else {
            pa5.set_high().unwrap();
        }
        delay(2000000);

        if pc13.is_low().unwrap() {
            break;
        }
    }
    defmt::info!("Done");
    hello_f446::exit()
}
