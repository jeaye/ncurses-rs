use constants::KEY_MAX;

pub const O_ONEVALUE: i32 = 0x01;
pub const O_SHOWDESC: i32 = 0x02;
pub const O_ROWMAJOR: i32 = 0x04;
pub const O_IGNORECASE: i32 = 0x08;
pub const O_SHOWMATCH: i32 = 0x10;
pub const O_NONCYCLIC: i32 = 0x20;
pub const O_SELECTABLE: i32 = 0x01;

pub const REQ_LEFT_ITEM: i32 = KEY_MAX + 1;
pub const REQ_RIGHT_ITEM: i32 = KEY_MAX + 2;
pub const REQ_UP_ITEM: i32 = KEY_MAX + 3;
pub const REQ_DOWN_ITEM: i32 = KEY_MAX + 4;
pub const REQ_SCR_ULINE: i32 = KEY_MAX + 5;
pub const REQ_SCR_DLINE: i32 = KEY_MAX + 6;
pub const REQ_SCR_DPAGE: i32 = KEY_MAX + 7;
pub const REQ_SCR_UPAGE: i32 = KEY_MAX + 8;
pub const REQ_FIRST_ITEM: i32 = KEY_MAX + 9;
pub const REQ_LAST_ITEM: i32 = KEY_MAX + 10;
pub const REQ_NEXT_ITEM: i32 = KEY_MAX + 11;
pub const REQ_PREV_ITEM: i32 = KEY_MAX + 12;
pub const REQ_TOGGLE_ITEM: i32 = KEY_MAX + 13;
pub const REQ_CLEAR_PATTERN: i32 = KEY_MAX + 14;
pub const REQ_BACK_PATTERN: i32 = KEY_MAX + 15;
pub const REQ_NEXT_MATCH: i32 = KEY_MAX + 16;
pub const REQ_PREV_MATCH: i32 = KEY_MAX + 17;

pub const MIN_MENU_COMMAND: i32 = KEY_MAX + 1;
pub const MAX_MENU_COMMAND: i32 = KEY_MAX + 17;
