# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.0.2] - 2020-11-23

#### Fixes

- Be explicit about spawn failure (closes [#109](https://github.com/assert-rs/assert_cmd/issues/109)).

## 1.0.1 (2020-03-30)

#### Fixes

- Reduce dependencies.

## 1.0.0 (2020-03-26)

Stable release!

## 0.12.2 (2020-03-26)

#### Features

* **cmd**:
  * Support timeouts (closes [#10](https://github.com/assert-rs/assert_cmd/issues/20)).

## 0.12.1 (2020-03-25)


#### Bug Fixes

* **cmd**:
  * Avoid stdin/stdout deadlocks by writing/reading in parallel (closes [#42](https://github.com/assert-rs/assert_cmd/issues/42)).

## 0.12.0 (2019-12-05)


#### Bug Fixes

*   More accurately name the cmd module ([15e40f67](https://github.com/assert-rs/assert_cmd/commit/15e40f6744b174e07c4fb4bd7703eb77d6e896ee), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))
* **stdin:**:  Provide a Command wrapper ([d159e875](https://github.com/assert-rs/assert_cmd/commit/d159e875aee71841198c67cd1a4e848b8bb9e465), closes [#73](https://github.com/assert-rs/assert_cmd/issues/73))

#### Breaking Changes

*   More accurately name the cmd module ([15e40f67](https://github.com/assert-rs/assert_cmd/commit/15e40f6744b174e07c4fb4bd7703eb77d6e896ee), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))
* **stdin:**:  Provide a Command wrapper ([d159e875](https://github.com/assert-rs/assert_cmd/commit/d159e875aee71841198c67cd1a4e848b8bb9e465), closes [#73](https://github.com/assert-rs/assert_cmd/issues/73))



## 0.11.1 (2019-03-23)


#### Bug Fixes

* **stdin:**  Docs didn't work ([2d4756a2](https://github.com/assert-rs/assert_cmd/commit/2d4756a2e20cafd5fa8904090eee53798a825196), closes [#71](https://github.com/assert-rs/assert_cmd/issues/71))



## 0.11.0 (2019-01-29)


#### Performance

* **cargo:**  Faster bin lookup ([93791474](https://github.com/assert-rs/assert_cmd/commit/9379147429ff1eb8cb0766c696d1ae6141b66a33), closes [#6](https://github.com/assert-rs/assert_cmd/issues/6), [#57](https://github.com/assert-rs/assert_cmd/issues/57), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))

#### Breaking Changes

* **cargo:**  Faster bin lookup ([93791474](https://github.com/assert-rs/assert_cmd/commit/9379147429ff1eb8cb0766c696d1ae6141b66a33), closes [#6](https://github.com/assert-rs/assert_cmd/issues/6), [#57](https://github.com/assert-rs/assert_cmd/issues/57), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))
  * As a side-effect, removed `cargo_example` in favor of using `escargot`.
  * See the [`assert_cmd::cargo` docs](https://docs.rs/assert_cmd/0.11.0/assert_cmd/cargo/index.html) for trade-offs with when to use `escargot` vs `assert_cmd`


## 0.10.2 (2018-11-21)


#### Bug Fixes

* **assert:**  Support Strings for easy comparison ([81035079](https://github.com/assert-rs/assert_cmd/commit/810350793df04ad4e3a7f6d760f23158432c5bb6), closes [#60](https://github.com/assert-rs/assert_cmd/issues/60))
* **docs:**
  * A broken link ([854f7c27](https://github.com/assert-rs/assert_cmd/commit/854f7c278b4977d3f24c47c208766f85bab81a18))
  * List caveats for cargo support.



## 0.10.1 (2018-10-10)


#### Bug Fixes

* Documentation fixes


## 0.10.0 (2018-10-10)


#### Breaking Changes

*   Remove deprecated functions ([fa01930c](https://github.com/assert-rs/assert_cmd/commit/fa01930cb9933d8b1ac024773a8bbf7330783507), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))
*   Better group API ([b1376f9a](https://github.com/assert-rs/assert_cmd/commit/b1376f9a29cbf093c17d1e3a641ee73aa5524e58), closes [#40](https://github.com/assert-rs/assert_cmd/issues/40), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))
*   Use predicates 1.0 ([1e0ece83](https://github.com/assert-rs/assert_cmd/commit/1e0ece8324dccb5d02c42c62b1ab2dea8032a924), closes [#8](https://github.com/assert-rs/assert_cmd/issues/8), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))
* **cargo:**  Reuse the current target ([403f612a](https://github.com/assert-rs/assert_cmd/commit/403f612ab66e7ccacc28c59ca1c52a5c230d4f9b), closes [#44](https://github.com/assert-rs/assert_cmd/issues/44), breaks [#](https://github.com/assert-rs/assert_cmd/issues/))

#### Features

* **assert:**  Simplify passing in Predicate<str> ([ee4c45ed](https://github.com/assert-rs/assert_cmd/commit/ee4c45ede885a57a2d2e2b5fe74801b12578893a), closes [#32](https://github.com/assert-rs/assert_cmd/issues/32))



## 0.9.1 (2018-08-09)


#### Bug Fixes

* **cargo:**  Point people to escargot ([2e32822c](https://github.com/assert-rs/assert_cmd/commit/2e32822ca22b4299a2a07a46ea431835c8f50401), closes [#44](https://github.com/assert-rs/assert_cmd/issues/44))



## 0.9.0 (2018-08-02)


#### Breaking Changes

* **assert:**
  *  Upgrade to predicates-core 0.9.0 ([e089a32d](https://github.com/assert-rs/assert_cmd/commit/e089a32d4fe1351a4a1958bd844ab49dccfbd826), closes [#1](https://github.com/assert-rs/assert_cmd/issues/1))
  *  Change set_stdin to clarify intent ([624a7988](https://github.com/assert-rs/assert_cmd/commit/624a7988a7527661cd821d7603fe18f5c1b49265), closes [#29](https://github.com/assert-rs/assert_cmd/issues/29))

#### Bug Fixes

* **assert:**
  * Change set_stdin to clarify intent ([624a7988](https://github.com/assert-rs/assert_cmd/commit/624a7988a7527661cd821d7603fe18f5c1b49265), closes [#29](https://github.com/assert-rs/assert_cmd/issues/29))
  * Reduced duplicate information being reported in failure message.
* Improve documentation to better jump start users.

#### Features

* **assert:**  Show cause of assert ([e089a32d](https://github.com/assert-rs/assert_cmd/commit/e089a32d4fe1351a4a1958bd844ab49dccfbd826), closes [#1](https://github.com/assert-rs/assert_cmd/issues/1))



## 0.6.0 (2018-07-18)


#### Features

* **with_stdin**: Accept files ([#24](https://github.com/assert-rs/assert_cmd/pull/24))

#### Breaking Changes

* **with_stdin**: Accept files ([#24](https://github.com/assert-rs/assert_cmd/pull/24))



## 0.5.0 (2018-07-13)


#### Features

* **with_stdin**: Work with chained APIs ([#23](https://github.com/assert-rs/assert_cmd/pull/23))

#### Breaking Changes

* **with_stdin**: Work with chained APIs ([#23](https://github.com/assert-rs/assert_cmd/pull/23))



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



## 0.3.0 (2018-06-07)

### Features

* Attempted to improve the documentation.

### Breaking Changes

* Moved all cargo stuff under `cargo` module.


## 0.2.0 (2018-06-06)


#### Features

*   Short-hand Assert::code ([e234685d](https://github.com/assert-rs/assert_cmd/commit/e234685d940b8b4df7b589d13d3014356965d5ef), [b3450b86](https://github.com/assert-rs/assert_cmd/commit/b3450b861ee08c529e86dbf857b7685e8697b275))

#### Bug Fixes

*  Simplify stdout/stderr str predicates ([8cdfb91e](https://github.com/assert-rs/assert_cmd/commit/8cdfb91e0f7a535d3d2b9fbb21f0df5d236a0f0a), closes [#11](https://github.com/assert-rs/assert_cmd/issues/11))

#### Breaking Changes

*   Change to predicates v0.5.0 ([5fa02435](https://github.com/assert-rs/assert_cmd/commit/5fa02435ffee0a3fb5f94fa374437ae71201f7d7))
*   Simplify stdout/stderr str predicates ([8cdfb91e](https://github.com/assert-rs/assert_cmd/commit/8cdfb91e0f7a535d3d2b9fbb21f0df5d236a0f0a), closes [#11](https://github.com/assert-rs/assert_cmd/issues/11))


<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v1.0.2...HEAD
[1.0.2]: https://github.com/assert-rs/assert_cmd/compare/v1.0.1...v1.0.2
