use std::collections::HashSet;
use std::iter::FromIterator;

fn dfs(number: usize, visited: &mut Vec<bool>, employee: &Vec<Vec<usize>>, task: &mut Vec<usize>) -> bool {
    if visited[number] {
        return false;
    }
    
    visited[number] = true;
    for &t in &employee[number] {
        if number != task[t] && (task[t] == 0 || dfs(task[t], visited, employee, task)) {
            task[t] = number;
            return true;
        }
    }
    
    false
}

fn main() {
    let input = std::io::stdin();
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    let mut employee: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    let mut task: Vec<usize> = vec![0; m+1];
    let mut count: Vec<usize> = vec![0; n+1];
    
    for i in 1..=n {
        buf.clear();
        input.read_line(&mut buf).unwrap();
        let temp: Vec<usize> = buf.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
        employee[i] = temp;
    }
    
    for number in 1..=n {
        let mut visited: Vec<bool> = vec![false; n+1];
        dfs(number, &mut visited, &employee, &mut task);
        let mut visited: Vec<bool> = vec![false; n+1];
        dfs(number, &mut visited, &employee, &mut task);
    }
    
    let result: usize = task.iter().skip(1).map(|&x| if x > 0 { 1 } else { 0 }).sum();
    println!("{}", result);
}
