use std::{io::{BufReader,BufWriter,BufRead,Write,stdin,stdout}, vec};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer=BufWriter::new(stdout().lock());
  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
  let mut inputs= input.trim().split_whitespace().map(|x|x.parse::<u32>().unwrap());
  //세로
  let n: u32= inputs.next().unwrap();
  //가로
  let m: u32= inputs.next().unwrap();
  let b: u32= inputs.next().unwrap();
  //n개의 줄에 각각 m개의 정수로 땅의 높이가 주어짐
  //블록을 제거하면 2초
  //인벤토리에서 좌표를 꺼내어 블록위에 넣으면 1초
  let mut matrix:Vec<Vec<u32>>= vec![];
  let mut t=500*500*2*257;
  let mut idx= 0;
  for i in 0..n{
     input.clear();
     reader.read_line(&mut input).unwrap();
     let  blocks:Vec<u32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();    
     matrix.push(blocks)
  }

  for target in 0..257{
    let mut max_target= 0;
    let mut min_target=0;

    for i in 0..n{
      for j in 0..m{
        if matrix[i as usize][j as usize]>=target{
          max_target+=matrix[i as usize][j as usize]-target
        }else{
          min_target += target - matrix[i as usize][j as usize]
        }
      }
    }
    if max_target + b >=min_target{
      if min_target+(max_target*2)<=t{
        t= min_target+(max_target*2);
        idx= target;
      }
    }
  }

  writeln!(writer,"{} {}",t,idx).unwrap();
  writer.flush().unwrap();
}