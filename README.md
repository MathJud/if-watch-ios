# Linking Test for libqaul

libqaul can be successfully linked in this dummy app by doing
the following steps:

1) compile libqaul
2) copy `liblibqaul.a` to `watchapp/libs/` folder.
3) open watchapp in Xcode and click `Product > Archive`

## Build this repository

### Build iOS Static Library

```sh
# copile libqaul for iOS
cargo build -p libqaul --target aarch64-apple-ios --release --lib --verbose --no-default-features

# copy the compiled library to the iOS project's libs folder
```

### Build iOS App

* Open the iOS project in Xcode.
* Click `Product > Archive`


## Strange Things

The linking only succeeds when running `Product > Archive` via Xcode's GUI.
When trying to run it via Xcode, it failes with the linking error, 
the same goes when trying to build or run it from the command line.
