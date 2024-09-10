# emg-filter-rs
This is an implementation of the [EMGFilters](https://github.com/oymotion/EMGFilters) library by OYMotion in Rust.

## Overview
Just like the OYMotion library, this library provides the following filters for processing sEMG signals.
* Anti-hum notch filter - for filtering out 50HZ or 60Hz power line noise
* Lowpass filter - filtering out noises above 150HZ
* Highpass filter - filtering out noises below 20HZ

Only input frequencies of 500HZ and 1000HZ are supported, but more can be added as needed.

## How to use
Use the library from GitHub by adding the following to your `Cargo.toml`
```tool
emg-filter-rs = { git = "https://github.com/Devils-Prosthetics/emg-filter-rs" }
```

## Differences from [EMGFilters](https://github.com/oymotion/EMGFilters) library by OYMotion
* It fixes issues, when used with multiple instances [#7](https://github.com/oymotion/EMGFilters/issues/7).
* It doesn't round from float to int multiple times unnecessarily, as a result, it should be more accurate.
