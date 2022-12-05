# std::process::Child

```rust
pub struct Child {
  pub stdin::Option<ChildStdin>,
  pub stdout:Option<ChildStdout>,
  pub stderr: Option<ChildStderr>,
  // some fields omitted
}
```
表示正在运行或退出的进程
子进程没有实现Drop的实现，所以如果不确保Child已经退出，那么它就会继续运行，即使在子进程的Child句柄已经离开了作用域之后。

## 字段

* stdin 写入子节点标准输入的句柄。
> let stdin = child.stdin.take().unwrap();

* stdout 从子节点的标准输出读取的句柄
> let stdout = child.stdout.take().unwrap();

* stderr 从子节点的标准错误读取的句柄
> let stderr = child.stderr.take().unwrap();

# 方法

* kill(&mut self) -> Result<()>
强制子进程退出，如果子节点已经退出，则返回 InvalidInput 错误。 到 ErrorKind 的映射不是函数的兼容性契约的一部分。 这等效于在 Unix 平台上发送 SIGKILL。

* id -> u32
返回与此子级关联的操作系统分配的进程标识符

* wait(&mut self) -> Result<ExitStatus>
等待子节点完全退出，并返回退出时的状态。 至少调用一次之后，该函数将继续具有相同的返回值。 子进程的 stdin 句柄 (如果有) 将在等待之前关闭。 这有助于避免死锁：它确保子进程在父进程等待子进程退出时不会阻止其等待父进程的输入。

* try_wait(&mut slef)->Result<Option<ExitStatus>>
* wait_with_output(self)->Result<Output>



