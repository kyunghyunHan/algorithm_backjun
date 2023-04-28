use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input == "#"{
            break;;
        }

        let mut count=0;
        for i in  input.chars(){
            if i=='a' ||i=='e' ||i=='i' ||i=='o' ||i=='u'||i=='A' ||i=='E' ||i=='I' ||i=='O' ||i=='U'  {

                count+=1  
            }
          
        }

        println!("{}",count);

        
    }
}
