use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::HashMap;
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
 loop{
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  let n= input.trim();
  
  let hv = HashMap::from([
    ("CU".to_string(), "see you".to_string()),
    (":-)".to_string(), "I’m happy".to_string()),
    (":-(".to_string(), "I’m unhappy".to_string()),
    (";-)".to_string(), "wink".to_string()),
    (":-P".to_string(), "stick out my tongue".to_string()),
    ("(~.~)".to_string(), "sleepy".to_string()),
    ("TA".to_string(), "totally awesome".to_string()),
    ("CCC".to_string(), "Canadian Computing Competition".to_string()),
    ("CUZ".to_string(), "because".to_string()),
    ("TY".to_string(), "thank-you".to_string()),
    ("YW".to_string(), "you’re welcome".to_string()),
    ("TTYL".to_string(), "talk to you later".to_string()),
    ]);


    let s=  hv.get(&n.to_string());
    match s {
   Some(s)=>{if s==&("talk to you later".to_string()){
    writeln!(writer,"{}",s).unwrap();
    break;
   }else{
    writeln!(writer,"{}",s).unwrap();

   }}
   None => {
    write!(writer, "{}", input).unwrap();
}
   }
    
 }

}