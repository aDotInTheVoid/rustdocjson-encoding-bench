```
alona@Ashtabula:~/dev/rust/rustdocjson-encoding-bench$ cargo r --release -- --sample-size 10 --bench
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/rustdocjson-encoding-bench --sample-size 10 --bench`
json size:     266577858
postcard size:  72862979
cbor size:     202567397
Benchmarking deserialize_json: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 24.1s.
deserialize_json        time:   [1.6131 s 1.6273 s 1.6421 s]

Benchmarking serialize_json: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 8.8s.
serialize_json          time:   [830.83 ms 858.67 ms 887.55 ms]

Benchmarking deserialize_postcard: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 14.2s.
deserialize_postcard    time:   [876.74 ms 914.05 ms 957.80 ms]
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe

serialize_postcard      time:   [304.67 ms 310.03 ms 316.34 ms]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking deserialize_cbor: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 27.1s.
deserialize_cbor        time:   [2.1324 s 2.1645 s 2.2046 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

serialize_cbor          time:   [322.64 ms 329.83 ms 336.18 ms]
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) low mild
```

