#[allow(unused_variables)]
fn main() {
    // let mut a = -10;
    // a += 2;
    // println!("{}", a);
    // let y = 32;
    // let (x, y) = (2, 3);
    // println!("The values of x and y are {}, {}", x, y);
    // let f = false;
    // if !f {
    //     println!("Success");
    // }

    print();
    println!("Failed");
}

fn print() -> ! {
    //print();
    panic!();
}