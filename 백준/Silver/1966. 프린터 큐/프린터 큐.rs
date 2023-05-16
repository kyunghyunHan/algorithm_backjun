use std::collections::{BinaryHeap, VecDeque};

fn main() {
    let mut testn = String::new();
    std::io::stdin().read_line(&mut testn).expect("Failed to read testn");
    let testn: i32 = testn.trim().parse().expect("Invalid testn");

    for _ in 0..testn {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut values = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());

        let n = values.next().unwrap();
        let m = values.next().unwrap();

        let mut pq = BinaryHeap::new(); // BinaryHeap for max heap behavior
        let mut q = VecDeque::new();

        let mut count = 0;

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut values = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());

        for (j, importance) in values.enumerate() {
            q.push_back((j, importance));
            pq.push(importance);
        }

        while let Some((location, value)) = q.pop_front() {
            if let Some(max) = pq.peek() {
                if *max == value {
                    pq.pop();
                    count += 1;
                    if m  as usize== location {
                        println!("{}", count);
                        break;
                    }
                } else {
                    q.push_back((location, value));
                }
            }
        }
    }
}
