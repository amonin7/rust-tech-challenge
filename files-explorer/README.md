# Files Explorer

A function that given a directory, recursively finds all files with a given file extension in that directory
and all subdirectories, and counts the number of lines in the file and prints it to stdout.

## Installation guide

```shell
git clone git@github.com:amonin7/rust-tech-challenge.git
cd rust-tech-challenge/files-explorer
cargo run
```

Then you should provide the filepath and the extension for the files you want to count lines.

### Example

Assume that we have the next structure:

```text
files-explorer
| - tmp
|   - 1.txt
|   - 2.txt
|   - tmp2
|       - 3.txt
|       - 4.txt
|       - 5.png
```
and each file has amount of lines, corresponding to its filename.
Then you should make this:
```text
$ cargo run
   Compiling files-explorer v0.1.0 (/Users/andreyminin/IdeaProjects/rust-tech-task/files-explorer)
    Finished dev [unoptimized + debuginfo] target(s) in 2.02s
     Running `target/debug/files-explorer`
tmp .txt
```
to get this result:
```text
Total line amount: 10
```