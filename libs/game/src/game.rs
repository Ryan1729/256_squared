use features::{GLOBAL_ERROR_LOGGER, GLOBAL_LOGGER};
use platform_types::{Button, Input, Speaker, State, StateParams, SFX};
use rendering::{Framebuffer, BLACK, BLUE, GREEN, RED, WHITE};

pub struct GameState {
    pub offset: usize,
    pub is_checkerboard: bool,
}

impl GameState {
    pub fn new(_seed: [u8; 16]) -> GameState {
        GameState {
            offset: 0,
            is_checkerboard: false,
        }
    }
}

pub struct EntireState {
    pub game_state: GameState,
    pub framebuffer: Framebuffer,
    pub input: Input,
    pub speaker: Speaker,
}

impl EntireState {
    pub fn new((seed, logger, error_logger): StateParams) -> Self {
        let framebuffer = Framebuffer::new();

        unsafe {
            GLOBAL_LOGGER = logger;
            GLOBAL_ERROR_LOGGER = error_logger;
        }

        EntireState {
            game_state: GameState::new(seed),
            framebuffer,
            input: Input::new(),
            speaker: Speaker::new(),
        }
    }
}

impl State for EntireState {
    fn frame(&mut self, handle_sound: fn(SFX)) {
        update_and_render(
            &mut self.framebuffer,
            &mut self.game_state,
            self.input,
            &mut self.speaker,
        );

        self.input.previous_gamepad = self.input.gamepad;

        for request in self.speaker.drain() {
            handle_sound(request);
        }
    }

    fn press(&mut self, button: Button::Ty) {
        if self.input.previous_gamepad.contains(button) {
            //This is meant to pass along the key repeat, if any.
            //Not sure if rewriting history is the best way to do this.
            self.input.previous_gamepad.remove(button);
        }

        self.input.gamepad.insert(button);
    }

    fn release(&mut self, button: Button::Ty) {
        self.input.gamepad.remove(button);
    }

    fn get_frame_buffer(&self) -> &[u32] {
        &self.framebuffer.buffer
    }
}

fn checkerboard_pattern(framebuffer: &mut Framebuffer, state: &mut GameState) {
    use rendering::{PALETTE, SCREEN_WIDTH};
    let mut index = state.offset % PALETTE.len();
    let mut last_row = 0;
    for (i, pixel) in framebuffer.buffer.iter_mut().enumerate() {
        let row = i / SCREEN_WIDTH;
        if last_row != row {
            index = (index + 1) % PALETTE.len();
        }
        last_row = row;
        *pixel = PALETTE[index];
        index = (index + 1) % PALETTE.len();
    }
}

fn test_pattern(framebuffer: &mut Framebuffer, state: &mut GameState) {
    use rendering::PALETTE;
    let mut index = state.offset % PALETTE.len();
    let mut counter = 0;
    for pixel in framebuffer.buffer.iter_mut() {
        *pixel = PALETTE[index];

        counter += 1;
        if counter >= PALETTE.len() {
            counter = 0;
            index = (index + 1) % PALETTE.len();
        }
    }
}

#[inline]
pub fn update_and_render(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    _speaker: &mut Speaker,
) {
    match input.gamepad {
        Button::A => framebuffer.clear_to(GREEN),
        Button::B => framebuffer.clear_to(BLUE),
        Button::Select => framebuffer.clear_to(WHITE),
        Button::Start => framebuffer.clear_to(RED),
        _ => {
            match input.gamepad {
                Button::Right => state.offset -= 1,
                Button::Left => state.offset += 1,
                Button::Up => state.is_checkerboard = !state.is_checkerboard,
                Button::Down => state.is_checkerboard = !state.is_checkerboard,
                _ => {}
            }
            if state.is_checkerboard {
                checkerboard_pattern(framebuffer, state);
            } else {
                test_pattern(framebuffer, state);
            }
        }
    }
}
