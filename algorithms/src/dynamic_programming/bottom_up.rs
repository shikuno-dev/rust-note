fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    // フィボナッチ数列の計算では、直前の2つの値のみが必要
    let (mut a, mut b) = (0, 1);

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

fn main() {
    let n = 10;
    println!("fib({}) = {}", n, fib(n)); // 出力: fib(10) = 55
}
