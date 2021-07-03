# StudyRust 学习日记 源码

#### 介绍
Rust是一门系统编程语言，专注于安全，尤其是并发安全，支持函数式和命令式以及泛型等编程范式的多范式语言。Rust在语法上和C++类似，但是设计者想要在保证性能的同时提供更好的内存安全。 Rust最初是由Mozilla研究院的Graydon Hoare设计创造，然后在Dave Herman, Brendan Eich以及很多其他人的贡献下逐步完善的。Rust的设计者们通过在研发Servo网站浏览器布局引擎过程中积累的经验优化了Rust语言和Rust编译器。Rust致力于成为优雅解决高并发和高安全性系统问题的编程语言，适用于大型场景，即创造维护能够保持大型系统完整的边界。这就导致了它强调安全，内存布局控制和并发的特点。标准Rust性能与标准C++性能不相上下。

目前Rust在商业领域增长迅速，其中包括：

- Amazon，使用Rust 作为构建工具。
- Atlassian，在后端使用Rust。
- Dropbox，在前后端均使用了Rust。
- Facebook，使用Rust 重写了源码管理工具。
- Google，在Fuchsia 项目中部分使用了Rust。
- Microsoft，在Azure IoT 网络上部分使用了Rust。
- npm，在其核心服务上使用了Rust。
- RedHat，使用Rust 创建了新的存储系统。
- Reddit，使用Rust 处理评论。
- Twitter，在构建团队中使用Rust。

目前国内巨头公司像字节跳动，蚂蚁金服等也都开始使用Rust语言了。

#### 前言

这里是 Rust 学习日记 的全部源码源码。关于教程在`简书`，`CSDN`，`掘金`，`知乎`同步更新。整套课程全部由笔者根据自己对Rust的理解和参考官方文档写的。当然本人水平有限，错误和不足之处在所难免，处理问题也有不妥之处，敬请各位大佬，专家批评改正。大家可以在这里提`ISSUE`或者在上述四个平台留言。

另外，本系列课程也是非常适合0编程语言基础的同学，如果您是Rust大神，那么可以忽略本系列的文章。本系列课程不是完全对官方文档的翻译，在文章中，我还会讲解一些常见的问题和注意事项，比较笔者已经踩过坑了。学习Rust之路可能比较陡峭，但是我会用最通俗的语言带你慢慢深入，课程将会从**基础，进阶，实战，算法**四个部分循序渐进的讲解。该系列教程所用的Rust版本是`1.52.1`（截至目前的最新版本）。

课程目前预计会更新100课时，`简书`和`CSDN`不仅更新Rust教程，而且还会更新关于Rust初学者的一些练习题（约100题）。

**讲个段子**

> 两个初学者的对话。
>
> C语言初学者：编译成功了，怎么运行又报错了...是哪里出了问题？
>
> Rust语言初学者：怎么又编译失败了，程序什么时候能跑起来。。。

——*如果你没有被Rust的编译器毒打过，请不要说你学过Rust*。

#### 博客地址

* [简书](https://www.jianshu.com/u/573f6a58cd12)
* [CSDN](https://blog.csdn.net/a1595901624)
* [掘金](https://juejin.cn/user/1679709499033422)
* [知乎](http://www.zhihu.com/people/1595901624
  )

#### 反馈方式

由于本人水平有限，错误和不足之处在所难免，处理问题也有不妥之处，敬请各位大佬，专家批评改正。这里有以下三种方式反馈问题：
* 在Gitee提`ISSUE`
* 在项目下面评论
* 在上述四个平台留言

#### 版权声明

源码仅供学习使用，切勿使用商业用途~

#### 源码目录

001-RUST  学习日记 第1课 ——Hello World

002-RUST  学习日记 第2课 ——Cargo

003-RUST  学习日记 第3课 ——IDE

004-RUST  学习日记 第4课 ——Rust规范

005-RUST  学习日记 第5课 ——变量和常量

006-RUST  学习日记 第6课 ——基本数据类型

007-RUST  学习日记 第7课 ——字面量和运算符

008-RUST  学习日记 第8课 ——类型转换

009-RUST  学习日记 第9课 ——输入与输出

010-RUST  学习日记 第10课 ——复合数据类型

011-RUST  学习日记 第11课 ——向量

未完待续...