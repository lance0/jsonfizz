# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-12-02

### Added
- **YAML Output Format**: Add `--format yaml` option for YAML serialization
- **TOML Output Format**: Add `--format toml` option for TOML serialization
- **TOML Input Support**: Add `--input-format toml` option for reading TOML files
- **YAML Input Support**: Add `--input-format yaml` option for reading YAML files
- **CSV Output Support**: Add `--format csv` option for converting JSON arrays to CSV
- **Complete Format Matrix**: Full bidirectional conversion between JSON, YAML, and TOML, plus CSV output
- **Shell Completion**: Generate shell completion scripts with `--generate-completion`
- **Performance Benchmarks**: Add `--benchmark` flag for comprehensive performance testing
- **Streaming Input Processing**: Improved memory efficiency for large stdin inputs
- **Progress Indicators**: Show progress for large file processing (>5MB)
- **File Size Warnings**: Alert users about large files (>50MB)
- **Anime Themes**: Added 5 new anime-inspired color themes (sakura, cyberpunk, ghibli, evangelion)
- **Enhanced Error Handling**: Better UTF-8 validation and error messages

### Changed
- **Version**: Bumped to 0.2.0 (feature additions)
- **Config System**: Enhanced with format support and better defaults
- **CLI Options**: Improved help text and option descriptions
- **Performance**: Better memory usage for large JSON processing

### Fixed
- **Build Warnings**: Removed unused imports and fixed compilation warnings
- **Config Merging**: Fixed format field handling in configuration
- **Error Types**: Added Yaml error variant for proper error handling

### Documentation
- **README**: Updated with new features, examples, and badges
- **Themes**: Documented all 11 available color themes
- **Usage Examples**: Added YAML and shell completion examples

## [0.1.0] - 2025-12-01

### Added
- Initial release of jsonfizz
- JSON formatting and pretty printing
- Multiple color themes (default, solarized, mono, rainbow, ocean, forest, pastel)
- JSON path queries with `--get` option
- Depth limiting with `--max-depth`
- Configuration file support (~/.jsonfizz.toml)
- Stdin/file input support
- Basic error handling

### Features
- Fast JSON formatting written in Rust
- Terminal color output with customizable themes
- Configurable indentation and formatting options
- MIT/Apache-2.0 dual licensing