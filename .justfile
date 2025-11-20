
_default:
    @just --list

start:
    open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
    xcrun simctl boot "Phone"
sim:
    cargo build --target x86_64-apple-ios

build:
    cd Scoreboard && dx bundle --platform ios --release --device true --out-dir .
    cd Scoreboard && ../scripts/sign.sh Scoreboard.app "dobson1@mac.com" ~/Library/Developer/Xcode/UserData/Provisioning\ Profiles/d66bb841-409c-4dce-a546-f2d148793109.mobileprovision
    cd Scoreboard && mv uk.org.darach.DummyApp-signed.ipa Scoreboard.ipa
    cd Scoreboard && rm -rf Scoreboard Scoreboard.app 
