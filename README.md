# cyclic_codes

Largely and liberally based on https://github.com/matrem94/believer

The master branch focuses on classical codes, and the quantum branch on quantum codes

An important difference: each decoding is done sequentially instead of in parallel since it allows us to re-use the same `ParityCheckMatrix` and reduce memory allocation and runtime. Then the parallelism is done at the code level: we simulate, say, 8 codes at a time instead of one code with 8 threads.
