# Changelog

## 1.0.0 - 2026-02-19

### Breaking Changes
- Renamed `ContainerDimensions` struct to `AssembledDimensions` (affects `GeneratedSvg.container_dimensions` field type)
- Renamed `Dimensions` struct to `ContainerDimensions` for clarity

### Added
- Added calculation and display of assembled container rack dimensions
- Assembled dimensions (width, height, depth) are now calculated and returned in `GeneratedSvg`
- CLI now displays actual assembled dimensions instead of zeros

### Changed
- Updated dependencies

## 0.3.3 - 2025-05-21

- Updated dependencies

## 0.3.2 - 2024-10-13

- Added URL for the web UI to README

## 0.3.1 - 2024-10-13

- Updated installation and usage instractions in README

## 0.3.0 - 2024-10-13

- Refactor default generation command to own `generate` command. Added command to list supported containers.
- New reqquired argument for generation command: `--containers`
- Container has now it's own struct which allows to support more containers
- Added Dependabot config
