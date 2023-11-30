use std::io ::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
use std::collections::HashMap;
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t:usize= input.trim().parse().unwrap();
    
    for i in 0..t{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let n:usize= input.trim().parse().unwrap();
        let mut hm: HashMap<String, usize>=HashMap::new();
        for _ in 0..n{
            input.clear();
            reader.read_line(&mut input).unwrap();
            let  s:Vec<String>= input.trim().split_whitespace().map(|x|x.to_string()).collect();
             //키가 존재하지 않으면 0을 삽입
            *hm.entry(s[1].clone()).or_insert(0) += 1;
        }
        let mut ans: usize = 1;
        for &val in hm.values() {
            ans *= val + 1; // 해당 종류는 안 입는 경우 포함
        }
        writeln!(writer,"{}", ans- 1).unwrap();

    }
    writer.flush().unwrap();
}