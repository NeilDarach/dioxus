
_default:
    @just --list

start:
    open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
    xcrun simctl boot "Phone"
sim:
    cargo build --target x86_64-apple-ios

build:
    cd test-app && dx bundle --platform ios --release --device true
    just patch

patch:
    [[ -d test-app/target/dx/test-app/release ]] && cp 'test-app/assets/AppIcon*.png' test-app/target/dx/test-app/release/ios/TestApp.app || true
    [[ -d test-app/target/dx/test-app/debug ]] && cp 'test-app/assets/AppIcon*.png' test-app/target/dx/test-app/debug/ios/TestApp.app || true
