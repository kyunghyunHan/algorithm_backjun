use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main() {
     let mut reader= BufReader::new(stdin().lock());
     let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let a:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let x:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let b:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let y:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let t:i32= input.trim().parse().unwrap();

    let a_bicycle= if t>30{
         a+ (t-30)*x*21

    }else{
      a
    };

    let b_bicycle= if t>45{
        b+ (t-45)*y*21

   }else{
     b
   };

   writeln!(writer,"{} {}",a_bicycle,b_bicycle).unwrap();
    writer.flush().unwrap();
}
