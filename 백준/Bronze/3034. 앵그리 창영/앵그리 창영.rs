use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main()-> std::io::Result<()>{
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
    let nwh = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    for i in 0..nwh[0]{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let leng:i32 = input.trim().parse().unwrap();
        let mut m = ((nwh[1]* nwh[1])+ (nwh[2]*nwh[2])) as f32;
        m = m.powf(0.5);
        if m>=leng as f32{
            writeln!(writer,"{}","DA").unwrap();
        }else{
            writeln!(writer,"{}","NE").unwrap();
        }

    }
    writer.flush().unwrap();

    
    Ok(())
}
