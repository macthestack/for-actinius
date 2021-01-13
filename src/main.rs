#![no_std]
#![no_main]

mod leds;

use panic_reset as _;

use actinius_icarus_bsp::{
    hal::{prelude::_embedded_hal_timer_CountDown, timer::Periodic, Timer},
    pac::TIMER2_NS as TIMER,
    Board, Leds,
};
use leds::{LedState, SettableLeds};
use rtt_target::{rprintln, rtt_init_print};

// Run every 10 seconds.
const INTERVAL: u32 = 10 * 1_000_000;

#[rtic::app(device = actinius_icarus_bsp::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        leds: Leds,
        timer: Timer<TIMER, Periodic>,
    }

    #[init(spawn = [disco])]
    fn init(c: init::Context) -> init::LateResources {
        rtt_init_print!();

        rprintln!("Start.");

        let mut board = Board::new(c.core, c.device);
        board.leds.set_state(LedState::Red);

        let timer = Timer::new(board.TIMER2_NS);
        let mut timer = timer.into_periodic();
        timer.enable_interrupt();
        timer.start(INTERVAL);

        c.spawn.disco().unwrap();

        init::LateResources {
            leds: board.leds,
            timer,
        }
    }

    #[task(binds = TIMER2, resources = [timer], spawn = [disco])]
    fn timer_handle(c: timer_handle::Context) {
        // Clearing Interrupt.
        c.resources
            .timer
            .event_compare_cc0()
            .write(|w| unsafe { w.bits(0) });

        c.spawn.disco().unwrap();
    }

    #[task(resources = [leds])]
    fn disco(c: disco::Context) {
        rprintln!("Disco.");
        for i in 0..128 {
            let color = match i % 6 {
                0 => LedState::Red,
                1 => LedState::Magenta,
                2 => LedState::Blue,
                3 => LedState::Cyan,
                4 => LedState::Green,
                _ => LedState::Yellow,
            };
            c.resources.leds.set_state(color);
            cortex_m::asm::delay(2_000_000)
        }
        c.resources.leds.set_state(LedState::Off);
    }

    #[idle]
    fn idle(_c: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    extern "C" {
        fn EGU1();
    }
};
