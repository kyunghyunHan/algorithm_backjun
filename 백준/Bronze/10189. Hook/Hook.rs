use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){



let mut writer= BufWriter::new(stdout().lock());



    writeln!(writer,"{}","#  # #### #### #  #").unwrap();
    writeln!(writer,"{}","#### #  # #  # # #").unwrap();
    writeln!(writer,"{}","#### #  # #  # # #").unwrap();
    writeln!(writer,"{}","#  # #### #### #  #").unwrap();


}
