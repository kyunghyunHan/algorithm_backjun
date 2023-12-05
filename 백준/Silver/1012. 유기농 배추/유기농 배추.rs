use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout}, array};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  //test 
  let t= input.trim().parse().unwrap();
  for _ in 0..t{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut inputs= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let m = inputs.next().unwrap();
    let n = inputs.next().unwrap();
    let k = inputs.next().unwrap();
    //배추의 개수만큼

    let mut array = vec![vec![0; n ]; m ];
    for _ in 0..k{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let coords: Vec<i32> = input
      .split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect();

      let x = coords[0] as usize;
      let y = coords[1] as usize;
      array[x][y] = 1;
    }
    //그래프를 순회하면서 탐색
    let mut total = 0;
    for i in 0..m {
        for j in 0..n {
          //배추가 심어져 잇으면
            if dfs(i as i32, j as i32, &mut array, m, n) {
              //흰지렁이
                total += 1;
            }
        }
    }
    writeln!(writer,"{}", total).unwrap();

  }
  writer.flush().unwrap();
}


fn dfs(x: i32, y: i32, array: &mut Vec<Vec<i32>>, M: usize, N: usize) -> bool {
  if x < 0 || y < 0 || x as usize >= M || y as usize >= N {
      return false;
  }
  //배추가 심어져 잇으면
  if array[x as usize][y as usize] == 1 {
    //0으로바꾸어 방문 표시
      array[x as usize][y as usize] = 0;
      dfs(x - 1, y, array, M, N);
      dfs(x + 1, y, array, M, N);
      dfs(x, y + 1, array, M, N);
      dfs(x, y - 1, array, M, N);
      return true;
  }
  //배추가 심어져 있지 않는 경우
  false
}