use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};


fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
   

    let n: usize =input.trim().parse().unwrap();

  
      

        match n {
            0 => {
                println!(" * * *");
                println!("*     *");
                println!("*     *");
                println!("*     *");
                println!("");
                println!("*     *");
                println!("*     *");
                println!("*     *");
                println!(" * * *");
            }
            1 => {
                println!("");
                println!("      *");
                println!("      *");
                println!("      *");
                println!("");
                println!("      *");
                println!("      *");
                println!("      *");
            }
            2 => {
                println!(" * * *");
                println!("      *");
                println!("      *");
                println!("      *");
                println!(" * * *");
                println!("*");
                println!("*");
                println!("*");
                println!(" * * *");
            }
            3 => {
                println!(" * * *");
                println!("      *");
                println!("      *");
                println!("      *");
                println!(" * * *");
                println!("      *");
                println!("      *");
                println!("      *");
                println!(" * * *");
            }
            4 => {
                println!("");
                println!("*     *");
                println!("*     *");
                println!("*     *");
                println!(" * * *");
                println!("      *");
                println!("      *");
                println!("      *");
            }
            5 => {
                println!(" * * *");
                println!("*");
                println!("*");
                println!("*");
                println!(" * * *");
                println!("      *");
                println!("      *");
                println!("      *");
                println!(" * * *");
            }
            6 => {
                println!(" * * *");
                println!("*");
                println!("*");
                println!("*");
                println!(" * * *");
                println!("*     *");
                println!("*     *");
                println!("*     *");
                println!(" * * *");
            }
            7 => {
                println!(" * * *");
                println!("      *");
                println!("      *");
                println!("      *");
                println!("");
                println!("      *");
                println!("      *");
                println!("      *");
            }
            8 => {
                println!(" * * *");
                println!("*     *");
                println!("*     *");
                println!("*     *");
                println!(" * * *");
                println!("*     *");
                println!("*     *");
                println!("*     *");
                println!(" * * *");
            }
            9 => {
                println!(" * * *");
                println!("*     *");
                println!("*     *");
                println!("*     *");
                println!(" * * *");
                println!("      *");
                println!("      *");
                println!("      *");
                println!(" * * *");
            }
            _ => println!("Invalid input"),
        }
    
}
