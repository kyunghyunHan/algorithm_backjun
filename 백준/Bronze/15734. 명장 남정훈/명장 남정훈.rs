use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());

    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let mut writer= BufWriter::new(stdout().lock());
    let mut l= nums.next().unwrap();
    let mut r= nums.next().unwrap();
    let mut a= nums.next().unwrap();
    let total= l+r+a;

    if l<r{
        let diff= usize::min(r-l,a);
        l+=diff;
        a-=diff
    }  else{
        let diff= usize::min(l-r,a);
        r += diff;
        a -= diff;
    }

    if r==l {
writeln!(writer,"{}",total-(a & 1)).unwrap();
    }else{
        writeln!(writer,"{}",total-(r-l)).unwrap();
    }


}
