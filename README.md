# fixed-length-search

The fastest algorithm for finding a path with a specific length in a graph

## How to run?

```sh
$ cargo run --release
```

## How fast is it?

Here is the output of the benchmark of the algorithm for a graph with 10 thousand vertices and density of 0.1:
```
Fill the graph - 250.07ms
Fixed length search - 19.52ms
The path is valid
```

Yep, that is milliseconds, not seconds.
