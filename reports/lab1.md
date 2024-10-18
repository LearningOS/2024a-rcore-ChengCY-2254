# 2024 秋冬季开源操作系统训练营 第二阶段实验报告
## 功能实现
对TaskControlBlock新增了以下字段
- start_time
- syscall_times
在src/task/mod中实现了以下函数

```
fn update_task_info()

fn update_syscall_counter(syscall_id:usize)

get_task_info
```
## 简答作业
**SBI环境为 0.2.0-alpha.3**  
**QEMU环境为 9.10**
1. >正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
   - bad address中，0x0这个地址不可写，被允许的内存范围在0x80000000..0x88000000之间。
   - bad instructions中，sret会让处理器从S态返回U态，但程序已是已U态，所以会出现异常。
   - bad register中，sstatus这个寄存器只能在S态中访问，非S态访问会导致异常。
2. >深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:  
   - >L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。
   - 刚进入`__restore`的时候，a0是TrapContext。
   - 用于恢复任务的执行，和内核态返回到用户态。
   - >L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
   - 处理了sstatus、sepc、sscratch这三个寄存器的值
   - status寄存器用来存储S态的状态信息 
   - sepc用于存储引发异常的指令地址
   - sscratch存储用户的堆栈指针

