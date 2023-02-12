# Simple Server

## Build
```shell
git clone https://github.com/Sakura1943/simple_server.ggit
cd simple_server
cargo build --release
cd target/release
cp -f ./simple_server /usr/bin
```

## Usage
*default*
```shell
simple_server # default port: 8000, dir: ./, host: 127.0.0.1, index_file: index.html
```
*custom*
```shell
simple_server -p[--port] 3000 -b[--bind] 0.0.0.0 -d[--dir] ./public -i[--index-file] index.htm
```
*help*
```shell
simple_server --help
```
