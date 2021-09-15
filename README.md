# APIs

## Requirements
* [Requirements V1](docs/v1/requirements.md)

## Results
* [Results V1](docs/v1/results.md)

## Benchmarks
### Outputs 
1. Coded files count; 
2. Coded lines count; 
3. Response time; 
4. Memory usage; 
5. CPU usage;

### Tools
#### AlDanial/cloc
[cloc][cloc] counts blank lines, comment lines, and physical lines of source 
code in many programming languages.

> Will be used to calculate output 1 (coded files count) and 2 (coded lines count);

```bash
cloc netcoreapi --exclude-dirs=bin,obj,Migrations
cloc pythondjango --exclude-dir migrations
cloc rustapi --exclude-dir=target
```

#### tsenart/vegeta
[Vegeta][vegeta] is a versatile HTTP load testing tool built out of a need to
drill HTTP services with a constant request rate. It can be used both as a command line utility
and a library.
> Will be used to calculate output 3 (response time)

#### gnuplot
[gnuplot][gnuplot] is a portable command-line driven graphing utility for Linux, OS/2, MS Windows,
OSX, VMS, and many other platforms.
> Will be used to generate chart;

```bash 
bash generate_bar_charts.sh v1
```

## References
### Rust
* [Creating a Rest API in Rust With Warp][rustapi]

[rustapi]: https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
[cloc]: https://github.com/AlDanial/cloc
[vegeta]: https://github.com/tsenart/vegeta
[gnuplot]: http://www.gnuplot.info/