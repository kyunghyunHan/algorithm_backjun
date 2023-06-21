use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let mut writer= BufWriter::new(stdout().lock());
    let n:usize= input.trim().parse().unwrap();
    let mut ans= 0;
    
    let mut start= 1;
    let mut end= n;
    while start<=end{
        let mut mid= (start+end)/2;

        if mid*(mid+1)/2<=n{
ans= mid;
start= mid+1;
        }else{
            end= mid-1;
        }
    }
    writeln!(writer,"{}",ans).unwrap();
}
/*이진탐색
1.1부터 mid까지의합 mid*(mid+1)/2
2.그 값이 n보다 크면 mid 값이 더 작아져야 하므로 end를 줄인다.
3.그 값이 n보다 작거나 같으면 mid값이 더 커져야 하므로 start를 키운다

* 이진탐색은 원소가 정렬되어 있어야 하고 랜덤엑세스가가능할떄 적용가능
 */