extern crate tcod;
use tcod::console;
use tcod::console::{BackgroundFlag, Console, Offscreen, Root};
use tcod::input;
use tcod::input::KeyCode;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

fn handle_input(cur_x: &mut i32, cur_y: &mut i32, exit: &mut bool, key_code: KeyCode) {
    match key_code {
            KeyCode::Escape => *exit = true,
            KeyCode::Left => {
                *cur_x = *cur_x - 1;
                if *cur_x < 0 {
                    *cur_x = 0
                }
            }
            KeyCode::Right => {
                *cur_x = *cur_x + 1;
                if *cur_x >= SCREEN_WIDTH {
                    *cur_x = SCREEN_WIDTH - 1
                }
            }
            KeyCode::Up => {
                *cur_y = *cur_y - 1;
                if *cur_y < 0 {
                    *cur_y = 0
                }
            }
            KeyCode::Down => {
                *cur_y = *cur_y + 1;
                if *cur_y >= SCREEN_HEIGHT {
                    *cur_y = SCREEN_HEIGHT - 1
                }
            }
            _ => {}
    }
}

fn main() {
    let mut con = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("libtcod Rust tutorial")
        .fullscreen(false)
        .init();

    let mut offscreen = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut exit = false;

    let mut cur_x = 40;
    let mut cur_y = 25;

    while !(con.window_closed() || exit) {
        offscreen.clear();
        offscreen.put_char(cur_x, cur_y, '@', BackgroundFlag::Set);

        console::blit(
            &offscreen,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut con,
            (0, 0),
            1.0,
            1.0,
        );

        con.flush();
        let keypress = con.check_for_keypress(input::KEY_PRESSED);
        
        if let Some(keypress) = keypress {
            handle_input(&mut cur_x, &mut cur_y, &mut exit, keypress.code);
        }
    }
}
