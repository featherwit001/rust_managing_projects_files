# rust_managing_projects_files
rust_managing_projects_files 

all the source code is typed by me so that something is different with what is displayed in the course video.

I explore what will happen when it comes to `mod some_module;` ; the differnece be# rust_managing_projects_files
rust_managing_projects_files 

all the source code is typed by me so that something is different with what is displayed in the course video.

I explore what will happen when it comes to `mod some_module;` ; the differnece between `mod` and `use` ; the association between `mod` and `pub`;

## crate 和 package

``` bash
# lib
cargo new --lib <my_project_name>

# binary executable
cargo new <my_project_name>
```

文件结构上，创建一个目录名称为<my_project_name>，其中包含`Cargo.toml` 和 `src/`， src/ 中包含lib.rs 或者 main.rs 或者两者都存在

在逻辑上，创建了一个 `package` ,名称为<my_project_name>，Cargo.toml 文件中也会标明。

一个 package 中可以含有 1 个或者 多个 crate：

一个 package 中可以含有 0 个或者 1 个 library crate, 以 `lib.rs` 为根

一个 package 中可以含有 0 个或者 多个 binary crate, 以 `main.rs` 为根，或者以 `src/bin/second_bin.rs` 或者 `src/bin/third_bin/main.rs` 为根

crate 指代在逻辑上相互关联的一些rust items，这种关联性是编写者制定的，这些items可以提供一些功能。

crate 一直指代同一个东西，但是在程序的不同阶段有不同的形态。可以是源文件，可以是编译结果(compiled artifacts),也可以是在crates.io上分享的crates

## mod 规则 

crate tree 的结构取决于源文件的目录结构和 module 结构。而mod 就是用于构建 module 结构的关键字。
在 `file.rs` 中遇到 `mod some_module` 指令之后，会按一定的规则寻找 `some_module` 模块。

规则如下:

假设源文件 `file.rs` 中的某行指令为 `mod some_module;`

那么 在当前文件 `file.rs` 的同级目录中寻找以 `some_module` 为名的文件或目录 

1. 如果找到**文件** `some_module.rs`，那么将该文件的内容作为 some_module 模块的 items,并将该模块放入`file.rs`中作为第一级子模块

2. 如果找到**目录** `some_module/`， 那么在目录中寻找到 `mod.rs` 文件，将 `mod.rs` 文件的内容作为 some_module 模块的 items,并将该模块放入`file.rs`中作为第一级子模块

3. 如果文件和目录都没有找到，

   1. 如果 `file.rs` 是 `main.rs`, `mod.rs`, `lib.rs` 等这些已经规定了特殊含义的文件，那么直接报错，找不到 `some_module` 模块

	 2. 如果 `file.rs` 不是 `main.rs`, `mod.rs`, `lib.rs` 等文件，那么就在 file.rs 的同级目录下，寻找 `file/` 目录, 并在 `file/` 目录中寻找以 `some_module` 为名的模块或者目录.
   处理方法如上两条所示。如果 `file/` 不存在，或者 `file/`中不存在以 `some_module` 为名的模块或者目录, 那么报错，找不到 `some_module` 模块

上述规则很简单，可以画流程图表示。这些规则是我自己根据 rust module 的两种风格，外加自己的测试之后总结出来的。

# pub 和 mod

`pub` 是修饰词，可以用于修饰 `mod`。Rust 模块中的所有 items 默认是私有的(private), 如果想要公开(public)，需要加入 `pub` 修饰。

私有和公开的含义如下：

在crate tree 视图下，不加pub时，所有的 module 都只能使用同级或者更高级的模块中的内容，不能使用低级模块中的内容。

即子模块可以使用父模块，父模块的父模块，和同级的其他模块的 items，不能使用自己的子模块和同级的其他模块的子模块中的 items。

模块间默认的使用权限规则总结为：向上保密，其余公开。

## pub 使用 

这里参照例子，自己尝试一下: `01_05_public_modules`

如果父模块的其他 items 想访问子模块，那么子模块就**不必**添加 `pub` 修饰，因为同级模块可以互相使用items。

如果父模块的 items 想使用子模块中的 items, 被使用的子模块 items **要**添加 `pub` 修饰。

这表示该子模块和父模块中的其他 items 具有同样的公开性。可以被父模块中的其他 items 使用，也可以被父模块的其他同级模块使用。


如果想要更高级的模块如祖父模块(父模块的父模块)想要使用子模块的 items, 子模块本身就**要**添加 `pub` 修饰，表示对上层模块公开，对父模块的父模块公开

那么就类似于父模块想要使用子模块中的 item, 子模块的 item 要添加 pub 修饰（层次结构是一样的）。

## pub 注意事项

注意： 如果子模块的 item 添加了 pub 修饰，但是子模块本身没有 pub 修饰，那么这个 item 依旧不能被父模块的其他 items 使用。

原因很简单，类似于树状结构的目录中，如果目录都不可以进入，那么目录中的内容都不可见，那么即使目录的内容中有一些是可以访问的，也无法访问了。


# mod 和 use

mod 是构建模块结构的，use只是引入一些 items, 并不改变结构。 使用 use 可以简化路径，可以起别名，可以批量引入。


# unit tests 和 integration tests

这部分内容有一定工程经验或者接受过正规的工程教育的人都对测试的重要性深有体会，我在学习了[CS3110_OCaml_Programming](https://github.com/featherwit001/CS3110_OCaml_Programming)之后
对单元测试(unit tests)和随机测试十分熟悉。

## 单元测试 unit tests

用到的一些attribute：
#[cfg(test)] 修饰模块，表示仅在测试时编译时被修饰的模块

#[test]修饰测试函数，表示测试函数，会在测试时被调用

#[ignore]修饰测试函数，表示一般测试时忽略该测试，仅在 `cargo test -- --ignore`时执行被修饰的测试函数

#[should_panic] 修饰测试函数，表示该函数会发生panic。发生panic,通过测试;不发生panic，测试失败。可以进一步细化panic类型，如下：

#[should_panic(expect="some string")] 表示该函数会发生含有"some string"提示字符的panic, 其他类型的panic，无法通过测试。

另外
``` bash
# cargo test 执行所有测试函数，包括unit tests 和 integration tests
# 操作顺序：1 编译test binary 2. 并行执行test binary 3.捕获tests的标准输出
cargo test

# cargo 参数传递规则
cargo test [arguments_for_cargo_test_utility] -- [arguments_for_the_test_binary]

# 测试时会捕获标准输出，这个参数传递给test binary，显示执行的测试的标准输出
cargo test -- --show-output

# 测试默认并行执行，限制执行线程数为1，可以取消并行，改为顺序执行
cargo test -- --test-threads=1

# 测试函数名含有xxx的测试函数
cargo test xxx
```
## 集成测试 integration tests

集成测试和单元测试的区别在于： 

单元测试在源文件中，测试独立的 items。

集成测试在源文件外，使用 package 名称引用 lib crate 下的 items, 使用提供的接口测试多个相互联系 item，例如组合操作，随机输入数据等 

这里的集成测试(integration tests)主要还是随机测试，需要建立`src/lib.rs` 和`tests/`目录 参考 '02_08' 和 '02_10'








