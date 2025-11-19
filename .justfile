
_default:
    @just --list

start:
    open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
    xcrun simctl boot "Phone"
sim:
    cargo build --target x86_64-apple-ios

build:
    cd test-app && dx bundle --platform ios --release --device true
