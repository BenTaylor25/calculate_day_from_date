
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
        _ => panic!("month_code() invalid month")
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
        _ => panic!("day_from_code() invalid day code")
    }
}

fn print_day_of_date(day: i32, month: i32, year: i32) {
    /* 
        The Gregorian Calendar repeats every 400 years.
        Move all years into the range 1800-2199 for ease of calculation.
    */
    let shifted_year = shift_by_400(year);

    /*
        1. Take the last two digits of the year.
        2. Multiply by 5/4 and truncate.
        3. Add century-offset (1800s: 3, 1900s: 1, 2000s: 0, 2100s 5).
    */
    let mut year_code = shifted_year % 100;
    year_code = five_quarters_round(year_code);
    year_code += calc_century_modifier(shifted_year);

    /*
        Each month has a fixed month code
        (but January and February are offset by -1 on leap years).

        1. Determine month_code.
        2. Sum date_of_month + month_code + year_code.
        3. REM 7.
    */
    let leap_offset = if is_leap_year(year) && month <= 2 {-1} else {0};
    let day_code = (day + month_code(month) + leap_offset + year_code) % 7;

    /*
        day_code of 0 => Sunday
        day_code of 1 => Monday
        ...
        day_code of 6 => Saturday
    */

    println!("{}   ({}/{}/{})", day_from_code(day_code), day, month, year);
}

fn main() {
    print_day_of_date(24, 09, 2057);
    print_day_of_date(14, 11, 1893);
    print_day_of_date(04, 03, 1992);
    print_day_of_date(18, 02, 2100);
    print_day_of_date(25, 07, 2003);
    print_day_of_date(26, 07, 3000);
    print_day_of_date(24, 05, 2550);
}
