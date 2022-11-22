# Linking Test of if-watch Crate for iOS

The linking of the [if-watch](https://github.com/mxinden/if-watch) crate
for iOS is failing due to the use of some MacOS API which is not available
on iOS.
Unfortunately the compilation of it is working fine and it only
fails at the linking step.

This repository shall help to investigate and fix this problem.

## Build this repository

### Build iOS Static Library

```sh
# change to the watchlib project folder
cd watchlib

# compile the static library for iOS
cargo lipo

# copy the compiled library to the iOS project's libs folder
cp target/universal/debug/libwatchlib.a ../watchapp/libs/
```

### Build iOS App

Open the iOS project in Xcode and run it.
You will get the linking errors documented below.

Or if you'd like to build the app from the command line:

```sh
# move to the watchapp project's folder
cd watchapp

# archive (build) the project
xcodebuild -project watchapp.xcodeproj -scheme watchapp -destination generic/platform=iOS build
```

#### Error when Linking 

The app fails linking with the following linking error:

```
Undefined symbol: _SCDynamicStoreCopyProxies
Undefined symbol: _SCDynamicStoreCreateRunLoopSource
Undefined symbol: _SCDynamicStoreCreateWithOptions
Undefined symbol: _SCDynamicStoreSetNotificationKeys
Undefined symbol: _kSCDynamicStoreUseSessionKeys
```

All of these belong to the MacOS API which are not available on iOS.

Please also consult this issue, in which we documented the problem in detail:
<https://github.com/qaul/qaul.net/issues/395>

The error output when run via the commandline is:

```
Ld /Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Products/Debug-iphoneos/watchapp.app/watchapp normal (in target 'watchapp' from project 'watchapp')
    cd /Users/foton/src/meshnet/examples/if-watch-ios/watchapp
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang -target arm64-apple-ios15.0 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS16.1.sdk -L/Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Intermediates.noindex/EagerLinkingTBDs -L/Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Products/Debug-iphoneos -Llibs -L/Users/foton/src/meshnet/examples/if-watch-ios/watchapp/libs -L/Users/foton/src/meshnet/examples/if-watch-ios/watchapp/libs -F/Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Intermediates.noindex/EagerLinkingTBDs -F/Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Products/Debug-iphoneos -filelist /Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Intermediates.noindex/watchapp.build/Debug-iphoneos/watchapp.build/Objects-normal/arm64/watchapp.LinkFileList -Xlinker -rpath -Xlinker @executable_path/Frameworks -dead_strip -Xlinker -object_path_lto -Xlinker /Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Intermediates.noindex/watchapp.build/Debug-iphoneos/watchapp.build/Objects-normal/arm64/watchapp_lto.o -Xlinker -export_dynamic -Xlinker -no_deduplicate -fobjc-link-runtime -L/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/iphoneos -L/usr/lib/swift -Xlinker -add_ast_path -Xlinker /Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Intermediates.noindex/watchapp.build/Debug-iphoneos/watchapp.build/Objects-normal/arm64/watchapp.swiftmodule -lwatchlib -Xlinker -no_adhoc_codesign -Xlinker -dependency_info -Xlinker /Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Intermediates.noindex/watchapp.build/Debug-iphoneos/watchapp.build/Objects-normal/arm64/watchapp_dependency_info.dat -o /Users/foton/Library/Developer/Xcode/DerivedData/watchapp-ghbxjguzxomyimezpzrqtbuyjbgo/Build/Products/Debug-iphoneos/watchapp.app/watchapp
Undefined symbols for architecture arm64:
  "_SCDynamicStoreCopyProxies", referenced from:
      system_configuration::dynamic_store::SCDynamicStore::get_proxies::hfc2e4f3c8ae2289f in libwatchlib.a(system_configuration-8e47f92b21843f5d.system_configuration.97b2cfb2-cgu.7.rcgu.o)
  "_SCDynamicStoreCreateRunLoopSource", referenced from:
      system_configuration::dynamic_store::SCDynamicStore::create_run_loop_source::h2b642b4339c1830c in libwatchlib.a(system_configuration-8e47f92b21843f5d.system_configuration.97b2cfb2-cgu.7.rcgu.o)
  "_SCDynamicStoreCreateWithOptions", referenced from:
      system_configuration::dynamic_store::SCDynamicStore::create::hf9c8ca236cf5137d in libwatchlib.a(system_configuration-8e47f92b21843f5d.system_configuration.97b2cfb2-cgu.7.rcgu.o)
  "_SCDynamicStoreSetNotificationKeys", referenced from:
      system_configuration::dynamic_store::SCDynamicStore::set_notification_keys::hc48e9fe1432ab393 in libwatchlib.a(if_watch-4a84650253756901.if_watch.77ef42d5-cgu.7.rcgu.o)
  "_kSCDynamicStoreUseSessionKeys", referenced from:
      system_configuration::dynamic_store::SCDynamicStoreBuilder$LT$T$GT$::create_store_options::h2baa5254ca546921 in libwatchlib.a(if_watch-4a84650253756901.if_watch.77ef42d5-cgu.7.rcgu.o)
ld: symbol(s) not found for architecture arm64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```
