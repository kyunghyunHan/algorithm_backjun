fn dfs(i: usize, emp: &Vec<Vec<usize>>, work: &mut Vec<usize>, visited: &mut Vec<bool>) -> bool {
    if visited[i] {
        return false;
    }
    visited[i] = true;
    for x in &emp[i] {
        if work[*x] == 0 || dfs(work[*x], emp, work, visited) {
            work[*x] = i;
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut  k: usize = iter.next().unwrap().parse().unwrap();

    let mut emp: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut work: Vec<usize> = vec![0; m + 1];

    for i in 1..=n {
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).unwrap();
        let temp: Vec<usize> = temp
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect();
        emp[i] = temp;
    }

    let mut count = 0;

    for i in 1..=n {
        let mut visited: Vec<bool> = vec![false; n + 1];
        if dfs(i, &emp, &mut work, &mut visited) {
            count += 1;
        }
    }

    for i in 1..=n {
        let mut visited: Vec<bool> = vec![false; n + 1];
        if dfs(i, &emp, &mut work, &mut visited) {
            count += 1;
            k -= 1;
            if k == 0 {
                break;
            }
        }
    }

    println!("{}", count);
}
