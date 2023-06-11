
fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (!year % 100 == 0 || year % 400 == 0)
}

fn shift_by_400(mut year: i32) -> i32 {
    while year < 1800 {
        year += 400;
    }

    while year >= 2200 {
        year -= 400;
    }

    year
}

fn calc_century_modifier(year: i32) -> i32 {
    if year < 1900 {
        3
    } else if year < 2000 {
        1
    } else if year < 2100 {
        0
    } else {
        5
    }
}

fn five_quarters_round(x: i32) -> i32 {
    x * 5 / 4
}

fn month_code(month: i32) -> i32 {
    match month {
        1 => 6,
        2 => 2,
        3 => 2,
        4 => 5,
        5 => 0,
        6 => 3,
        7 => 5,
        8 => 1,
        9 => 4,
        10 => 6,
        11 => 2,
        12 => 4,
        _ => -500   // refactor to use Option?
    }
}

fn day_from_code(day_code: i32) -> &'static str {
    match day_code -1 {
        0 => "Monday",
        1 => "Tuesday",
        2 => "Wednesday",
        3 => "Thursday",
        4 => "Friday",
        5 => "Saturday",
        6 => "Sunday",
        _ => "[it broke]"   // refactor to use Option?
    }
}

fn print_day_of_date(day: i32, month: i32, year: i32) {
    let shifted_year = shift_by_400(year);

    let mut year_code = shifted_year % 100;
    year_code = five_quarters_round(year_code);
    if is_leap_year(shifted_year) {
        year_code -= 1;
    }
    year_code += calc_century_modifier(shifted_year);

    let day_code = (day + month_code(month) + year_code) % 7;

    println!("{}/{}/{} is a {}", day, month, year, day_from_code(day_code));
}

fn main() {
    print_day_of_date(24, 9, 2057);
    print_day_of_date(25, 7, 2003);
    print_day_of_date(18, 2, 2100);
    print_day_of_date(4, 3, 1992);
}
