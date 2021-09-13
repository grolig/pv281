# Change Log

## [Unreleased]

## v1.7.1 - 2021-02-07

### Fixed

- Fix Marpit's `<section>` detection to use `firstElementChild` instead of `firstChild` ([#33](https://github.com/marp-team/marpit-svg-polyfill/pull/33))

### Changed

- Upgrade development Node LTS ans dependent packages to the latest version ([#34](https://github.com/marp-team/marpit-svg-polyfill/pull/34))
- Rename `master` branch to `main` ([#35](https://github.com/marp-team/marpit-svg-polyfill/pull/35))

## v1.7.0 - 2020-08-18

### Changed

- Use transformation matrix generated from `getScreenCTM()` ([#30](https://github.com/marp-team/marpit-svg-polyfill/pull/30))

## v1.6.0 - 2020-08-16

### Added

- Allow to change the target of observer ([#28](https://github.com/marp-team/marpit-svg-polyfill/pull/28))
- Return clean-up function from `observe()` ([#28](https://github.com/marp-team/marpit-svg-polyfill/pull/28))

### Changed

- Upgrade dependent packages to the latest version ([#29](https://github.com/marp-team/marpit-svg-polyfill/pull/29))

## v1.5.0 - 2020-07-16

### Changed

- Propagate zoom factor in the top window into `<iframe>` through `postMessage` ([#24](https://github.com/marp-team/marpit-svg-polyfill/pull/24))
- Migrate from TSLint to ESlint ([#25](https://github.com/marp-team/marpit-svg-polyfill/pull/25))
- Upgrade dependent packages to the latest version ([#26](https://github.com/marp-team/marpit-svg-polyfill/pull/26))

## v1.4.0 - 2020-07-09

### Fixed

- Follow zoom factor of the parent window in `<iframe>` ([#21](https://github.com/marp-team/marpit-svg-polyfill/pull/21))

### Changed

- Upgrade development Node LTS ans dependent packages to the latest version ([#22](https://github.com/marp-team/marpit-svg-polyfill/pull/22))
- Test against Node 14 ([#23](https://github.com/marp-team/marpit-svg-polyfill/pull/23))

## v1.3.0 - 2020-06-07

### Fixed

- Fix visually shift depending on zoom level ([#18](https://github.com/marp-team/marpit-svg-polyfill/issues/18), [#19](https://github.com/marp-team/marpit-svg-polyfill/pull/19))

### Changed

- Simplify transformation ([#19](https://github.com/marp-team/marpit-svg-polyfill/pull/19))
- Upgrade dependent packages to the latest version ([#20](https://github.com/marp-team/marpit-svg-polyfill/pull/20))

## v1.2.1 - 2020-04-18

### Changed

- Upgrade dependent packages to the latest version ([#17](https://github.com/marp-team/marpit-svg-polyfill/pull/17))

## v1.2.0 - 2020-01-12

### Changed

- Upgrade dependent packages to the latest version ([#14](https://github.com/marp-team/marpit-svg-polyfill/pull/14))

### Removed

- CI test against EOL Node 8 ([#14](https://github.com/marp-team/marpit-svg-polyfill/pull/14))

## v1.1.1 - 2019-07-13

### Changed

- Upgrade dependent packages to the latest version ([#12](https://github.com/marp-team/marpit-svg-polyfill/pull/12))

## v1.1.0 - 2019-06-20

### Changed

- Simplify WebKit detection (JS build for browser become about 5x smaller) ([#9](https://github.com/marp-team/marpit-svg-polyfill/pull/9))
- Upgrade dependent packages to the latest version ([#10](https://github.com/marp-team/marpit-svg-polyfill/pull/10))

## v1.0.0 - 2019-06-05

### Breaking

- Drop support of Node 6.

### Changed

- Upgrade dependent packages to the latest version ([#7](https://github.com/marp-team/marpit-svg-polyfill/pull/7))
- Automate GitHub release ([#8](https://github.com/marp-team/marpit-svg-polyfill/pull/8))

## v0.3.0 - 2019-03-20

### Fixed

- Incorrect scaling with custom zoom level ([#3](https://github.com/marp-team/marpit-svg-polyfill/issues/3), [#4](https://github.com/marp-team/marpit-svg-polyfill/pull/4))

### Changed

- Upgrade dependent packages to the latest ([#5](https://github.com/marp-team/marpit-svg-polyfill/pull/5))

## v0.2.0 - 2018-12-31

### Fixed

- Fix incorrect scaling in split background ([#2](https://github.com/marp-team/marpit-svg-polyfill/pull/2))

## v0.1.0 - 2018-12-29

### Changed

- Split picking out logics into `polyfills()` function ([#1](https://github.com/marp-team/marpit-svg-polyfill/pull/1))

## v0.0.1 - 2018-12-29

- Initial release.
