# check_port使用

程序包含2个子命令，server和client

## server侧
```shell
./check_port server -h

server sub command

Usage: check_port server [OPTIONS] --port <PORT>

Options:
      --ip <IP>        IP address allowed to connect from client [default: 0.0.0.0]
      --port <PORT>    udp Port to listen, must be 1 to 65535
      --token <TOKEN>  check token, send from client [default: "hello world"]
  -h, --help           Print help
```
server可以起N个进程，每个进程分别监听一个端口，比如：
```shell
# 允许所有ip(0.0.0.0)，token为默认hello world
./check_port server --port 3000
./check_port server --port 3001
./check_port server --port 3002
```

## client侧
```shell
client sub command

Usage: check_port client [OPTIONS] --ip <IP> --from-port <FROM_PORT> --to-port <TO_PORT>

Options:
      --ip <IP>                udp server IP address to connect, for example 127.0.0.1
      --from-port <FROM_PORT>  udp Port to connect from, must be 1 to 65535
      --to-port <TO_PORT>      udp Port to connect to, must be 1 to 65535 and <= from. will connect to IP:[from, to]
      --token <TOKEN>          check token, send to server and also check from response [default: "hello world"]
      --timeout <TIMEOUT>      timeout for check task, ms [default: 1000]
      --max-task <MAX_TASK>    max number of concurrent tasks [default: 300]
  -h, --help                   Print help
```
client可以针对某个IP，进行端口段扫描：
```shell
./check_port client --ip 127.0.0.1 --from-port 3000 --to-port 3002 --timeout 1000
```
timeout指server回包超时时间，单位是ms。如果端口段很多，可以修改max-task来找到合适的并发任务数量，提升扫描速度（默认300，一般不需要动，不一定越大越快）

## 其他
TOKEN是指的server会验证client发过来的包内容，不一样则会认为端口不通，默认都是"hello world"
