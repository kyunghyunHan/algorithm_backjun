use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn is_leap_year(year: i32) -> bool {
  (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn calculate_anniversary_date(n: i32) -> String {
  let start_year = 2014;
  let start_month = 4;
  let start_day = 1;

  let mut year = start_year;
  let mut month = start_month;
  let mut day = start_day;

  for _ in 0..n {
      day += 1;

      let days_in_month = match month {
          1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
          4 | 6 | 9 | 11 => 30,
          2 => {
              if is_leap_year(year) {
                  29
              } else {
                  28
              }
          }
          _ => panic!("Invalid month"),
      };

      if day > days_in_month {
          day = 1;
          month += 1;

          if month > 12 {
              month = 1;
              year += 1;
          }
      }
  }

  format!("{:04}-{:02}-{:02}", year, month, day)
}

fn main() {
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let n= input.trim().parse().unwrap();
  let anniversary_date = calculate_anniversary_date(n);
  println!("{}", anniversary_date);
}
