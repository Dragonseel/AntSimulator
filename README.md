# AntSimulator

A simple simulator that simulates the life of very simplified ants.
They exist, they spend energy and they try to find food and eat it.

In the beginning of a simulation, a nest exists. They spawn the ants.

The behavior of ants and nests is in the ant_lib.
The update-function is dynamically loaded into the application,
so that the ant-colony-logic is hot-reload-able.

# Todo
  - [x] Hot-Reloading
  - [x] Nests
  - [x] Proof-Of-Concept LazyStatic storage for Colony logic
  - [ ] Ants can carry food
  - [ ] Ants can deliver food to nest
  - [ ] Way to calculate fitness/score


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
