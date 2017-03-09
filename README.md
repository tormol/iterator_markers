# iterator_markers
Extra marker traits for iterators:

* **[UniqueIterator](http://tormol.github.io/rustdoc/iterator_markers/trait.UniqueIterator.html)**
* **[AscendingIterator](http://tormol.github.io/rustdoc/iterator_markers/trait.AscendingIterator.html)**
* **[DescendingIterator](http://tormol.github.io/rustdoc/iterator_markers/trait.DescendingIterator.html)**

## Feature flags:
* **unstable**: Implement for `Range` and `RangeInclusive`.
* (opt-out) **std**: Implement for the map and set iterators in std and don't set `#[no_std]`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
