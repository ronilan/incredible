@echo off

set PLATFORM=%1
shift

if "%PLATFORM%"=="--terminal" (
    cargo run --bin incredible -- %*
)

if "%PLATFORM%"=="--macos" (
    cargo run --bin incredible_macos --features macos-native -- %*
)

if "%PLATFORM%"=="--windows" (
    cargo run --bin incredible_windows --features windows-native -- %*
)

if "%PLATFORM%"=="--wasm" (
    pnpm run dev -- %*
)