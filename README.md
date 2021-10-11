## The official boot image (for end users)

警告：如果您没有足够多的 Android 背景知识，请不要轻易尝试这里提供的任何东西！本项目的目标设备（小米多看阅读器 Pro）暂时还没有官方提供的恢复镜像文件，因此一旦有任何疏忽，您的设备将会变砖且无法恢复！

Warning: DO NOT use anything here if you don't know what you're doing! The device (Xiaomi Duokan Reader Pro) does not have a factory image provided by Xiaomi yet. Therefore, if you do anything wrong, you may leave your device in a bricked, non-restorable state.

If you understand everything above, this is the official `boot.img` of Xiaomi Duokan Reader **Pro**: [boot-virgo-perf1-1.1.8.210610.7z](https://github.com/Kazurin-775/duokan-rooter/releases/download/publish/boot-virgo-perf1-1.1.8.210610.7z)

- Device name: Xiaomi Duokan Reader **Pro**
- Codename (`ro.build.product`): `virgo-perf1`
- System version: MiReaderPro 1.1.8.210610
- Build fingerprint (`ro.build.fingerprint`): `Allwinner/virgo_perf1/virgo-perf1:8.1.0/OPM1.171019.026/20210610-121349:user/test-keys`

For users who want to root their devices, simply download the `boot.img`, patch it using the Magisk app, and then boot the patched image on the device.

Since `fastboot boot` does not work properly on this device, it is required to flash a patched image to the device in order to be able to boot it. For safety reasons, I strongly recommend flashing everything to `recovery` instead of `boot`. This is because the Android OS reflashes the official recovery image at every boot, and thus, restoring from a bad state is as easy as forcefully rebooting the device. In contrast, if you flashed a bad image to `boot`, there would be no way back (without a factory image provided by Xiaomi).

## What this project does

This project is for **Xiaomi Duokan Reader Pro**, whose recovery accepts `update.zip`s signed by a test key, allowing an end user to root the device with a custom `update.zip`. It should also be suitable for most Allwinner-based E-ink readers, since they all use a test key to sign `update.zip`s. For more information, consider reading [this comprehensive article about rooting Allwinner E-ink devices](https://github.com/qwerty12/inkPalm-5-EPD105-root).

This project provides a flashable `update.zip` that creates and boots a **pwned recovery image** on-the-fly, which further removes the restrictions of the official Android recovery. The new, pwned recovery has the following new features enabled:

- SELinux permissive
- ADB enabled &amp; rooted
- ADB authorization removed

This allows the user to do more powerful things, such as dumping the boot image to `/data/media`, without the restrictions of SELinux.

This project originally targets the Xiaomi Duokan Reader Pro, but it should work on most devices that allow the user to sideload a custom `update.zip`, with a small amount of modification.

## Example of a pwned recovery

![](images/pwned_recovery.jpg)

## Technical details

Work in progress...

## Special thanks to

- The [Magisk](https://github.com/topjohnwu/Magisk) installer by [topjohnwu](https://github.com/topjohnwu) and others, where all the ideas of this project come from
- [qwerty12](https://github.com/qwerty12) for his [comprehensive article about the rooting of inkPalm 5](https://github.com/qwerty12/inkPalm-5-EPD105-root)

## Calling for help

If you have, or you know how to do any of the following things on this device, please inform me (via issues / discussions / etc.):

- Using `fastboot boot` to boot a custom boot image
- Entering recovery or bootloader from a power-off state
- Any official firmware image or guidance on debricking released by Xiaomi
