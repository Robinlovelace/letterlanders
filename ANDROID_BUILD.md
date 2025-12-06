# Building for Android

To build the `.apk` for Android, you need to set up the Android development environment (Java JDK, Android SDK, NDK).

## Prerequisites
1.  **Install Android Studio** and use the SDK Manager to install:
    *   Android SDK Platform
    *   Android SDK Platform-Tools
    *   NDK (Side-by-side)
    *   CMake
    *   Java JDK (version 17 or newer)

2.  **Set Environment Variables** (e.g., in `.bashrc`):
    ```bash
    export JAVA_HOME="/path/to/java/jdk"
    export ANDROID_HOME="$HOME/Android/Sdk"
    export NDK_HOME="$ANDROID_HOME/ndk/<version>"
    ```

3.  **Install Tauri Android Target**:
    ```bash
    rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
    ```

## Build Command
Initialize the Android project (run once):
```bash
cd app
npm run tauri android init
```

Build the APK:
```bash
npm run tauri android build
```
*   This will generate an APK at `app/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk`.
*   To sign it for the Play Store, follow the [Tauri Distribution Guide](https://tauri.app/v1/guides/distribution/sign-android).
