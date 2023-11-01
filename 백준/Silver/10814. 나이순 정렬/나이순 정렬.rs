use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    //온라인 저지 회원의 수  
    let n:i32= input.trim().parse().unwrap();
    let mut users: Vec<(i32,String,i32)>= Vec::new();
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut v= input.trim().split_whitespace();
        //나이는 1도다 크거나 같으며 200보다 작거나 같은 숫자
        let age:i32= v.next().unwrap().parse().unwrap();
        //이름은 대소문자
        let name= v.next().unwrap().trim().to_string();
        users.push((age,name,i));
    }


    users.sort_by(|a,b|{
    //a와 b의 나이를 비교
    //같으면 인덱스를 비교하여 정렬
  if a.0==b.0{
    a.2.cmp(&b.2)
  }else{
    //나이가 다르면 나이로 비교하여 정렬
    a.0.cmp(&b.0)
  }
    });

    for (a, b, _) in users {
        writeln!(writer, "{} {}", a, b).unwrap();
    }

    writer.flush().unwrap();
}