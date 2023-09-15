use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
     
     reader.read_line(&mut input).unwrap();

     let nums= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
     let n= nums[1];
     let m = nums[0];


     let mut map = vec![vec![0; 101]; 101];
     let mut ans = 0;
     let mut cnt = 1;
     
     let mut d = 0;
     let mut r = 0;
     let mut c = 0;
     map[r][c] = 1;
 
     while cnt < m * n {
         let nr = (r as i32 + dr[d]) as usize;
         let nc = (c as i32 + dc[d]) as usize;
 
         if nr >= 0 && nr < m as usize && nc >= 0 && nc < n as usize && map[nr][nc] == 0 {
             map[nr][nc] = 1;
             cnt += 1;
             r = nr;
             c = nc;
         } else {
             ans += 1;
             d = (d + 1) % 4;
         }
     }
 
     println!("{}", ans);
 }
 
 const dr: [i32; 4] = [0, 1, 0, -1];
 const dc: [i32; 4] = [1, 0, -1, 0];