use features::{GLOBAL_ERROR_LOGGER, GLOBAL_LOGGER};
use platform_types::{Button, Input, Speaker, State, StateParams, SFX};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rendering::{Framebuffer, BLACK, BLUE, GREEN, PURPLE, RED, WHITE};

pub enum Mode {
    ViewFunc2,
    VisualizeFunc,
    TestPattern,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::ViewFunc2
    }
}

//a way to represent one of the possible functions from an i8 to another i8.
//AKA fn f(a: i8) -> i8;
pub type Func = [i8; 256];

//a way to represent one of the possible functions from two i8s to a third one.
//AKA fn f(a: i8, b: i8) -> i8;
pub type Func2 = [[i8; 256]; 256];

pub struct GameState {
    pub x_offset: usize,
    pub y_offset: usize,
    pub is_checkerboard: bool,
    pub rng: XorShiftRng,
    pub func: Func,
    pub func2: Func2,
    pub mode: Mode,
}

fn randomize_func<R: Rng>(rng: &mut R, func: &mut Func) {
    rng.fill(func);
}

fn randomize_func2<R: Rng>(rng: &mut R, func: &mut Func2) {
    for row in func.iter_mut() {
        rng.fill(row);
    }
}

impl GameState {
    pub fn new(seed: [u8; 16]) -> GameState {
        let mut rng = XorShiftRng::from_seed(seed);

        let mut func = [0; 256];
        randomize_func(&mut rng, &mut func);

        let mut func2 = [[0; 256]; 256];
        randomize_func2(&mut rng, &mut func2);

        GameState {
            x_offset: 0,
            y_offset: 0,
            is_checkerboard: false,
            rng,
            func,
            func2,
            mode: Default::default(),
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
    let mut index = state.x_offset % PALETTE.len();
    let mut last_row = 0;
    let mut counter = 0;
    for (i, pixel) in framebuffer.buffer.iter_mut().enumerate() {
        let row = i / SCREEN_WIDTH;
        if last_row != row {
            index = (index + 1) % PALETTE.len();
        }
        last_row = row;
        *pixel = PALETTE[index];

        counter += 1;
        if counter >= PALETTE.len() {
            counter = 0;
            index = (index + 1) % PALETTE.len();
        }
    }
}

fn test_pattern(framebuffer: &mut Framebuffer, state: &mut GameState) {
    use rendering::PALETTE;
    let mut index = state.x_offset % PALETTE.len();
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

fn xy_to_i((x, y): (i8, i8)) -> usize {
    let (x_corner, y_corner) = (x.wrapping_sub(-128), y.wrapping_sub(-128));

    ((((1 << 8) - y_corner as i16) as u8 as usize) << 8) | x_corner as u8 as usize
}

fn i_to_xy(i: usize) -> (i8, i8) {
    let (x_corner, y_corner) = ((i & 0b1111_1111) as i8, ((1 << 8) - (i >> 8)) as i8);

    (x_corner.wrapping_add(-128), y_corner.wrapping_add(-128))
}

#[cfg(test)]
mod test {
    use super::*;

    use quickcheck::quickcheck;

    quickcheck! {
        fn i_roundtrips(i: usize) -> bool {
            i == xy_to_i(i_to_xy(i))
        }

        fn xy_roundtrips(xy: (i8, i8)) -> bool {
            xy == i_to_xy(xy_to_i(xy))
        }
    }
}

fn update_and_render_test_pattern(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
) {
    if input.gamepad.contains(Button::Up) || input.gamepad.contains(Button::Right) {
        state.is_checkerboard = !state.is_checkerboard
    }
    if input.gamepad.contains(Button::Down) || input.gamepad.contains(Button::Left) {
        state.is_checkerboard = !state.is_checkerboard
    }
    if state.is_checkerboard {
        checkerboard_pattern(framebuffer, state);
    } else {
        test_pattern(framebuffer, state);
    }
}

fn apply_func2(framebuffer: &mut Framebuffer, state: &mut GameState) {
    framebuffer.clear_to(RED);

    let func = &state.func2;

    for i in 0..(256 * 256) {
        let (mut x, mut y) = i_to_xy(i);

        x = func[state.x_offset][x as u8 as usize];
        y = func[state.y_offset][y as u8 as usize];

        let i = xy_to_i((x, y));

        framebuffer.buffer[i] = BLUE;
    }
}

fn update_and_render_view_func2(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
) {
    let (left, right, up, down) = if input.gamepad.contains(Button::B) {
        (
            input.gamepad.contains(Button::Left),
            input.gamepad.contains(Button::Right),
            input.gamepad.contains(Button::Up),
            input.gamepad.contains(Button::Down),
        )
    } else {
        (
            input.pressed_this_frame(Button::Left),
            input.pressed_this_frame(Button::Right),
            input.pressed_this_frame(Button::Up),
            input.pressed_this_frame(Button::Down),
        )
    };

    if right {
        state.x_offset = (state.x_offset as u8).saturating_add(1) as _;
    }
    if left {
        state.x_offset = (state.x_offset as u8).saturating_sub(1) as _;
    }
    if up {
        state.y_offset = (state.y_offset as u8).saturating_add(1) as _;
    }
    if down {
        state.y_offset = (state.y_offset as u8).saturating_sub(1) as _;
    }

    match input.gamepad {
        Button::Select => framebuffer.clear_to(WHITE),
        Button::Start => {
            randomize_func2(&mut state.rng, &mut state.func2);
            framebuffer.clear_to(GREEN)
        }
        _ => {
            apply_func2(framebuffer, state);
        }
    }
}

fn apply_func(framebuffer: &mut Framebuffer, state: &mut GameState) {
    framebuffer.clear_to(RED);

    let func = &state.func;

    for x in (-128)..=127 {
        let y = func[x as u8 as usize];

        let i = xy_to_i((x, y));

        framebuffer.buffer[i] = BLUE;
    }
}

fn update_and_render_visualize_func(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
) {
    match input.gamepad {
        Button::Select => framebuffer.clear_to(WHITE),
        Button::Start => {
            randomize_func(&mut state.rng, &mut state.func);
            framebuffer.clear_to(GREEN)
        }
        _ => {
            apply_func(framebuffer, state);
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
    if input.pressed_this_frame(Button::A) {
        state.mode = match state.mode {
            Mode::ViewFunc2 => Mode::VisualizeFunc,
            Mode::VisualizeFunc => Mode::TestPattern,
            Mode::TestPattern => Mode::ViewFunc2,
        };
    }

    return match state.mode {
        Mode::ViewFunc2 => {
            update_and_render_view_func2(framebuffer, state, input);
        }
        Mode::VisualizeFunc => {
            update_and_render_visualize_func(framebuffer, state, input);
        }
        Mode::TestPattern => {
            update_and_render_test_pattern(framebuffer, state, input);
        }
    };
}
