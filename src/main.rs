use std::io;

fn main() {
    // let length:i32=7;
    fn triangle(){
    println!("Please input length :");

    let mut length = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");
        let length: i32 = length.trim().parse()
        .expect("Please type a number!");

    let space_max:i32 = length/2;
    let mut space:i32 = space_max;
    let mut count = 1;

    while count <=length {
        let mut aster = 1;
        let mut sp=1;
        while sp<= space {
            print!(" ");
            sp+=1;
        }
        while aster <= count{
            print!("*");
            aster+=1;
        }
        print!("\n");
        space =space -1;
        count +=2;  
    } 
    }

    triangle();
}

