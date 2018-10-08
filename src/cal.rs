use chrono::prelude::*;
use chrono::Duration;
use itertools::Itertools;

pub fn printcal() {
    let now = Local::today();
    let a = cal_month(&now);
    let b = cal_month(&now.with_month(now.month() + 1).unwrap());
    let c = cal_month(&now.with_month(now.month() + 2).unwrap());

    let joined = joinlines(&a, &b);
    let joined = joinlines(&joined, &c);
    println!("{}", joined);
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

pub fn joinlines(first: &str, second: &str) -> String {
    let first = first.split(|x| x == '\n');
    let second = second.split(|x| x == '\n');
    let maxlen = first.clone().map(|x| x.len()).max().unwrap();

    first
        .zip(second)
        .map(|(fst, snd)| format!("{:width$} {}", fst, snd, width = maxlen))
        .join("\n")
}
