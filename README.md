# sudoku

```console
$ cat input.txt
8 _ _ _ 1 _ _ _ _
_ _ _ 4 _ _ _ _ _
_ 7 _ _ _ _ _ _ 3
_ _ _ _ _ 2 _ _ _
_ _ _ _ _ 7 _ 6 _
_ _ _ _ _ _ 5 1 _
_ _ _ _ _ 3 _ _ 7
_ _ 6 _ _ _ _ _ 2
4 _ 1 _ 5 _ _ _ _
$ cargo run --release < input.txt
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/sudoku`
8 6 4 3 1 9 7 2 5
3 9 2 4 7 5 6 8 1
1 7 5 8 2 6 4 9 3
6 1 9 5 8 2 3 7 4
5 4 3 1 9 7 2 6 8
2 8 7 6 3 4 5 1 9
9 5 8 2 6 3 1 4 7
7 3 6 9 4 1 8 5 2
4 2 1 7 5 8 9 3 6

```
