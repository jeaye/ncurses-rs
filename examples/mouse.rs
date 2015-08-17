extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    raw();
    keypad(stdscr, true);
    noecho();

    clear();
    cbreak();

    mousemask((ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION) as u64, None);

    let mut count = 0;
    loop {
        let ch = getch();
        if ch == 'q' as i32 {
            break;
        }
        clear();
        count += 1;
        mvprintw(1, 1, &format!("Has mouse: {}", has_mouse()));
        mvprintw(2, 1, &format!("Key code: {:x}; mouse code: {:x}", ch, KEY_MOUSE));
        if ch == KEY_MOUSE {
            let mut event = MEVENT { id: 0, x: 0, y: 0, z: 0, bstate: 0 };
            let ret = getmouse(&mut event);
            if ret != OK {
                mvprintw(3, 1, "getmouse() error!");
                getch();
                break;
            }
            mvprintw(event.y, event.x, "X");
            mvprintw(3, 3, &format!("Mouse Event: x={}, y={}, z={}", event.x, event.y, event.z));
            mvprintw(4, 3, &format!("Mouse device id: {:x}", event.id));
            mvprintw(5, 3, &format!("Mouse button mask: {:x}", event.bstate));
        }
        mvprintw(6, 1, &format!("Event number: {:04}", count));
        refresh();
    }
    endwin();
}
