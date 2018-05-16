extern crate tcod;
use tcod::console::{Root, Console, BackgroundFlag};
use tcod::input::KeyCode;

fn main() {
    let mut con = Root::initializer().size(80,50).title("libtcod Rust tutorial").fullscreen(false).init();
    let mut exit = false;
    while !(con.window_closed() || exit) {
        con.clear();
        con.put_char(40, 25, '@', BackgroundFlag::Set);
        con.flush();
        let keypress = con.wait_for_keypress(true);
        match keypress.code {
            KeyCode::Escape => exit = true,
            _ => {}
        }
    }
}