<a name="0.5.0"></a>
## 0.6.0 (2018-07-18)


#### Features

* **with_stdin**: Accept files ([#24](https://github.com/assert-rs/assert_cmd/pull/24))

#### Breaking Changes

* **with_stdin**: Accept files ([#24](https://github.com/assert-rs/assert_cmd/pull/24))



<a name="0.5.0"></a>
## 0.5.0 (2018-07-13)


#### Features

* **with_stdin**: Work with chained APIs ([#23](https://github.com/assert-rs/assert_cmd/pull/23))

#### Breaking Changes

* **with_stdin**: Work with chained APIs ([#23](https://github.com/assert-rs/assert_cmd/pull/23))



<a name="0.4.0"></a>
## 0.4.0 (2018-06-28)


#### Features

* **assert:** Short-hand output predicates
  *  `stdout()`/`stderr()` accept `str`([43eceba0](https://github.com/assert-rs/assert_cmd/commit/43eceba04ad0d612f417fc46d140795115895204), closes [#2](https://github.com/assert-rs/assert_cmd/issues/2))
  *  `stdout()`/`stderr()` accept byte slices ([111abca9](https://github.com/assert-rs/assert_cmd/commit/111abca91db0e1d6ea6a6b94566f7b3425131a64))
  *  `code()` accept a set. ([72dca8d0](https://github.com/assert-rs/assert_cmd/commit/72dca8d00d8084a6b9fd2c5566c1e9543db58b83))
* **assert:** Context on status failures ([af52e9c5](https://github.com/assert-rs/assert_cmd/commit/af52e9c52edf3684db0f5ce23cbaa4650d0118a1), closes [#16](https://github.com/assert-rs/assert_cmd/issues/16))

#### Breaking Changes

* **assert:** Short-hand output predicates changed assert signatures ([43eceba0](https://github.com/assert-rs/assert_cmd/commit/43eceba04ad0d612f417fc46d140795115895204), closes [#2](https://github.com/assert-rs/assert_cmd/issues/2))
* Switch OutputError from Fail ([1061baa0](https://github.com/assert-rs/assert_cmd/commit/1061baa03fadc70924a2bf2d0640ef679dc7178d))
* **cargo:**  Define a CargoError ([7bd71a8a](https://github.com/assert-rs/assert_cmd/commit/7bd71a8a67c5a29a35f3876ef49204681eca1ef6))



<a name="0.3.0"></a>
## 0.3.0 (2018-06-07)

### Features

* Attempted to improve the documentation.

### Breaking Changes

* Moved all cargo stuff under `cargo` module.


<a name="0.2.0"></a>
## 0.2.0 (2018-06-06)


#### Features

*   Short-hand Assert::code ([e234685d](https://github.com/assert-rs/assert_cmd/commit/e234685d940b8b4df7b589d13d3014356965d5ef), [b3450b86](https://github.com/assert-rs/assert_cmd/commit/b3450b861ee08c529e86dbf857b7685e8697b275))

#### Bug Fixes

*  Simplify stdout/stderr str predicates ([8cdfb91e](https://github.com/assert-rs/assert_cmd/commit/8cdfb91e0f7a535d3d2b9fbb21f0df5d236a0f0a), closes [#11](https://github.com/assert-rs/assert_cmd/issues/11), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))

#### Breaking Changes

*   Change to predicates v0.5.0 ([5fa02435](https://github.com/assert-rs/assert_cmd/commit/5fa02435ffee0a3fb5f94fa374437ae71201f7d7), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))
*   Simplify stdout/stderr str predicates ([8cdfb91e](https://github.com/assert-rs/assert_cmd/commit/8cdfb91e0f7a535d3d2b9fbb21f0df5d236a0f0a), closes [#11](https://github.com/assert-rs/assert_cmd/issues/11), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))
