use bitflags::bitflags;

pub enum JoypadButtons {
    Start,
    Select,
    B,
    A,
    Down,
    Up,
    Left,
    Right,
}

bitflags! {
    #[derive(Default)]
    struct Buttons:u8 {
        const START  = 1 << 7;
        const SELECT = 1 << 6;
        const B      = 1 << 5;
        const A      = 1 << 4;
        const DOWN   = 1 << 3;
        const UP     = 1 << 2;
        const LEFT   = 1 << 1;
        const RIGHT  = 1 << 0;
    }
}

impl From<JoypadButtons> for Buttons {
    fn from(button: JoypadButtons) -> Self {
        match button {
            JoypadButtons::Start => Self::START,
            JoypadButtons::Select => Self::SELECT,
            JoypadButtons::B => Self::B,
            JoypadButtons::A => Self::A,
            JoypadButtons::Down => Self::DOWN,
            JoypadButtons::Up => Self::UP,
            JoypadButtons::Left => Self::LEFT,
            JoypadButtons::Right => Self::RIGHT,
        }
    }
}

pub struct JoyPad {
    button_select: bool,
    direction_select: bool,
    buttons: Buttons,
}

impl Default for JoyPad {
    fn default() -> Self {
        Self {
            button_select: true,
            direction_select: true,
            buttons: Buttons::default(),
        }
    }
}

impl JoyPad {
    pub fn button_select(&self) -> bool {
        self.button_select
    }
    pub fn direction_select(&self) -> bool {
        self.direction_select
    }

    pub fn read_joypad(&self) -> u8 {
        let mut output = 0xCF;
        if !self.button_select {
            output &= !self.buttons.bits() >> 4;
        }
        if !self.direction_select() {
            output &= !self.buttons.bits();
        }
        // || (((!self.button_select) as u8) << 5) || ((!self.direction_select as u8) << 4)
        // only the bottom 4 bits are read
        output
    }

    pub fn write_joypad(&mut self, value: u8) {
        self.button_select = ((value >> 5) & 1) == 0;
        self.direction_select = ((value >> 4) & 1) == 0;
    }

    pub fn press_joypad(&mut self, button: JoypadButtons) {
        self.buttons.remove(button.into())
    }

    pub fn release_joypad(&mut self, button: JoypadButtons) {
        self.buttons.insert(button.into())
    }
}