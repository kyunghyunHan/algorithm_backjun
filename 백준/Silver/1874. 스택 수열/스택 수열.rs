use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main() {
let mut reader= BufReader::new(stdin().lock());
let mut input =String::new();
reader.read_line(&mut input).unwrap();
let n:i32= input.trim().parse().unwrap();


let mut current = 0;
let mut stack: Vec<i32> = vec![0];
let mut answer: Vec<String> = vec![];
let mut writer= BufWriter::new(stdout().lock());
// 수열 입력받기
for _ in 0..n as usize {
    input.clear();
    reader.read_line(&mut input).unwrap();
    // Boundary Condition (stack, current, target)
    let target:i32= input.trim().parse().unwrap();
    // (+) When current < target
    while current < target {
        current += 1;
        stack.push(current);
        answer.push("+".to_owned());

        // Current = Target 에 도달했을 때 마지막 값을 빼줌
        if current == target {
            stack.pop();
            answer.push("-".to_owned());
        }
    }

    // Current 값이 Target 값보다 작은 경우
    if current > target {

        //오름차순 수열에 위반되는 경우 i.e current > target인데, Push(+)가 수행되는 경우
        if stack[stack.len() - 1] < target {
            stack.push(-1);
            break;
        }

        while stack[stack.len() - 1] >= target {
            stack.pop();
            answer.push("-".to_owned());
        }
    }
}

// 답 출력
if stack.len() != 1 {
    writeln!(writer,"NO").unwrap();
} else {
    for method in answer {
        writeln!(writer,"{}", method).unwrap();
    }
}

}
