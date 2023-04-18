use std::collections::HashSet;
use std::iter::FromIterator;

fn dfs(employee_number: usize, visited: &mut Vec<bool>, employee: &Vec<Vec<usize>>, work: &mut Vec<usize>) -> bool {
    if visited[employee_number] {
        return false;
    }
        
    visited[employee_number] = true;
    for &work_number in &employee[employee_number] {
        if work[work_number] == 0 || dfs(work[work_number], visited, employee, work) {
            work[work_number] = employee_number;
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
    
    let mut employee: Vec<Vec<usize>> = Vec::new();
    employee.push(Vec::new());
    for _ in 0..n {
        buf.clear();
        input.read_line(&mut buf).unwrap();
        let temp: Vec<usize> = buf.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
        employee.push(temp);
    }
    
    let mut work: Vec<usize> = vec![0; m+1];
    for i in 1..=n {
        let mut visited: Vec<bool> = vec![false; n+1];
        dfs(i, &mut visited, &employee, &mut work);
    }
    
    let result: usize = work.iter().skip(1).map(|&x| if x > 0 { 1 } else { 0 }).sum();
    println!("{}", result);
}
