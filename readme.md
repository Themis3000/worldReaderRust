# wordReaderRust

This is my first project to try and learn rust! This program reads the data within minecraft region files (.mca files). Made a program like this, but implemented in python instead. Their implementations are nearly identical, although there's certainly some differences. You can see the python version here: https://github.com/Themis3000/worldReader

Both when writing in python and when writing in rust I didn't worry so much about profiling and optimization. I was mostly just worried about completing the program as soon as possible. I was really quite curious to see how the python version performed vs. the rust version.

The results for how long it took to process the 7.15MB are here (ryzen 5 2600, non-isolated environment):

- Python: ~3.6 seconds
- Rust: ~350ms (~0.35 seconds)

These results are incredible! With 0 previous experience with rust, the rust implementation is >10x faster!

I may revisit this project in the future. It would be nice to try and optimize this, utilize multithreading (There's room for easily divvying-up the task to multiple threads), and publish it as a crate. It would also be cool to learn how to compile and publish this as a python package as well, I know it's possible, but I'm unfamiliar with the process the limitations.