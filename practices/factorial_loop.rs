fn main(){

    let n:i64 = 20;
    factorial(n);


}

fn factorial(num:i64) {
    let mut temp: i64 = 1;
    if num == 0 {
        print!("{}", temp);
    }
    for i in 1..num+1 {
        temp*=i;
    }
    print!("{}", temp);
}
