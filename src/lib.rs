//! # rust-rdkafka
//! Kafka client library for Rust based on [librdkafka].
//!
//! ## The library
//! `rust-rdkafka` provides a safe Rust interface to librdkafka. It is currently based on librdkafka 0.9.4.
//!
//! ### Documentation
//!
//! - [Current master branch](https://fede1024.github.io/rust-rdkafka/)
//! - [Latest release](https://docs.rs/rdkafka/)
//!
//! ### Features
//!
//! The main features provided at the moment are:
//!
//! - Support for Kafka 0.8.x, 0.9.x and 0.10.x (timestamp support coming soon). For more information about  broker compatibility options, check the [librdkafka documentation].
//! - Consume from single or multiple topics.
//! - Automatic consumer rebalancing.
//! - Customizable rebalance, with pre and post rebalance callbacks.
//! - Offset commit.
//! - Message production.
//! - Access to cluster metadata (list of topic-partitions, replicas, active brokers etc).
//! - Access to group metadata (list groups, list members of groups, hostnames etc).
//! - Access to producer and consumer metrics and statistics.
//!
//! [librdkafka documentation]: https://github.com/edenhill/librdkafka/wiki/Broker-version-compatibility
//!
//! ### Client types
//!
//! `rust-rdkafka` provides low level and high level consumers and producers. Low level:
//!
//! * [`BaseConsumer`]: simple wrapper around the librdkafka consumer. It requires to be periodically `poll()`ed in order to execute callbacks, rebalances and to receive messages.
//! * [`BaseProducer`]: simple wrapper around the librdkafka producer. As in the consumer case, the user must call `poll()` periodically to execute delivery callbacks.
//!
//! High level:
//!
//!  * [`StreamConsumer`]: it returns a [`stream`] of messages and takes care of polling the consumer internally.
//!  * [`FutureProducer`]: it returns a [`future`] that will be completed once the message is delivered to Kafka (or failed).
//!
//! [`BaseConsumer`]: https://docs.rs/rdkafka/0.9.0/rdkafka/consumer/base_consumer/struct.BaseConsumer.html
//! [`BaseProducer`]: https://docs.rs/rdkafka/0.9.0/rdkafka/producer/struct.BaseProducer.html
//! [`StreamConsumer`]: https://docs.rs/rdkafka/0.9.0/rdkafka/consumer/stream_consumer/struct.StreamConsumer.html
//! [`FutureProducer`]: https://docs.rs/rdkafka/0.9.0/rdkafka/producer/struct.FutureProducer.html
//! [librdkafka]: https://github.com/edenhill/librdkafka
//! [futures]: https://github.com/alexcrichton/futures-rs
//! [`future`]: https://docs.rs/futures/0.1.3/futures/trait.Future.html
//! [`stream`]: https://docs.rs/futures/0.1.3/futures/stream/trait.Stream.html
//!
//! *Warning*: the library is under active development and the APIs are likely to change.
//!
//! ### Asynchronous data processing with tokio-rs
//! [tokio-rs] is a platform for fast processing of asynchronous events in Rust. The interfaces exposed by the `StreamConsumer` and the `FutureProducer` allow rust-rdkafka users to easily integrate Kafka consumers and producers within the tokio-rs platform, and write asynchronous message processing code. Note that rust-rdkafka can be used without tokio-rs.
//!
//! To see rust-rdkafka in action with tokio-rs, check out the [asynchronous processing example] in the examples folder.
//!
//! [tokio-rs]: https://tokio.rs/
//! [asynchronous processing example]: https://github.com/fede1024/rust-rdkafka/blob/master/examples/asynchronous_processing.rs
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rdkafka = "^0.9.0"
//! ```
//!
//! This crate will compile librdkafka from sources and link it statically to your executable. To compile librdkafka you'll need:
//!
//! * the GNU toolchain
//! * GNU `make`
//! * `pthreads`
//! * `zlib`
//! * `libssl-dev`: optional, *not* included by default (feature: `ssl`).
//! * `libsasl2-dev`: optional, *not* included by default (feature: `sasl`).
//!
//! To enable ssl and sasl, use the `features` field in `Cargo.toml`. Example:
//!
//! ```toml
//! [dependencies.rdkafka]
//! version = "^0.9.0"
//! features = ["ssl", "sasl"]
//! ```
//!
//! ## Compiling from sources
//!
//! To compile from sources, you'll have to update the submodule containing librdkafka:
//!
//! ```bash
//! git submodule update --init
//! ```
//!
//! and then compile using `cargo`, selecting the features that you want. Example:
//!
//! ```bash
//! cargo build --features "ssl sasl"
//! ```
//!
//! ## Examples
//!
//! You can find examples in the `examples` folder. To run them:
//!
//! ```bash
//! cargo run --example <example_name> -- <example_args>
//! ```
//!
//! ## Tests
//!
//! ### Unit tests
//!
//! The unit tests can run without a Kafka broker present:
//!
//! ```bash
//! cargo test --lib
//! ```
//!
//! ### Integration tests
//!
//! rust-rdkafka contains a suite of integration tests which is automatically executed by travis in
//! docker container. Given the frequent interaction with C code that rust-rdkafka has to do, tests
//! are executed in valgrind to check eventual memory errors and leaks.
//!
//! To run the full suite using docker-compose:
//!
//! ```bash
//! ./integration_tests.sh.
//! ```
//!
//! To run locally, instead:
//!
//! ```bash
//! KAFKA_HOST="kafka_server:9092" cargo test
//! ```
//!
//! In this case there is a broker expected to be running on `KAFKA_HOST`.
//! The broker must be configured with default partition number 3 and topic autocreation in order
//! for the tests to succeed.
//!

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate futures;
extern crate errno;

extern crate rdkafka_sys as rdsys;

pub use rdsys::types as types;

pub mod client;
pub mod commit;
pub mod config;
pub mod consumer;
pub mod error;
pub mod groups;
pub mod message;
pub mod metadata;
pub mod producer;
pub mod statistics;
pub mod topic_partition_list;
pub mod util;

// Re-export
pub use message::Message;
