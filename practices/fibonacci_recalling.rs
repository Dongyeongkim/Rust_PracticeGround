fn main(){
    let startpoint1:i64 = 1;
    let startpoint2:i64 = 1;
    let count_num:i64 = 10;
    print!("{} ", startpoint1);
    fibonacci(startpoint1, startpoint2, count_num);
}



fn fibonacci(start1:i64, start2:i64, count:i64) {
    print!("{} ",start2);
    let temp = start2;
    let start2 = start1+start2;
    let start1 = temp;
    let count = count - 1;

    if count == 0 {
        print!("{} ", start2);
    }
    else{
        Fibonacci(start1, start2, count);
    }
}
