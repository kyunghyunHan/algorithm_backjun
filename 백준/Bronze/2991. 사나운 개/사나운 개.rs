use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main()-> std::io::Result<()>{
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
    let abcd= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    input.clear();
     reader.read_line(&mut input).unwrap();
    let pmn= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();

    for i in pmn{
        let mut atacked= 0;
        if 0<i % (abcd[0]+abcd[1]) && i % (abcd[0]+abcd[1]) <=abcd[0]{
            atacked+=1;
        }
        if 0<i % (abcd[2]+abcd[3]) && i % (abcd[2]+abcd[3]) <=abcd[2]{
            atacked+=1;
        }
        writeln!(writer,"{}",atacked).unwrap();
    }
    Ok(())
}

