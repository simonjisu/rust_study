fn main() {
    let x = 5;
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        if x == 5 {
            println!{"{}", x};
        }
    }
}