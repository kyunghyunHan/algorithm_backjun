use core::num;
use std::io::{BufReader,BufRead,BufWriter,Write,stdout,stdin};

fn main(){
    let mut gold= [0;1001];
    let mut sliver= [0;1001];
    let mut bronze= [0;1001];
    let mut res= 0;
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
let  nums: Vec<usize>= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

let n= nums[0];
let k= nums[1];
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let  nums2: Vec<usize>= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        gold[nums2[0]]= nums2[1];
        sliver[nums2[0]]= nums2[2];
        bronze[nums2[0]]= nums2[3];
    }

    for i in 1..=n{
        if gold[i]>gold[k]{
            res+=1;
        }else if gold[i]==gold[k]{
            if sliver[i]>sliver[k]{
                res+=1;
            }else if sliver[i]==sliver[k]{
                if bronze[i]>bronze[k]{
                    res+=1;
                }
            }
        }
    }
    println!("{}",res+1);
}