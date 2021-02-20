fn main(){

    let n:i64 = 30;
    fibonacci(n);


}

fn fibonacci(num:i64) {
    let mut startpoint1 = 1;
    let mut startpoint2=1;
    for _ in 1..num+1 {
        print!("{} ", startpoint1);
        let temp: i64 = startpoint2;
        startpoint2 = startpoint1 + startpoint2;
        startpoint1 = temp;
    }
}
