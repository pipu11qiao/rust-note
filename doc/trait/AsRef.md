# rust 中的 AsRef 特征介绍

在学习 std::path 的过程中查看了 std::path::Path 的 push 方法

```rust
    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        self._push(path.as_ref())
    }
```

上述代码的可以看出，push()方法接收一个 path 参数，该参数的类型是实现了 AsRef<Path> 这个特征的类型。
这里的 AsRef<Path>是一种带有约束的特征，点击改特征继续查看：

```rust
pub trait AsRef<T: ?Sized> {
    /// Converts this type into a shared reference of the (usually inferred) input type.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_ref(&self) -> &T;
}
```

其中<T:?Sized>标识特征 T 在编译器就已经知道内存大小，但是也不做强势约束
可以看到该特征包含了 as_ref 方法返回一个 T 泛型的类型数据的引用&T。

### 文档的翻译：

用来做一个引用类型到引用类型的转换，对应&slef -&T
如果想做一个有一些消耗的转换推荐使用 From 特征(也是一个很重要的特征，用来做类型转换),这里的一些消耗是啥意思？？？

使用 T 这个特征约束用来将提供的数据转为 T 类型的引用，只要改类型可以转为 T 类型。

### 例如：

创建一个函数，参数是 AsRef<str>特征的，表明我们接收的参数都可以通过转换为&str 类型(这里的转换为调用参数的 as_ref 方法)。

```rust
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}
let s = "hello";
is_hello(s);
let s = "hello".to_string();
is_hello(s);
```
is_hello 接受一个可以转为&str的类型数据s,因为Sting和&str类型都实现了AsRef<str>所以参数这两种类型都可以

### 在push() 方法的运用

可以在push()方法中加深一些理解

```rust
    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        self._push(path.as_ref())
    }
```
这里的path参数是可以转换为&Path的类型数据。在PathBuf文件中我们找到了它为那些类型添加了该特征

```rust
impl AsRef<Path> for str {
    #[inline]
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}
impl AsRef<Path> for String {
    #[inline]
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}
impl AsRef<Path> for OsStr {
}
impl AsRef<Path> for OsString {
}
// Path PathBuf 等等

```
这些特征的转换方法as_ref中能将对应的类型转为Path类型

通过添加这个特征，我们就可以在调用push方法中传入str,String,OsStr,OsString,Path,PathBuf,等等这些类型的数据

### 这个转换as_ref()方法是在什么时候被调用的？？

猜测是在编译后由编译器实现的，自动添加as_ref的代码转换类型。这应该也是特征的机制吧