use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::iter::Iterator;
//최대 랜선수
const N_MAX: usize = 1_000_000;
//최대길이
const K_MAX: usize = 10_000;
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
 let mut inputs= input.trim().split_whitespace().map(|x|x.parse::<u64>().unwrap());
 //랜선의 갯수k

 //필요한 랜선의 개수n
 let (k, n) = (inputs.next().unwrap() as usize, inputs.next().unwrap());

let mut nums= [0u64;K_MAX];

//각 랜선의 길이
for num in nums.iter_mut().take(k){
  input.clear();
  reader.read_line(&mut input).unwrap();
  //랜선의길이
  *num =input.trim().parse().unwrap();
  
}
let sum: u64 = nums.iter().take(k).sum();

//시작1
let mut start= 1u64;
//끝
let mut end= sum/n+1;
//최개길이
let mut max_len=u64::MIN;
//레이블 사용
//이진탐색
'outer: while start < end {
  //중간길이
  let middle_len = start + (end - start) / 2;
  //남은길이
  let leftover = sum - (middle_len * n);
  //남은 값들의합
  let mut leftover_sum = 0u64;
  for num in nums.iter().take(k) {
    //각 랜선의 길이를 이용하여 남은 값들의 합을 계산
    leftover_sum += *num % middle_len;
    //초과하면
    if leftover_sum > leftover {
        end = middle_len;
        //즉시 다음 레이블로 이동
        continue 'outer;
    }
   

   } 
    //max_len 값을 업데이트
    max_len = u64::max(middle_len, max_len);
    //다음 반복을 위해 start를 조정
    start = middle_len + 1;
}

writeln!(writer,"{max_len}").unwrap();

}