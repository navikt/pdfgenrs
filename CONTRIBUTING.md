# Contributing
This project is open to accept feature requests and contributions from the open source community.
Please fork the repo and start a new branch to work on.


## Building locally
This project is using [Cargo](https://doc.rust-lang.org/cargo/) for its build tool.

To run a build simply execute the following:
```shell script
cargo build
```

also run check formatting
```shell script
cargo fmt -- --check
```

and also run the linter
```shell script
cargo clippy --all-targets -- -D warnings
```

If this change can affect performance, you have run
```shell script
cargo bench --bench performance
```
and checked `Total (ms)` is as following
ms for single thread is under 800 ms and for multi thread is under 1200 ms



## Testing
If you are adding a new feature or bug fix please ensure there is proper test coverage.
execute the following to run test:
```shell script
cargo test
```


## Pull Request Review
If you have a branch on your fork that is ready to be merged, please create a new pull request. The maintainers will review to make sure the above guidelines have been followed and if the changes are helpful to all library users, they will be merged.
