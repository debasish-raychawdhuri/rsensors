use std::{thread::sleep, time::Duration};
mod sensors;
use pancurses::{curs_set, init_pair, initscr, Attribute, COLOR_BLACK, COLOR_GREEN};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let w = initscr();
    init_pair(0, COLOR_GREEN, COLOR_BLACK);
    curs_set(0);
    let mut window = w.derwin(10, 40, 2, 2).unwrap();
    loop {
        let temperatures = sensors::get_temperatures()?;
        let size = temperatures.len();
        window.resize(6 + size as i32, 44);
        let mut line = 1;
        window.attron(Attribute::Bold);
        window.color_set(0);
        window.mvprintw(line, 2, format!("{:^40}\n", "Temperatures"));
        window.attroff(Attribute::Blink);
        line += 1;
        window.mvprintw(
            line,
            1,
            format!("────────────────────────────────────────────",),
        );
        line += 1;
        window.mvprintw(
            line,
            2,
            format!("{:15} {:>7} {:>7} {:>7}\n", "", "Curr", "Max", "Min"),
        );
        line += 1;
        window.mvprintw(
            line,
            2,
            format!("{:15} {:>7} {:>7} {:>7}\n", "", "────", "───", "───"),
        );
        for (name, temperature) in temperatures {
            line += 1;
            window.mvprintw(
                line,
                2,
                format!(
                    "{:15} {:>5.0}°C {:>5.0}°C {:>5.0}°C\n",
                    name, temperature.current, temperature.maximum, temperature.minimum
                ),
            );
            window.mv(line + 1, 1);
        }

        window.draw_box(0, 0);
        window.refresh();
        //Fan speed
        let mut window = w.derwin(10, 40, size as i32 + 8, 2).unwrap();
        let fan_speed = sensors::get_fan_speeds()?;
        let size = fan_speed.len();
        window.resize(6 + size as i32, 44);
        let mut line = 1;
        window.attron(Attribute::Bold);
        window.color_set(0);
        window.mvprintw(line, 2, format!("{:^40}\n", "Fan Speeds"));
        window.attroff(Attribute::Blink);
        line += 1;
        window.mvprintw(
            line,
            1,
            format!("────────────────────────────────────────────",),
        );
        line += 1;
        window.mvprintw(
            line,
            2,
            format!("{:15} {:>7} {:>7} {:>7}\n", "", "Curr", "Max", "Min"),
        );
        line += 1;
        window.mvprintw(
            line,
            2,
            format!("{:15} {:>7} {:>7} {:>7}\n", "", "────", "───", "───"),
        );
        for (name, temperature) in fan_speed {
            line += 1;
            window.mvprintw(
                line,
                2,
                format!(
                    "{:15}   {:>5.0}   {:>5.0}   {:>5.0}\n",
                    name, temperature.current, temperature.maximum, temperature.minimum
                ),
            );
            window.mv(line + 1, 1);
        }

        window.draw_box(0, 0);
        window.refresh();
        //window.getch();
        sleep(Duration::from_secs(5));
    }
    //print_sensors()
}
