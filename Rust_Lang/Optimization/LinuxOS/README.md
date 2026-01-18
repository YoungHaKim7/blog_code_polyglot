- https://askubuntu.com/questions/1553558/cannot-call-perf-on-ubntu-24-04

```bash
sudo apt updat
sudo apt install linux-tools-generic
```

- export 설정해줘야함.

```bash
export perf=/usr/lib/linux-tools/6.8.0-87-generic/perf
$perf stat <your_binary>
```
