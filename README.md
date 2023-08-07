# Rust Allocator Benchmarks on WASM

This project serves to provide performance data for Rust WASM allocators.

This was originally created in service of [Talc](https://github.com/SFBdragon/talc). Results have been published there.

After running one of the commands below, open `localhost:8080/` give it a few seconds, then open the console to see the results.

#### Benchmark `talc`
```
wasm-pack build --release -- --features talc && cd www && npm run start && cd ..
```

#### Benchmark `lol_alloc`
```
wasm-pack build --release -- --features lol_alloc && cd www && npm run start && cd ..
```

#### Benchmark default
```
wasm-pack build --release && cd www && npm run start && cd ..
```
