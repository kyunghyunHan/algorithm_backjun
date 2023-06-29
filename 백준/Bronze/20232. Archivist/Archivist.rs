use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::HashMap;
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  let n: usize = input.trim().parse().expect("Invalid input");

let mut archivist: HashMap<usize, String> = HashMap::from([
  (1995, "ITMO".to_string()),
  (1996, "SPbSU".to_string()),
  (1997, "SPbSU".to_string()),
  (1998, "ITMO".to_string()),
  (1999, "ITMO".to_string()), 
  (2000, "SPbSU".to_string()), 
  (2001, "ITMO".to_string()), 
  (2002, "ITMO".to_string()),
  (2003, "ITMO".to_string()), 
  (2004, "ITMO".to_string()),
  (2005, "ITMO".to_string()),
  (2006, "PetrSU, ITMO".to_string()), 
  (2007, "SPbSU".to_string()),
  (2008, "SPbSU".to_string()),
  (2009, "ITMO".to_string()),
  (2010, "ITMO".to_string()),
  (2011, "ITMO".to_string()),
  (2012, "ITMO".to_string()),
  (2013, "SPbSU".to_string()),
  (2014, "ITMO".to_string()),
  (2015, "ITMO".to_string()),
  (2016, "ITMO".to_string()),
  (2017, "ITMO".to_string()), 
  (2018, "SPbSU".to_string()),
  (2019, "ITMO".to_string()),
]);

let score= archivist.get(&n);
match score {
  Some(s) => writeln!(writer, "{}", s).expect("Failed to write output"),
  None => writeln!(writer, "Score not found").expect("Failed to write output"),
}

}