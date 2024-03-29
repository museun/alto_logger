# alto_logger

[![Crates][crates_badge]][crates]
[![Documentation][docs_badge]][docs]
[![Actions][actions_badge]][actions]

## filtering

use the environmental variable `RUST_LOG` with `module_name=level`

> RUST_LOG="tokio=warn,my_module=info,my_module::inner=trace"

## optional features

`time` enables printing out a UTC timestamp with [`time`](https://time-rs.github.io/book/api/format-description.html)

[docs_badge]: https://docs.rs/alto_logger/badge.svg
[docs]: https://docs.rs/alto_logger
[crates_badge]: https://img.shields.io/crates/v/alto_logger.svg
[crates]: https://crates.io/crates/alto_logger
[actions_badge]: https://github.com/museun/alto_logger/workflows/Rust/badge.svg
[actions]: https://github.com/museun/alto_logger/actions
