# Profiling

## Useful links
[flamegraphing rust binaries' cpu usage with perf](https://gist.github.com/dlaehnemann/df31787c41bd50c0fe223df07cf6eb89)  
[Rust Performance: A story featuring perf and flamegraph on Linux](https://blog.anp.lol/rust/2016/07/24/profiling-rust-perf-flamegraph/)  
[Profiling Rust applications on Linux](https://llogiq.github.io/2015/07/15/profiling.html)  

## Best approaches from above links

This was only tested with a very basic program so may need to experiment with different approaches if this is not suitable.
```shell
sudo perf record -g --call-graph dwarf,16384 -e cpu-clock -F 997 target/release/msc-2048-ai
sudo perf script | stackcollapse-perf.pl | stackcollapse-recursive.pl | c++filt | flamegraph.pl > flame.svg
```

Using oprofile
```shell
sudo operf target/release/msc-2048-ai
sudo opannotate --source
```

## Installing FlameGraph
```shell
cd ~
mkdir Tools
cd Tools
git clone https://github.com/brendangregg/FlameGraph
cd ~
echo "PATH=~/Tools/FlameGraph:$PATH" >> ~/.profile
source ~/.profile
```
