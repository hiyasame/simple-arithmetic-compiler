# simple-arithmetic-compiler

> 未完成

学习项目，目的是做一些编译原理的实践

使用编译原理的思路实现简单的整数四则运算，编译到自定义的栈式计算机指令 (类似JVM Bytecode)

> (1 + 2) * 3 / 4 - 5

将会被编译为:

~~~
push 1
push 2
add
push 3
mul
push 4
div
push 5
sub 
ret
~~~

## 流程

- 分词，将传入的四则运算式分为 `NUMBER`, `OPERATOR`, `PAREN` 三种类型的 Token
- 语法分析，根据预先设计好的文法进行递归下降解析，得到一棵 AST
~~~
Expr -> Term ExprTail
ExprTail ->  + Term ExprTail
             - Term ExprTail
             null
Term -> Factor TermTail
TermTail ->  * Factor TermTail
             / Factor TermTail
             null
Factor -> (Expr)
           num
~~~
- 将AST转译到指令

## usage

编译四则运算式到栈式计算机指令

~~~shell
./arithmetic compile -s test.txt -o code.txt
~~~

解释执行栈式计算机指令

~~~shell
./arithmetic exec code.txt
~~~