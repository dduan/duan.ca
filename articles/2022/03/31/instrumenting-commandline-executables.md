# Using Instruments on Non-macOS, Non-iOS Executables
2022-03-31T20:39:05-07:00
tag: Instruments, SwiftPM, Swift

How do you use use Instruments, the instrumentation tool distributed alongside Xcode? It's easy if you are
profiling a macOS or iOS app. No so with a regular old command-line tool built, say, with SwiftPM. To do so,
you'll need to codesign the target binary. Otherwise Instruments will give you a `"Failed to gain
authorization"` error message.

Here's the steps:

1. Create a entitlement file with `com.apple.security.get-task-allow` set to true:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict>
        <key>com.apple.security.get-task-allow</key>
        <true/>
    </dict>
</plist>
```

Let's call it `Entitlements.plist`

2. Build the executable.

3. Codesign your executable with the entitlement file

```
codesign -s - -f --entitlements path/to/Entitlements.plist path/to/executable
```

Now proceed with your profiling in Instruments.

Not sure where this is officially documented. So I'm officially documenting it here.
