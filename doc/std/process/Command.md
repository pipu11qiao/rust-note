 # std::process::Command 
 进程生成器，提供对如何生成新进程的细粒度控制。
 
可以使用Command::new(program)生成默认配置，其中program提供了要执行的程序路径。其他生成器方法允许在生成之前更改配置，例如，通过添加参数：

Command 可以重用以生成多个进程。构建器方法无需立即是进程sapwan即可更改命令。

```rust
use std::process::Command;


fn main(){

let mut echo_hello = Command::new("sh");
echo_hello.arg("-c")
          .arg("echo hello");
let hello_1 = echo_hello.output().expect("failed to execute process");
let hello_2 = echo_hello.output().expect("failed to execute process");

}
```
也可以在生成进程之后调用构建子方法，然后使用修改后的设置spawn新建一个进程。


# 方法

* new<S: AsRef<OsStr>>(program: S) -> Command

* arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command
 添加参数已传递给程序。
 每次只能传递一个参数。因此，而不是
 > .arg("-c /path/to/repo")

 注意参数不是通过shell传递的，而是按字面意义提供给程序的，这意味着shell语法,例如引号、转义字符、单词拆分、全局模式、替换等，没有效果

 * args<I, S>(&mut self, args: I) -> &mut Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>, 

* env<K, V>(&mut self, key: K, val: V) -> &mut Command
where
    K: AsRef<OsStr>,
    V: AsRef<OsStr>, 
    插入或更新环境变量映射。

请注意，环境变量名称在 Windows 上不区分大小写 (但保留大小写)，在所有其他平台上则区分大小写。

* envs<I, K, V>(&mut self, vars: I) -> &mut Command
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>, 
* env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Command
* env_clear(&mut self) -> &mut Command
* current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Command 设置子进程的工作目录。
* stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command
子进程的标准输入 (stdin) 句柄的配置。
与 spawn 或 status 一起使用时默认为 inherit，与 output 一起使用时默认为 piped。
* stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command
子进程的标准错误 (stderr) 句柄的配置。
与 spawn 或 status 一起使用时默认为 inherit，与 output 一起使用时默认为 piped。
* stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command
子进程的标准错误 (stderr) 句柄的配置。
与 spawn 或 status 一起使用时默认为 inherit，与 output 一起使用时默认为 piped。
* spawn(&mut self) -> Result<Child>
将命令作为子进程执行，并返回其句柄。
默认情况下，stdin、stdout 和 stderr 都是从父级继承的。
* output(&mut self) -> Result<Output>
将命令作为子进程执行，等待其完成并收集所有输出。
默认情况下，将捕获 stdout 和 stderr (并用于提供结果输出)。 Stdin 不是从父级继承的，子进程尝试从 stdin 流中进行读取的任何尝试都将导致该流立即关闭。
* status(&mut self) -> Result<ExitStatus>
将命令作为子进程执行，等待其完成并收集其状态。
默认情况下，stdin、stdout 和 stderr 都是从父级继承的。
*  get_program(&self) -> &OsStr
* get_args(&self) -> CommandArgs<'_>ⓘ
* get_envs(&self) -> CommandEnvs<'_>ⓘ
* get_current_dir(&self) -> Option<&Path>