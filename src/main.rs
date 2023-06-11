
fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
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
    if year < 1900 {   // 1800s
        3
    } else if year < 2000 {   // 1900s
        1
    } else if year < 2100 {   // 2000s
        0
    } else {   // 2100s
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
    match day_code {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        0 => "Sunday",
        _ => "[it broke]"   // refactor to use Option?
    }
}

fn print_day_of_date(day: i32, month: i32, year: i32) {
    let shifted_year = shift_by_400(year);   // 1996

    let mut year_code = shifted_year % 100;   // 96
    year_code = five_quarters_round(year_code);   // 120
    if is_leap_year(shifted_year) {
        year_code -= 1;   // 119
    }
    year_code += calc_century_modifier(shifted_year);   // 120

    let day_code = (day + month_code(month) + year_code) % 7;

    println!("{}", day + month_code(month) + year_code);
    println!("{}/{}/{} is a {}", day, month, year, day_from_code(day_code));
}

fn main() {
    // print_day_of_date(24, 9, 2057);
    // print_day_of_date(25, 7, 2003);
    // print_day_of_date(18, 2, 2100);
    // print_day_of_date(4, 3, 1992);

    print_day_of_date(22, 4, 1996);
}
