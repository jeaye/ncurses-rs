extern mod ncurses;

fn main()
{
    ncurses::initscr();

    ncurses::printw("Hello, world!");
    ncurses::refresh();

    ncurses::getch();
    ncurses::endwin();
}

