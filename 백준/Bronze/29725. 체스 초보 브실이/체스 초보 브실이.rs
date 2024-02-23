use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};
fn main(){
    let mut _reader= BufReader::new(stdin().lock());
    let mut _writer= BufWriter::new(stdout().lock());
    let mut _wite_team= 0;
    let mut _black_team= 0;
    for _i in 0..8{
        let mut _input = String::new();
        _reader.read_line(&mut _input).unwrap();
        let _s= _input.trim();
        for  _si in _s.chars(){
             match _si {
                 'K'=>{
                    _wite_team+=0;
                 },
                 'k'=>{
                    _black_team+=0;
                 },
                 'P'=>{
                    _wite_team+=1;
                 },
                 'p'=>{
                    _black_team+=1;
                 },
                 'N'=>{
                    _wite_team+=3;
                 },
                 'n'=>{
                    _black_team+=3;
                 },
                 'B'=>{
                    _wite_team+=3;
                 },
                 'b'=>{
                    _black_team+=3;
                 },
                 'R'=>{
                    _wite_team+=5;
                 },
                 'r'=>{
                    _black_team+=5;
                 },
                 'Q'=>{
                    _wite_team+=9;
                 },
                 'q'=>{
                    _black_team+=9;
                 },
                 _=>{
                    _wite_team+=0;
                 }
             }

        }
         
    }
    writeln!(_writer,"{}",_wite_team-_black_team).unwrap();
    _writer.flush().unwrap();
}
