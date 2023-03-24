use std::io;

fn main(){
  loop{

    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();

    let v:Vec<i32>= input_a.split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect();

    if v[0]==0||v[1]==0{
       break;
    }else if v[0] <= v[1]{
         if v[1]%v[0]==0 {
        println!("factor");
         }else{
            println!("neither");
         }
    }else if v[0]>v[1]{
        if v[0]%v[1]==0 {
            println!("multiple");
             }else{
                println!("neither");
             }
    }
 
  }
   

}


