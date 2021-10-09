## What this project does

This project is for **Xiaomi Mi Duokan Reader Pro**, whose recovery accepts `update.zip`s signed by a test key, allowing an end user to root the device with a crafted `update.zip`. For more information, consider reading [this](https://github.com/qwerty12/inkPalm-5-EPD105-root).

This project provides a flashable `update.zip` that creates and boots a **pwned recovery image** on-the-fly, which further removes the restrictions of the recovery. The new, pwned recovery has the following new features enabled:

- SELinux permissive
- ADB enabled &amp; rooted
- ADB authorization removed

This allows the user to do more powerful things, such as dumping the boot image to `/data/media`, without the restrictions of SELinux.

This project originally targets the Xiaomi Mi Duokan Reader Pro, but it should work on most devices that allows the user to sideload a custom `update.zip`, with a small amount of modification.
