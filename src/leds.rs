use actinius_icarus_bsp::Leds;

pub trait SettableLeds {
    fn set_state(&mut self, state: LedState) -> ();
}
impl SettableLeds for Leds {
    fn set_state(&mut self, state: LedState) {
        let (r, g, b) = match state {
            LedState::Off => (false, false, false),
            LedState::Red => (true, false, false),
            LedState::Green => (false, true, false),
            LedState::Blue => (false, false, true),
            LedState::Yellow => (true, true, false),
            LedState::Cyan => (false, true, true),
            LedState::Magenta => (true, false, true),
            LedState::White => (true, true, true),
        };

        match r {
            true => self.red.enable(),
            false => self.red.disable(),
        }
        match g {
            true => self.green.enable(),
            false => self.green.disable(),
        }
        match b {
            true => self.blue.enable(),
            false => self.blue.disable(),
        }
    }
}
#[allow(unused)]
pub enum LedState {
    Off,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
}
