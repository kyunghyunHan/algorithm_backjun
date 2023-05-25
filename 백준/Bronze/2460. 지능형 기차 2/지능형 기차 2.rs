use std::io;

fn main() {
    let mut get_on: [i32; 10] = [0; 10];
    let mut get_off: [i32; 10] = [0; 10];
    let mut max = 0;
    let mut people = 0;

    for i in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();

        let get_on_count: i32 = values.next().unwrap().parse().expect("Invalid input");
        let get_off_count: i32 = values.next().unwrap().parse().expect("Invalid input");

        get_on[i] = get_on_count;
        get_off[i] = get_off_count;

        people = get_off[i] - get_on[i] + people;
        if max < people {
            max = people;
        }
    }

    println!("{}", max);
}
