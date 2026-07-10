# Practice Tool Changelog

## v1.9.6-alpha.3 (2026-07-10)

**IMPORTANT:** This version is an alpha version. Regard it a fast-changing early test and report issues if meeting any problem.

### Added

* Add CER item spawning support. Item IDs are collected from [ConvergenceER_CN](https://github.com/soarqin/ConvergenceER_CN) repository. See [codegen README](xtask/src/codegen/README.md) for more technical details.

### Changed

* Refactor data files to merge additional CSV to `xtask/src/codegen/item_ids.yml`. Remove that CSV file.
* Allow `xtask` CLI to run only one step.

## v1.9.6-alpha.2 (2026-07-10)

**IMPORTANT:** This version adds many inelegant changes with hardcoded binary data files (`er-params.bin`) and much AI coding. Performance may be degraded.

**IMPORTANT:** This version is an alpha version. Regard it a fast-changing early test and report issues if meeting any problem.

### Added

* Add param ID and some other static data of targeted enemy.
  * All physical and attribute damange cut rate (in percentage; **positive values mean damage taken reduction** and vice versa).

### Changed

* **(BREAKING CHANGE)** Upgrade almost all dependencies. Much interface incompatibility is fixed by AI agents and maybe needs to be reverted in the future.
  * Also see commit [`851ab09`](https://github.com/LittleYe233/practice-tool-core/commit/851ab09009c9a03e77d3b663056098888ac25b3a) in [practice-tool-core](https://github.com/LittleYe233/practice-tool-core).
* `cargo xtask dist` now adds `CHANGELOG.md` and `er-params.bin` to compressed archive.
* Translate almost all labels (including widgets in [practice-tool-core](https://github.com/LittleYe233/practice-tool-core)) into Simplified Chinese.

### Fixed

* **(BREAKING CHANGE)** Fix GitHub CI to run flawlessly in current repository.
  * Still need some tweaks maybe.

## v1.9.5 (2026-05-31)

This version syncs from @soarqin's patch (commit SHA 68f0710dc53931dd0b9809bfad5090781b00274b) to latest Elden Ring version 1.16.2.

## v1.9.4 (2026-05-10)

This version syncs from @soarqin's latest commit.

Note that CER item data is parsed from CSV file and it needs to be merged with Serde JSON deserializer. It may cause performance issues. Also only some items are added due to lack of data from CT table.

### Added

* Add some of CER 2.2 items (data from Cheat Engine table on Convergence's Discord server).

### Changed

* Modify release readme and widget help text.
