# path 模块
跨平台路径操作,该模块提供两种类型PathBuf和Path, PathBuf 和 Path的关系类似于String和str的关系。这些类型是围绕OsString和OsStr的薄包装类型。

通过遍历Path上的components方法返回的结构及可以将路径解析为Component。Component大致对应于路径分隔符（/或\\)之间的字符串。


## 使用简单

要解析路径，可以从str切片创建Path切片，并使用它的方法解决对应路径需求
```rust
use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let path = Path::new("/tmp/foo/bar.txt");

    let parent = path.parent();
    assert_eq!(parent, Some(Path::new("/tmp/foo")));

    let file_name = path.file_name();
    assert_eq!(file_name, Some(OsStr::new("bar.txt")));

    let extension = path.extension();
    assert_eq!(extension,Some(OsStr::new("txt")));
}

```
如果要构建或修改路径，请使用PathBuf

```rust
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from("/path/hello");
    path.push("Study");
    path.push("day1");
    path.set_extension("txt");

    print!("{}",path.to_str().unwrap());///path/hello/Study/day1.txt
}

```

path::MAIN_SEPARATOR 当前平台的路径组件的主要分隔符
is_separator 确定字符是否为当前平台允许的路径分隔符之一


# Path 路径切片

此类型支持需要检查路径的操作，包括将路径分为其各个组成部分，提取文件名，确定路径是否为绝对路径。

这是未定义的大小的类型，表示必须始终在&或Box之类的指针后面使用它。

* new
* as_os_str -> &OsStr
* to_str -> Option<&str> 因为需要验证ut8的有效性
* to_string_lossy -> Cow<'_, str>
* to_path_buf -> PathBuf
* is_absolute -> bool
* is_relative -> bool
* has_root -> bool
* parent -> Opation<&Path> 不包含最终组成部分的Path
* ancestors -> Ancestors<'_> 在 Path 及其祖先上生成一个迭代器。
* file_name -> Option<&OsStr> 如果是文件则是文件名(带后缀名的),如果是目录则是目录名称
* strip_prefix<P>(&self,base:P)->Result<&Path,stripPrefixError>
where P: AsRef<Path> 返回链接到base时产生self的路径 ，以base为前缀，中间找到&self时产生的路径
* start_with<P: AsRef<Path>>(&self, base: P) -> bool
* end_with<P: AsRef<Path>>(&self, base: P) -> bool
* file_stem -> Option<&OsStr> 提取 self.file_name 的茎 (non-extension) 部分
* file_prefix -> Option<&OsStr> 文件名(不包含扩展的)
* extension -> Option<&OsStr> 扩展名
* join<P: AsRef<Path>>(&self, path: P) -> PathBuf
* with_file_name<S: AsRef<OsStr>>(&self, file_name: S) -> PathBuf
* with_extension<S: AsRef<OsStr>>(&self, extension: S) -> PathBuf
* components ->  Components<'_>
生成路径的Component上迭代器。
```rust
use std::path::{Path, Component};
use std::ffi::OsStr;

let mut components = Path::new("/tmp/foo.txt").components();

assert_eq!(components.next(), Some(Component::RootDir));
assert_eq!(components.next(), Some(Component::Normal(OsStr::new("tmp"))));
assert_eq!(components.next(), Some(Component::Normal(OsStr::new("foo.txt"))));
assert_eq!(components.next(), None)

```
* iter -> Iter<'_> 在视为 OsStr slice 的路径的组件上生成迭代器。
* display -> Display<'_>
* metadata -> Reault<Metadata> fs::metadata的别名，该函数将遍历符号链接以查询目标文件的信息。??
* symlink_meatadata -> Result<Meatdata> 查询有关文件的元数据，而无需遵循符号链接。??
*  canonicalize(&self) -> Result<PathBuf> 返回路径的规范，绝对形式，所有中间组件均已标准化，符号链接已解析。
```rust
use std::path::{Path, PathBuf};

let path = Path::new("/foo/test/../test/bar.rs");
assert_eq!(path.canonicalize().unwrap(), PathBuf::from("/foo/test/bar.rs"));
```
* read_link(&self) -> Result<PathBuf> 读取符号链接，返回链接指向的文件。是fs::read_link的别名
* read_dir(&self) -> Result<ReadDir> 返回目录中条目的迭代器。这是 fs::read_dir 的别名。
* exists() -> bool 如果路径指向现有实体，则返回 true。 该函数将遍历符号链接以查询有关目标文件的信息。 如果您无法访问文件的元数据，例如 由于权限错误或损坏的符号链接，这将返回 false。
* try_exists(&self) -> Result<bool>
* is_file -> bool 如果路径在磁盘上并且指向常规文件，则返回 true。 该函数将遍历符号链接以查询有关目标文件的信息。 如果您无法访问文件的元数据，例如由于权限错误或损坏的符号链接，这将返回 false。
* is_dir -> bool 如果路径在磁盘上并且指向目录，则返回 true。 该函数将遍历符号链接以查询有关目标文件的信息。 如果您无法访问文件的元数据，例如 由于权限错误或损坏的符号链接，这将返回 false。

* is_symlink ->bool 如果路径存在于磁盘上并且指向符号链接，则返回 true。 这个函数不会遍历符号链接。 如果符号链接损坏，这也将返回 true。 如果您无法访问包含该文件的目录，例如，由于权限错误，这将返回 false。

* into_path_buf(self: Box<Path>) -> PathBuf 无需复制或分配即可将 Box<Path> 转换为 PathBuf。

# PathBuf

拥有的可变路径(类似于String)
这种类型提供了push和set_extension之类的方法，这些方法会改变路径。它还实现了Deref到Path,这意味着Path切片上的所有方法也可以在PathBuf值上使用。

```rust
use std::path::PathBuf;

let mut path = PathBuf::new();

path.push(r"C:\");
path.push("windows");
path.push("system32");

path.set_extension("dll");
let path: PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
let path = PathBuf::from(r"C:\windows\system32.dll");

```


* new -> PathBuf
* with_capacity(capaciry:unsize) -> PathBuf 创建具有给定容量的新 PathBuf，用于创建内部 OsString。 请参见在 OsString 上定义的 with_capacity。
* as_path -> &Path
* push<P: AsRef<Path>>(&mut self, path: P)
* pop(&mut self) -> bool 将parent弹出，如果是根路径不执行操作
* set_file_name<S:AsRef<OsStr>>(&mut self, filename:S)
* set_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> bool
* into_os_string(self)->OsString
* into_boxed_path(self) -> Box<Path>
* capacity(&self) -> usize
* clear(&mutself) -> 在 OsString 的底层实例上调用 clear。
* reserve (&mut self, additional: usize) 在 OsString 的底层实例上调用 reserve_exact。
* shrink_to_fit(&mut self)在 OsString 的底层实例上调用 shrink_to_fit。
* shrink_to(&mut self, min_capacity: usize) 在 OsString 的底层实例上调用 shrink_to。
