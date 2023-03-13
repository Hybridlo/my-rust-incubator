Step 3.8: Logging and tracing
=============================

__Estimated time__: 1 day

[Rust] has flexible type system and [metaprogramming][1] capabilities, allowing to build both efficient and highly reusable log system. The idea is very similar to [`serde`] and is introduced in a widely used [`log`], [`slog`] and [`tracing`] crates.




## Simple logging

[`log`] crate represents a __single unified frontend interface (facade)__ which is used by all libraries at the same time, but is backed by one actual backend implementation on your choice. This allows to control all the logs (of application and its dependencies) from a single place and in a unified manner: opt-in and opt-out logs of libraries, separate logs by destinations, etc.

> - Libraries should link only to the `log` crate, and use the provided macros to log whatever information will be useful to downstream consumers.
> - Executables should choose a logger implementation and initialize it early in the runtime of the program. Logging implementations will typically include a function to do this.

One interesting part is that log levels can be [disabled at compile time][3], thus have __no runtime performance impact at all__, unless you're debugging.

For better understanding and familiarity with [`log`]'s design, concepts, usage, and features, read through the following articles:
- [Official `log` crate docs][`log`]




## Structured logging

For [structured logging][4] there is the excellent [`slog`] crate in [Rust] ecosystem.

> The ambition is to be The Logging Library for Rust. `slog` should accommodate a variety of logging features and requirements. If there is a feature that you need and standard `log` crate is missing, `slog` should have it.

It's __backward and forward compatible with [`log`]__ crate, extending its ideas and is baked with an [excellent performance][5].

For better understanding and familiarity with [`slog`]'s design, concepts, usage, and features, read through the following articles:
- [Official `slog` crate docs][`slog`]
- [Official `slog` crate wiki][6]




## Tracing

The famous [`tracing`] crate is fabulous at both [tracing][10] and [structured logging][4].

> `tracing` expands upon logging-style diagnostics by allowing libraries and applications to record structured events with additional information about _temporality_ and _causality_ — unlike a log message, a span in `tracing` has a beginning and end time, may be entered and exited by the flow of execution, and may exist within a nested tree of similar spans. In addition, `tracing` spans are _structured_, with the ability to record typed data as well as textual messages.

Its "killer feature", undoubtedly, is [spans functionality][7], so [people tend to prefer it over `slog`][9] even for usual logging. It's also __[backward and forward compatible][8] with [`log`]__ crate.

Speaking of [tracing][10], the [`tracing`] crate has good integrations with [OpenTelemetry]-compatible distributed tracing systems (and similar ones). All this allows to reuse the same solution both for logging, tracing (like [Jaeger], [Zipkin]), profiling (like [coz], [Tracy]), error reporting (like [Sentry]), etc.

For better understanding and familiarity with [`tracing`]'s design, concepts, usage, and features, read through the following articles:
- [Official `tracing` crate docs][`tracing`]
- [Yoav Danieli: Guide to OpenTelemetry Distributed Tracing in Rust][11]




## Task

Implement two loggers:
1. Global main `app.log` logger which prints all its logs to `STDOUT`, but `WARN` level (and higher) logs to `STDERR`.
2. Local `access.log` logger which writes all its logs to `access.log` file.

All logs should be structured and logged in a JSON format, and have time field with nanoseconds ([RFC3339] formatted).

Examples:
```json
{"lvl":"ERROR","file":"app.log","time":"2018-07-30T12:14:14.196483657Z","msg":"Error occurred"}
{"lvl":"INFO","file":"access.log","time":"2018-07-30T12:17:18.721127239Z","msg":"http","method":"POST","path":"/some"}
```




[`log`]: https://docs.rs/log
[`serde`]: https://docs.rs/serde
[`slog`]: https://docs.rs/slog
[`tracing`]: https://docs.rs/tracing
[coz]: https://github.com/plasma-umass/coz
[Jaeger]: https://www.jaegertracing.io
[OpenTelemetry]: https://opentelemetry.io
[Rust]: https://www.rust-lang.org
[RFC3339]: https://www.ietf.org/rfc/rfc3339.txt
[Sentry]: https://sentry.io
[Tracy]: https://github.com/wolfpld/tracy
[Zipkin]: https://zipkin.io

[1]: https://en.wikipedia.org/wiki/Metaprogramming
[3]: https://docs.rs/log/#compile-time-filters
[4]: https://dzone.com/articles/what-is-structured-logging
[5]: https://github.com/slog-rs/slog/wiki/What-makes-slog-fast
[6]: https://github.com/slog-rs/slog/wiki/FAQ
[7]: https://docs.rs/tracing#spans
[8]: https://docs.rs/tracing#log-compatibility
[9]: https://www.reddit.com/r/rust/comments/kdo29n/slog_vs_tracing_which_one_do_you_prefer
[10]: https://en.wikipedia.org/wiki/Tracing_(software)
[11]: https://www.aspecto.io/blog/distributed-tracing-with-opentelemetry-rust
