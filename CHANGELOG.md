# Changelog

## [v1.2.1]
### Added
- Composition support (#23)
- Wayland feature flag
- Workbooks (#21)
- Compilation flag for teachers' workbok editor
- Keyboard shortcuts for saving and opening files
- Auto save when a file is chosen (#20)
- Tooltips for the controls

### Fixed
- Fixed bug serializing/deserializing workbooks
- Fixed missing extension when saving workbooks
- Slightly improved keyboard event handling
- Title staying with the old value when compiling a new program that gives a compilation error

### Changed
- Updated dependencies

## [v1.2.0] - 2023-03-30
### Added
- Pagination to the workbook
- A real exercise for the workbook

### Fixed
- Fix undefined output (#18)

### Changed
- Moved library to external crate
- Improved cli (#19)

## [v1.1.0] - 2023-02-08
### Added
- CNAME for gh-pages
- Verbose flag for cli
- Non-interactive CLI mode: This mode just prints out the result and it is the default when running in CLI mode
- Infinite loop detection

### Fixed
- Detect undefined states
- WASM build (again)

### Changed 
- Replaced the `println!` with the logging functions that are appropriate for each verbose level
- Improved tape size and position
- Check if the tape has, at least, a 1 and throw an error if it doesn't.
- The program stops when finished

## [v1.0.2] - 2023-02-03
### Added
- A table to view the tape input
- An about page
- I10n - App translation (#7)
- Added test for wasm build
- Language toggle
- Added parser rule for program descriptions
- Added description label at the top
- More tests for the parser

### Fixed
- URI for gh-pages
- Github actions
- WASM build

### Changed
- Refactor: Created a lib with the turing logic

## [v1.0.0] - 2022-11-30
### Added
- Minor optimizations
- An error window
- Syntax error window and panel
- WASM support (#4)

### Fixed
- Two bugs

### Changed
- Improved tests
- Improved tokenization
- Improved error handling
- Improved parser structure and robustness
- Improved calculations for the tape position
- Improved font size
- Update syntax to make initial state independent from tape definition

## [v0.2.1] - 2022-11-21
### Added
- Workflow for github pages
- Tests
- Readme

### Fixed
- Fixed final state's instruction needing to be explicitly defined (#2)

## [v0.2.0] - 2022-11-21
- Initial version