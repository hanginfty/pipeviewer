<h1 align="center">PipeViewer</h1>

一个Rust编写的查看管道传输速率的命令行工具

### 示例:

### 编译：

```shell
# clone this project:
git clone --depth=1 https://https://github.com/hanginfty/pipeviewer

cd pipeviewer

cargo build --release
```

### 用法:

```shell
pv --help

pipeviewer 

USAGE:
    pipeviewer [FLAGS] [OPTIONS] [infile]

FLAGS:
    -h, --help       Prints help information
    -s, --silent     
    -V, --version    Prints version information

OPTIONS:
    -o, --outfile <outfile>    Write output to a file instead of stdout.

ARGS:
    <infile>    Read file from a file instead of stdin
```

## 示例

```shell
# receive standard input, print bytes received & rate
echo "hello rust" | pv
```

<img title="" src=".\imgs\Snipaste_2022-08-12_16-47-12.png" height="80px">

```shell
# receive standard input, write in outputfile if specify '-o' flag
yes | pv -o > /dev/null
```

<img title="" src=".\imgs\Snipaste_2022-08-12_16-49-21.png" height="80px">