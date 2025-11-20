
_default:
    @just --list

start:
    open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
    xcrun simctl boot "Phone"
sim:
    cargo build --target x86_64-apple-ios

build:
    cd Scoreboard && dx bundle --platform ios --release --device true --out-dir .
    just sign

sign:
    cd Scoreboard && ../scripts/sign.sh Scoreboard.app "dobson1@mac.com" <( sops --decrypt --output-type binary ../mobileprovisions/default.mobileprovision.enc )
    cd Scoreboard && mv uk.org.darach.DummyApp-signed.ipa Scoreboard.ipa
    cd Scoreboard && rm -rf Scoreboard Scoreboard.app 

encrypt src dest:
    sops --encrypt --age "$(cat ~/.ssh/id_ed25519.pub)" --input-type binary "{{src}}"  > "{{dest}}"

