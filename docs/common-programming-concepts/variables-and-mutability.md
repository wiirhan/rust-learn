# 变量和可变性

默认情况下变量是**不可变的**。Rust鼓励选用不可变性。

当变量不可变时，表示一旦一个值绑定到一个变量名后，就不能再更改该值了。

我们可以通过在变量名前面加上**mut**使得它们可变。

### 变量和常量之间的差异

常量也是绑定到一个常量名且不允许更改的值。

#### 差异

1. 常量不允许使用**mut**。常量不仅仅默认不可变，而且自始至终不可变。
2. 常量使用**const**关键字而不是**let**关键字来声明，并且值的类型必须标注。
3. 常量可以在任意作用域内声明，包括全局作用域，这对于代码中很多部分都需要知道一个值的情况特别有用。
4. 常量只能设置为常量表达式，而不能是函数调用的结果或是只能在运行时计算得到的值。

#### 约定

Rust 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词，另外对数字字面量可插入下划线以提高可读性

```rust
const MAX_POINTS: u32 = 100_000;
```

### 变量遮蔽

可以声明和前面变量具有相同名称的新变量。

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

```

mut 和变量遮蔽之间的另一个区别是，因为我们在再次使用 let 关键字时有效地创建了一个新的变量，所以我们可以改变值的类型，但重复使用相同的名称。

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    let mut spaces = "   ";
    spaces = spaces.len();
}

```