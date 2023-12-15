use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut  input= String::new();
    reader.read_line(&mut input).unwrap();
    let n:usize= input.trim().parse().unwrap();
    let mut tree= Vec::new();
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();

        let inputs= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
        tree.push(inputs);
    }
    let mut result:Vec<usize>=vec![];

    solution(0, 0, n, &mut tree,&mut  writer,&mut result);
    let count_0 = result.iter().filter(|&&x| x == 0).count();
    let count_1 = result.iter().filter(|&&x| x == 1).count();
    writeln!(writer,"{}",count_0).unwrap();
    writeln!(writer,"{}",count_1).unwrap();

}

fn solution(x:usize,y:usize,n:usize,tree:&mut Vec<Vec<usize>>,writer:&mut BufWriter<std::io::StdoutLock<'_>>,result:&mut Vec<usize>){
    let  color= tree[x][y];
    for i in x..x+n{
        for j in y..y+n{
            if color !=tree[i][j]{
                solution(x, y, n/2, tree, writer,result);
                solution(x, y+n/2, n/2, tree, writer,result);
                solution(x+n/2, y, n/2, tree, writer,result);
                solution(x+n/2, y+n/2, n/2, tree, writer,result);
                return
            }
        }
    }
    if color==0{
       result.push(0);
    }else{
        result.push(1);
    }

}