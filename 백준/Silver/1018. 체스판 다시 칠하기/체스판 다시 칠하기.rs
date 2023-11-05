use core::num;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());

  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
  let nums= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
  let n= nums[0];
  let m = nums[1];
  let mut board = Vec::new();
    let mut result = Vec::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        board.push(input.trim().to_string());
    }

    for i in 0..n - 7 {
        for j in 0..m - 7 {
            let mut draw1 = 0;
            let mut draw2 = 0;

            for a in i..i + 8 {
                for b in j..j + 8 {
                    if (a + b) % 2 == 0 {
                        if board[a].as_bytes()[b] as char != 'B' {
                            draw1 += 1;
                        }
                        if board[a].as_bytes()[b] as char != 'W' {
                            draw2 += 1;
                        }
                    } else {
                        if board[a].as_bytes()[b] as char != 'W' {
                            draw1 += 1;
                        }
                        if board[a].as_bytes()[b] as char != 'B' {
                            draw2 += 1;
                        }
                    }
                }
            }

            result.push(draw1);
            result.push(draw2);
        }
    }

    
  writeln!(writer,"{:?}",result.iter().min().unwrap()).unwrap();
}
