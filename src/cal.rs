use chrono::prelude::*;
use chrono::Duration;
use yansi::{Style,Color};

use utils;

struct Cell {
    date: NaiveDate,
    content: (String,String)
}

pub fn printcal() {
    let now = Local::today();
    let a = cal_month(&now);
    let b = cal_month(&now.with_month(now.month() + 1).unwrap());
    let c = cal_month(&now.with_month(now.month() + 2).unwrap());

    let joined = utils::joinlines(&a, &b);
    let joined = utils::joinlines(&joined, &c);
    println!("{}", joined);
}

pub fn dbg() {
    let begin = Local::today().naive_local();
    let end = begin + Duration::days(5);
    let cells = get_cells(begin, end);
    let cells = expand_cells_to_week(cells);

    let render = render_cells(&cells);
    print!("{}", render);
}

fn render_cells(cells: &Vec<Cell>) -> String {
    let mut result = String::with_capacity(50);

    let now = cells[0].date.clone();

    result.push_str(&format!("{:>28} {:<8}\n",
        now.format("%B").to_string(),
        now.format("%Y").to_string()
    ));
    let weekdays = &[ "Mo", "Tu", "We", "Th", "Fr", "Sa", "Su" ].iter().map(|x| format!("{:8}", x)).collect::<String>();
    result.push_str(weekdays);
    result.push_str("\n");

    let flow = render_flow(7, 8, cells);
    result.push_str(&flow);

    result
}

fn render_flow(cells_per_line: usize, cell_width: usize, cells: &Vec<Cell>) -> String {
    let mut result = String::with_capacity(50);

    let style = Style::new().bg(Color::Fixed(236));

    let it = cells.iter();
    let mut n = 0;
    while n < (cells.len() / cells_per_line) {
        let line = it.clone().skip(n * cells_per_line).take(cells_per_line);
        for cell in line.clone() {
            let cellstr = &format!("{:width$}", &cell.content.0, width = cell_width);
            let cellstr = &format!("{}", style.paint(cellstr));
            result.push_str(cellstr);
        }
        result.push_str("\n");
        for cell in line {
            let cellstr = &format!("{:width$}", &cell.content.1, width = cell_width);
            let cellstr = &format!("{}", style.paint(cellstr));
            result.push_str(cellstr);
        }
        let emptyline = &format!("{:width$}", "", width = cell_width * cells_per_line);
        result.push_str("\n");
        result.push_str(&format!("{}\n", style.paint(emptyline)));
        n += 1;
    }

    result
}

fn get_cells(date_begin: NaiveDate, date_end: NaiveDate) -> Vec<Cell> {
    let mut result = vec!();
    let mut date = date_begin;
    while date < date_end {
        let cell = cell_whatever(date);
        result.push(cell);

        date += Duration::days(1);
    }
    result
}

fn cell_whatever(date: NaiveDate) -> Cell {
    let fst = date.format("%d").to_string();
    let snd = String::from("");
    Cell{date, content: (fst, snd)}
}

fn cell_empty(date: NaiveDate) -> Cell {
    let fst = date.format("%d").to_string();
    let snd = String::from("");
    Cell{date, content: (fst, snd)}
}

fn expand_cells_to_week(cells: Vec<Cell>) -> Vec<Cell> {
    let mut result = vec!();

    let mut day = NaiveDate::from_isoywd(cells[0].date.year(), cells[0].date.iso_week().week(), Weekday::Mon);
    while day < cells[0].date {
        let cell = cell_empty(day);
        result.push(cell);

        day += Duration::days(1);
    }

    let mut day = cells[cells.len() - 1].date;

    for cell in cells {
        result.push(cell);
    }

    let last_date = NaiveDate::from_isoywd(day.year(), day.iso_week().week(), Weekday::Sun);
    day += Duration::days(1);
    while day <= last_date {
        let cell = cell_empty(day);
        result.push(cell);

        day += Duration::days(1);
    }

    result
}

pub fn cal_month(now: &Date<Local>) -> String {
    let mut result = String::with_capacity(50);

    result.push_str(&format!("{:>11} {:<8}\n",
        now.format("%B").to_string(),
        now.format("%Y").to_string()
    ));
    result.push_str("Su Mo Tu We Th Fr Sa\n");

    let this_month = now.month();
    let mut current_day = Local.ymd(now.year(), now.month(), 1);

    let one_day = Duration::days(1);
    for _ in 0..current_day.weekday().num_days_from_sunday() {
        result.push_str("   ");
    }
    while current_day.month() == this_month {
        result.push_str(&format!("{:>2} ", current_day.day()));
        if current_day.weekday() == Weekday::Sat {
            result.push_str("\n");
        }
        current_day = current_day + one_day;
    }
    result.push_str("\n");

    result
}
