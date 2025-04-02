struct ArrayStack<T, const N: usize>
where
    T: Copy,
{
    // 1-indexed とするため、容量は N+1 個の Option<T> を用意（0 番目は未使用）
    data: [Option<T>; N + 1],
    top: usize, // 初期値 0 はスタックが空であることを示す
}

impl<T, const N: usize> ArrayStack<T, N>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self {
            data: [None; N + 1],
            top: 0,
        }
    }

    // PUSH(x):
    //   top <- top + 1
    //   S[top] <- x
    pub fn push(&mut self, x: T) -> Result<(), &str> {
        if self.top >= N {
            Err("Stack Overflow")
        } else {
            self.top += 1;
            self.data[self.top] = Some(x);
            Ok(())
        }
    }

    // POP:
    //   top <- top - 1
    //   return S[top+1]
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            let ret = self.data[self.top];
            self.top -= 1;
            ret
        }
    }

    // スタックのトップの要素を返す（削除はしない）
    pub fn peek(&self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.data[self.top]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
}

// fn main() {
//     // 例: 整数型のスタック、容量 5
//     let mut stack: ArrayStack<i32, 5> = ArrayStack::new();

//     // PUSH 操作
//     stack.push(10).unwrap();
//     stack.push(20).unwrap();
//     stack.push(30).unwrap();

//     println!("スタックのトップ: {:?}", stack.peek()); // 30

//     // POP 操作
//     println!("pop -> {:?}", stack.pop()); // 30
//     println!("pop -> {:?}", stack.pop()); // 20
//     println!("pop -> {:?}", stack.pop()); // 10

//     println!("スタックは空ですか？ {}", stack.is_empty());
// }
