use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

/// 再帰的な関数のメモ化を実現するためのジェネリックな構造体
pub struct Memoize<F, A, R>
where
    A: Eq + Hash + Clone,
    R: Clone,
{
    // 対象となる関数。第一引数に自身への参照をとることで、再帰呼び出し時にもメモ化機構を利用できるようにする。
    f: F,
    // 引数と計算結果の対応を保持するキャッシュ。Mutexで保護してスレッドセーフにしている。
    cache: Mutex<HashMap<A, R>>,
}

impl<F, A, R> Memoize<F, A, R>
where
    // 関数は、自身（&Memoize<F, A, R>）と引数Aを受け取り、結果Rを返す
    F: Fn(&Memoize<F, A, R>, A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    /// 新たなMemoize構造体を生成する
    pub fn new(f: F) -> Self {
        Self {
            f,
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// 引数 `arg` に対する結果を返す。キャッシュに存在すればそれを返し、なければ計算後にキャッシュへ保存する
    pub fn call(&self, arg: A) -> R {
        {
            // キャッシュをロックして、すでに計算済みかどうかチェック
            let cache = self.cache.lock().unwrap();
            if let Some(result) = cache.get(&arg) {
                return result.clone();
            }
        }
        // キャッシュに存在しなければ、関数を実行
        let result = (self.f)(self, arg.clone());
        // 結果をキャッシュに保存
        self.cache.lock().unwrap().insert(arg, result.clone());
        result
    }
}

fn main() {
    // フィボナッチ数を計算する再帰的関数をメモ化
    let fib = Memoize::new(|memo: &Memoize<_, u64, u64>, n: u64| -> u64 {
        if n < 2 {
            n
        } else {
            memo.call(n - 1) + memo.call(n - 2)
        }
    });

    // テストとして、0から19までのフィボナッチ数を計算して表示
    for n in 0..20 {
        println!("fib({}) = {}", n, fib.call(n));
    }
}
