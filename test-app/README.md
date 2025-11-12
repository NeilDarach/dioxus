# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Start the simulator
```bash
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
```

Boot a phone
```bash
xcrun simctl boot "iPhone 16 Pro"
```

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform ios
```

To build a release IPA for signing:

```bash
dx bundle --platform ios --release --device true
edit target/dx/test-app/release/ios/test-app.app/info.plist
set CFBundleIdentifier to 'uk.org.darach.TestApp'
```

To sign the IPA for copying to the phone
```bash
brew install ios-app-signer
open /Applications/iOS App Signer.app
```
Input file: the app - test-app/target/dx/test-app/release/ios/TestApp.app
Signing Certificate: automatic
Provisioning Profile: Choose Custom File / ~/Libary/Developer/XCode/UserData/Provisioning Profiles/

Installing on the iphone
Back in XCode, press cmd-shift-2 to get the device list and drag the ipa file over


# Provisioning profile
To set up a signing profile in XCode (only required once)
1. Open a new XCode project, use all defaults
1. Set the app bundle id to match this app (uk.org.darach.TestApp)
1. On the projects settings page set the minimum ios version to 18.6
1. Set up a team on the signing panel of the project
1. Build the app, which should create a signing profile in ~/Library/Developer/Xcode/UserData/Provisioning\ Profiles like d66bb841-409c-4dce-a546-f2d148793109.mobileprovision

# References
Notes on signing the app - https://github.com/DioxusLabs/dioxus/discussions/3545
