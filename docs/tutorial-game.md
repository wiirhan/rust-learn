# 猜数字游戏

### 说明

程序将会随机生成一个 1 到 100 之间的随机整数。接着它会请玩家猜一个数并输入，然后提示猜测是大了还是小了。如果猜对了，它会打印祝贺信息并退出。

### 代码

1. 使用宏打印了两行语句
2. 创建一个可变的字符串变量guess
3. 从标准库中引入了输入/输出功能
4. 调用 read_line 方法从标准输入句柄获取用户输入
5. 处理异常，如果有异常则打印

#### println!

打印字符串的宏

#### crate

crate是一个Rust代码包。现在正在构建的项目是一个二进制crate，它生成一个可执行文件。
