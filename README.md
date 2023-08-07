# Rust Allocator Benchmarks on WASM

This project serves to provide performance data for Rust WASM allocators.

This was originally created in service of [Talc](https://github.com/SFBdragon/talc). Results have been published there.

The current actual benchmark is simply randomly allocating, deallocating, and reallocating chunks of memory.

Instructions:
1. Open up a terminal in the `www` folder
2. Run `npm install webpack@latest`
3. Run one of the command below
4. Open `localhost:8080/` in a modern browser
5. Give it a few seconds
6. Open up the console to see the results.

#### Benchmark `talc`
```
wasm-pack build --release -- --features talc && npm run start
```

#### Benchmark `lol_alloc`
```
wasm-pack build --release -- --features lol_alloc &&  npm run start
```

#### Benchmark default
```
wasm-pack build --release && npm run start
```
