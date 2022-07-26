# rust 迭代器 (iterator) 详解

​      

# 翻译来源

https://github.com/rustomax/rust-iterators

这篇文章的目的是为一些常见的[iterator](https://so.csdn.net/so/search?q=iterator&spm=1001.2101.3001.7020)提供参考资料。并不能替代[Iterator 
 API](https://doc.rust-lang.org/std/iter/trait.Iterator.html)或者[书中的Rust iterator核心概念 ](https://doc.rust-lang.org/book/second-edition/ch13-02-iterators.html)，事实上这篇文章的内容来自以上两种内容。

> 为了更好理解本篇文章内容，推荐读者至少粗略了解[Rust](https://so.csdn.net/so/search?q=Rust&spm=1001.2101.3001.7020)

# 如何编译运行例子

```
git clone https://github.com/rustomax/rust-iterators.git
cd rust-iterators/
cargo run123
```

代码中使用了`nightly`版本的特性，如果你的Rust 为`statble`版本，请注释相应的代码区域。

# 0介绍

生活是重复的，其中的大部分事物都是成系列的。我们经常需要记录(count)、列举 
 (enumerate)、反复申明(iterate)这些事物。在编程中，有多种方式产生重复事物(repetition)，其中最为人熟知的是C风格的for循环。

```
for ( x = 0; x < 10; ++x ) {
  // do something
}123
```

虽然这种可行的方法足够强大而且足够灵活以适应多种情况，但它承担着对应的bug份额，例如错误分号放置、无意中在循环内部修改变量。本着与其他语言特性的安全和一致的精神，Rust中没有C风格的循环。 相反，Rust利用[迭代器](https://so.csdn.net/so/search?q=迭代器&spm=1001.2101.3001.7020)实现类似的目标（还有更多）。

# 1.基本Range

在Rust中循环一系列整数的最基本的方法是Range。Range由`..`标记产生，它生成步长为1的iterator 。

```
for i in 1..11 {
    print!("{} ", i);
}
// output: 1 2 3 4 5 6 7 8 9 101234
```

上面的代码将打印从1到10的一系列数字，而不包括最后一个数字11.换句话说，`..`会产生一个iterator ，它包含左边的数，排除在右边的数。 为了得到一个包含两端的范围的iterator，你使用`...`符号。 包含两端的范围的iterator目前是一个不稳定的功能，需要`nightly`编译器

```
#![feature(inclusive_range_syntax)]

for i in 1...10 {
  print!("{} ", i);
}
// output: 1 2 3 4 5 6 7 8 9 10123456
```

如果你不使用循环迭代器变量，你可以通过利用`_`来避免实例化它。 例如，下面的代码不需要实例化一个循环迭代器变量就可以输出迭代器中元素的数量：

```
let mut n: i32 = 0;
for _ in 0..10 {
  n += 1;
}
println!("num = {}", n);
// output: num = 10123456
```

上面的例子是有些多余，因为Rust中的迭代器有count()函数，它返回迭代器中元素的数量，而不需要在循环中对它们进行计数：

```
println!("num = {}", (0..10).count());
// output: num = 1012
```

> 你会发现有经验的Rust程序员能够用非常简洁的迭代器语言来表达，而不是采用的传统循环代码行。 当我们谈论适配器(adaptor)，消费者(consumer)和将迭代器方法链接(chaining)到复杂的语句时，我们将覆盖下面的一些模式。

# 2.深层发掘（Digging Deeper）

如果基本的增量顺序Range 不能满足你的需要，Rust中有很多方法来定制Range迭代器。 我们来看几个常见的问题。

通常，Range递增不是1，而是增加一个不同的数字。 这可以通过`filter()`方法来实现。 它应用一个闭包(closure)，它可以为迭代器的每个元素返回true或false，并产生一个只包含闭包返回true的元素的迭代器。

下面的迭代器将产生一个0到20之间的偶数序列。

```
for i in (0..21).filter(|x| (x % 2 == 0)) {
  print!("{} ", i);
}
// output: 0 2 4 6 8 10 12 14 16 18 201234
```

因为`filter()`使用闭包，所以非常灵活，可以用来生成复杂迭代器。 例如，下面的迭代器产生0到20之间的一系列整数，它们除以2和3得到余数：

```
for i in (0..21).filter(|x| (x % 2 == 0) && (x % 3 == 0)) {
  print!("{} ", i);
}
// output: 0 6 12 181234
```

虽然默认范围是递增的，但是使用`rev()`方法可以很容易地将其反转。

```
for i in (0..11).rev() {
  print!("{} ", i);
}
// output: 10 9 8 7 6 5 4 3 2 1 01234
```

另一个常见的迭代器适配器`map()`将闭包应用于每个元素，并返回结果迭代器。下面是一个迭代器的例子，它产生一个从1到10的数字的正方形序列：

```
for i in (1..11).map(|x| x * x) {
    print!("{} ", i);
}
// output: 1 4 9 16 25 36 49 64 81 1001234
```

`fold()`是一个非常强大的方法。 它返回一个特殊的“累加器”类型闭包的结果给迭代器的所有元素，得到一个单一的值。 下面的迭代器产生从1到5的数字的平方和。

```
#![feature(inclusive_range_syntax)]

let result = (1...5).fold(0, |acc, x| acc + x * x);
println!("result = {}", result);

// output: result = 55123456
```

也许理解这里发生的最简单的方法是以更程序化的方式重写上面的例子：

```
#![feature(inclusive_range_syntax)]

let mut acc = 0;

for x in 1...5 {
  acc += x * x;
}

let result = acc;
println!("result = {}", result);

// output: result = 55123456789101112
```

哇！`fold()`版本是不是更加简洁和可读？

# 3.[数组](https://so.csdn.net/so/search?q=数组&spm=1001.2101.3001.7020)迭代（Iterating over Arrays）

与迭代Range类似，我们可以迭代一个数组。 这样做的好处是数组可以包含任意类型的值，而不仅仅是整数。 唯一的警告是该数组不是一个迭代器。 我们需要使用`iter()`方法把它变成一个迭代器。

```
let cities = ["Toronto", "New York", "Melbourne"];

for city in cities.iter() {
  print!("{}, ", city);
}
// output: Toronto, New York, Melbourne,123456
```

# 4.组合迭代器适配器（Combining Iterator Adaptors）

在前面的章节中，我们介绍了各种各样的方法，可以让你生成许多不同类型的迭代器，当你开始结合这些方法的时候，Rust表现十分突出。

如果你想要一个`10`到`0`之间以步长`2`Range呢？ 通过将一个特性和几个方法组合到一个迭代器中可以很容易地完成这个任务：

```
#![feature(inclusive_range_syntax)]

for i in (0...10).rev().filter(|x| (x % 2 == 0)) {
  print!("{} ", i);
}
// output: 10 8 6 4 2 0123456
```

需要一个不连续的Range（基本上是两个不相Range的组合）？ 您可以使用`chain()`方法组合多个范围：

```
let c = (1..4).chain(6..9);

for i in c {
  print!("{} ", i);
}
// output: 1 2 3 6 7 8123456
```

你可以得到很有创意的组合的东西！ 下面是一个迭代器，结合了两个范围：第一个递增和过滤，另一个 是递减。 不知道这样一个可憎的东西怎么产生，但在这里却是实现！

```
let r = (1..20)
  .filter(|&x| x % 5 == 0)
  .chain((6..9).rev());

for i in r {
  print!("{} ", i);
}
// output: 5 10 15 8 7 612345678
```

> 请注意，在上面的例子中，Rust允许我们通过将复杂的迭代器语句拆分为多行来更好地表示复杂的迭代器语句。

另一个方便的方法是`zip()`。 它有点类似于`chain()`，因为它将两个迭代器合并为一个。 与`chain()`相比，`zip()`不产生连续的迭代器，而是产生元组(tuple)的迭代器： 
 ![这里写图片描述](https://cloud.githubusercontent.com/assets/20992642/17650212/185c5486-6216-11e6-8fd7-34d2aa976c07.PNG)

```
let cities = ["Toronto", "New York", "Melbourne"];
let populations = [2_615_060, 8_550_405, ‎4_529_500];

let matrix = cities.iter().zip(populations.iter());

for (c, p) in matrix {
  println!("{:10}: population = {}", c, p);
}
// output:
// Toronto   : population = 2615060
// New York  : population = 8550405
// Melbourne : population = 4529500123456789101112
```

# 5.字符Range（Ranges of Characters）

操作字符串或文本的字节数通常需要迭代字符Range的能力。 `char_iter`提供了方便的方法来产生这样的范围。 `char_iter`支持Unicode字符。

要使用`char_iter`，请在`Cargo.toml`中添加以下内容

```
[dependencies]
char-iter = "0.1"12
```

接着通过`char_iter::new()`产生字符Range

```
extern crate char_iter;

for c in char_iter::new('Д', 'П') {
  print!("{} ", c);
}
// output: Д Е Ж З И Й К Л М Н О П123456
```

# 6.向量迭代（Iterating over Vectors）

向量是Rust的基本结构之一。 就其性质而言，它非常适合于表示一系列重复项目。 Rust中有许多语言工具允许使用向量作为迭代器，反之亦然。

在最简单的情况下，类似于我们如何从数组创建迭代器，我们可以使用`iter()`方法从矢量创建迭代器。 事实上，这被认为是Rust在迭代向量中最习惯的方式。

```
let nums = vec![1, 2, 3, 4, 5];

for i in nums.iter() {
   print!("{} ", i);
}
// output: 1 2 3 4 5123456
```

事实上，上面的模式非常普遍，Rust的引用操作符`＆`为其提供了句法糖。

```
let nums = vec![1, 2, 3, 4, 5];
for i in &nums {
   print!("{} ", i);
}
// output: 1 2 3 4 512345
```

注意上面的借用(borrow)是不可改变的。 换句话说，它们是只读的。 如果我们想要改变我们的向量，我们必须使用可变的借用`＆mut`。 例如，下面的代码将可变地迭代一个矢量，使处理中的每个元素加倍。

```
let mut nums = vec![1, 2, 3, 4, 5];
for i in &mut nums {
    *i *= 2;
}
println!("{:?}", nums);

//output: [2, 4, 6, 8, 10]1234567
```

然而，现在你是一个迭代忍者()，你不会使用上面的`for`循环语法。 你会用一个地`map()`来代替，对吗？

```
let nums = vec![1, 2, 3, 4, 5];
let nums = nums.iter().map(|x| x * 2);
println!("{:?}", nums);

//output: [2, 4, 6, 8, 10]12345
```

> 轻微的离题。 如果我们想要使用可变的迭代器将元素添加到向量中，如下所示：
>
> ```
> let mut nums = vec![1, 2, 3, 4, 5];
> for i in &mut nums {
>     nums.push(*i);
> }
> println!("{:?}", nums);12345
> ```
>
> 它不编译，并抛出错误信息`cannot borrow `nums` as mutable more than once at a time`。 你看，我们的迭代器（在`for`循环中实例化）已经借用nums作为可变。 `push`表达试图再次这样做，这是禁止的。 这是在Rust中著名的安全机制。 如果我们可以将某个`push`入向量中，同时迭代它，则会导致迭代器失效，从而导致未定义的行为。 Rust可以在编译时防止发生这种情况。 迭代器不仅强大，而且它们也是超级安全的。

现在，我们做相反的事情 : 从迭代器创建一个向量。 为了做到这一点，我们需要所谓的消费者。 消费者迫使懒惰的迭代器实际产生值。`collect()`是一个普通的消费者。 它从一个迭代器获取值并将它们转换为所需类型的集合。 下面我们将从`1`到`10`的一系列数字变换成一个向量`i32`：

```
let v = (1..11).collect::<Vec<i32>>();
println!("{:?}", v);
// output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]123
```

为了获得向量的元素及其索引，可以使用`enumerate()`方法，该方法在每次迭代中返回一个包含索引和项目的元组：

```
let v = vec![1, 2, 3];
for (i, n) in v.iter().enumerate() {
    println!("v[{}] = {}", i, n);
}
// output:
// v[0] = 1
// v[1] = 2
// v[2] = 312345678
```

还有一些其他的功能，使向量上的迭代器特别有用。 
 `min()`和`max()`，例如返回Option，分别包含向量元素的最小值和最大值：

```
let v = vec![3, 5, 0, -2, 3, 4, 1];
let max = v.iter().max();
let min = v.iter().min();

println!("max = {:?}, min = {:?}", max, min);

// output: max = Some(5), min = Some(-2)1234567
```

`sum()`返回迭代器中所有值的总和。 以下程序利用`sum()`方法来计算一个相当平庸的学生的平均成绩：

```
let grades = vec![4, 5, 6, 9, 7, 4, 8];
let sum: i32 = grades.iter().sum();
let gpa = sum as f32 / grades.len() as f32;

println!("sum = {}, gpa = {:.2}", sum, gpa);

// output: sum = 43, gpa = 6.141234567
```

# 7.无限与超越(Infinity and Beyond)

到目前为止，我们已经处理了在某些有限范围的值上运行的迭代器。 Rust以这种方式推广迭代器，实际上可以创建一个无限范围！ 让我们考虑下面的例子：

```
let r = (1..).collect::<Vec<i32>>();1
```

`(1..)`定义了一个从1开始并且无限增量的Range。 实际上，这样的程序编译和运行，但最终崩溃的错误消息：`fatal runtime error: out of memory`。 那么，你可能会说这不是很实际。 事实上，无限范围本身是无用的。 让他们有用的是将他们与其他适配器和消费者结合起来。

一个特别有用的模式涉及使用`take()`方法来限制迭代器返回的项目数量。 下面的迭代器将返回可以被`5`整除的整数的正方形序列中的前`10`个。

```
let v = (1..)
  .map(|x| x * x)
  .filter(|x| x % 5 == 0 )
  .take(10)
  .collect::<Vec<i32>>();

println!("{:?} ", v);

// output: [25, 100, 225, 400, 625, 900, 1225, 1600, 2025, 2500]123456789
```

# 8.Itertools

[itertools](https://docs.rs/itertools/0.6.0/itertools)包含强大的附加迭代器适配器。 以下是一些例子。

为了使用`itertools`，需要在`Cargo.toml`加入如下配置：

```
[dependencies]
itertools = "0.6"12
```

请回忆我们如何使用`filter()`生成一个偶数范围？ `Itertools`有一个方便的`step()`方法。

```
extern crate itertools;
use itertools::Itertools;

for i in (0..11).step(2) {
    print!("{} ", i);
}

//output: 0 2 4 6 8 1012345678
```

`unique()`适配器消除了迭代器的重复。 重复项不需要顺序。

```
extern crate itertools;
use itertools::Itertools;

let data = vec![1, 4, 3, 1, 4, 2, 5];
let unique = data.iter().unique();

for d in unique {
  print!("{} ", d);
}

//output: 1 4 3 2 51234567891011
```

`join()`适配器将迭代器元素组合为单个字符串，元素之间有一个分隔符。

```
extern crate itertools;
use itertools::Itertools;

let creatures = vec!["banshee", "basilisk", "centaur"];
let list = creatures.iter().join(", ");
println!("In the enchanted forest, we found {}.", list);

// output: In the enchanted forest, we found banshee, basilisk, centaur.12345678
```

`sorted_by()`适配器将自定义排序顺序应用于迭代器元素，返回排序后的向量。 根据2016年“世界幸福指数”，以下计划将打印出前5名最幸福的国家。

> sorted_by() 使用[Ordering trait](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html)排序

```
extern crate itertools;
use itertools::Itertools;

let happiness_index = vec![ ("Austria", 12), ("Costa Rica", 14), ("Norway", 4),
  ("Australia", 9), ("Netherlands", 7), ("New Zealand", 8), ("United States", 13),
  ("Israel", 11), ("Denmark", 1), ("Finland", 5), ("Iceland", 3),
  ("Sweden", 10), ("Canada", 6), ("Puerto Rico", 15), ("Switzerland", 2) ];

let top_contries = happiness_index
  .into_iter()
  .sorted_by(|a, b| (&a.1).cmp(&b.1))
  .into_iter()
  .take(5);

for (country, rating) in top_contries {
  println!("# {}: {}", rating, country);
}

// output:
// # 1: Denmark
// # 2: Switzerland
// # 3: Iceland
// # 4: Norway
// # 5: Finland123456789101112131415161718192021222324
```

# 9.定制迭代器（Creating Your Own Iterators）

Rust的优点在于，你可以使用通用语言工具来扩展它。 让我们利用这个强大的力量，创造我们自己的迭代器！ 我们将构建一个非常简单的迭代器，产生一系列由浮点数`(f32，f32)`组成的温度`(华氏，摄氏)`对。 温度使用公知的公式计算：`°C =(°F-32)/ 1.8`。

迭代器以一个结构体(struct)开始。 我们命名的结构体名称也将是迭代器的名称。 我们将调用`FahrToCelc`。 该结构体包含一些有用的信息，这些信息在随后的迭代器调用之间保持不变。 我们将有两个 `f32 fields` : 华氏温度和增量步长：

```
struct FahrToCelc {
  fahr: f32,
  step: f32,
}1234
```

接下来，我们将创建一个的方法`new()`，它通过初始化迭代器的初始值以华氏温度和增量步长进行初始化。 这个方法严格来说不是必须的，不是迭代器实现的一部分，但是我觉得它是一个很好的语法糖，可以提高程序的整体可读性：

```
impl FahrToCelc {
  fn new(fahr: f32, step: f32) -> FahrToCelc {
    FahrToCelc { fahr: fahr, step: step }
  }
}12345
```

最后，我们通过为结构实现`Iterator Trait`来编写迭代器的行为。 至少需要包含以下内容：

- 定义`Item`类型。 它描述了迭代器将产生什么样的东西。 如前所述，我们的迭代器产生由浮点数`(f32，f32)`元组表示的温度对`(华氏，摄氏)`，所以我们的`Item`类型定义如下所示：

```
 type Item = (f32, f32);1
```

- 函数`next()`实际上会生成下一个`Item`。 `next()`对`self`进行可变引用( mutable reference)，并返回一个封装下一个值的`Option`。 我们必须返回一个选项而不是项目本身的原因是因为许多迭代器需要考虑它们已经达到序列结束的情况，在这种情况下它们返回`None`。 由于我们的迭代器生成一个无限序列，我们的next()方法将始终返回`Option <Self :: Item>`。 因此，我们的`next()`函数声明如下所示：

```
fn next (&mut self) -> Option<Self::Item>1
```

`next()`函数通常也会进行一些内部管理。 我们逐步增加华氏温度`fahr`，以便在随后的迭代中返回。 对内部字段进行这些修改是我们需要将`self`的可变引用传递给`next()`作为参数的原因。 
 结合在一起，这里是迭代器特征的实现：

```
impl Iterator for FahrToCelc {
  type Item = (f32, f32);

  fn next (&mut self) -> Option<Self::Item> {
    let curr_fahr = self.fahr;
    let curr_celc = (self.fahr - 32.0) / 1.8;
    self.fahr = self.fahr + self.step;
    Some((curr_fahr, curr_celc))
  }
}12345678910
```

最终的完整程序如下：

```rust
struct FahrToCelc {
  fahr: f32,
  step: f32,
}

impl FahrToCelc {
  fn new(fahr: f32, step: f32) -> FahrToCelc {
    FahrToCelc { fahr: fahr, step: step }
  }
}

impl Iterator for FahrToCelc {
  type Item = (f32, f32);

  fn next (&mut self) -> Option<Self::Item> {
    let curr_fahr = self.fahr;
    let curr_celc = (self.fahr - 32.0) / 1.8;
    self.fahr = self.fahr + self.step;
    Some((curr_fahr, curr_celc))
  }
}

fn main() {
  // pass the starting temperature and step to the initializer function
  let ftc = FahrToCelc::new(0.0, 5.0);

  // produce the iterator table of first 5 values
  let temp_table = ftc.take(5);

  // print out the temperature table nicely
  for (f, c) in temp_table {
    println!("{:7.2} °F = {:7.2} °C", f, c);
  }
}

// output:
//  0.00 °F =  -17.78 °C
//  5.00 °F =  -15.00 °C
// 10.00 °F =  -12.22 °C
// 15.00 °F =   -9.44 °C
// 20.00 °F =   -6.67 °C
```