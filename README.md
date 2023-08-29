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

## pub 使用 

这里参照例子比较好: 01_0

如果父模块的其他 items想访问子模块，那么子模块就不必添加 `pub` 修饰，因为同级模块可以互相使用items。

如果父模块的 items 想使用子模块中的 items, 被使用的子模块 items 要添加 pub 修饰。

这表示该子模块和父模块中的其他 items 具有同样的公开性。可以被父模块中的其他 items 使用，也可以被父模块的其他同级模块使用。


如果想要更高级的模块如祖父模块(父模块的父模块)想要使用子模块的items, 子模块本身就要添加 pub 修饰，表示对上层模块公开，对父模块的父模块公开

那么就类似于父模块想要使用子模块中的 item, 子模块的 item 要添加 pub 修饰（层次结构是一样的）。



## pub 注意事项

注意： 如果子模块的 item 添加了 pub 修饰，但是子模块本身没有 pub 修饰，那么这个 item 依旧不能被父模块的其他 items 使用。

原因很简单，类似于树状结构的目录中，如果目录都不可以进入，那么目录中的内容都不可见，那么即使目录的内容中有一些是可以访问的，也无法访问了。


# mod 和 use

mod 是构建模块结构的，use只是引入一些 items, 并不改变结构。 使用 use 可以简化路径，可以起别名，可以批量引入。


