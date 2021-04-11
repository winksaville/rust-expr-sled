# Experiment with sled

Initially this is from [sled.rs/introduction](https://sled.rs/introduction.html)
I then used [this stackoverflow](https://stackoverflow.com/questions/58358179/using-sled-how-do-i-serialize-and-deserialize)
as a more complex example. You can change the two if statments
in main to not do an insert, shows persistence. And the conditional
remove to empty the database. Also, if you delete my_storage_directory
it will be re-created on the next run.

```
wink@3900x:~/prgs/rust/projects/expr-sled-intro (main)
$ cargo run
   Compiling expr-sled-intro v0.1.0 (/home/wink/prgs/rust/projects/expr-sled-intro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/expr-sled-intro`
y=Symbol {
    symbol: "BNB",
    filters: [
        LotSize {
            min_qty: 1.0,
            max_qty: 100.0,
            step_size: 1.0,
        },
        MarketLotSize {
            min_qty: 20.0,
            max_qty: 200.0,
            step_size: 2.0,
        },
    ],
}
```
## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
