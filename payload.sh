#!/system/bin/sh

echo 'Preparing...'
exec 2>&1
export PATH="$PATH:/system/bin"
cd '/tmp'
chmod 755 'magiskboot'

echo 'Dumping recovery image...'
dd 'if=/dev/block/by-name/recovery' 'of=boot.img' || exit 1

# the following patch causes the recovery image to fail to boot
### echo 'Applying cmdline patch...'
### ./magiskboot hexpatch 'boot.img' '73656C696E75783D31' '73656C696E75783D30' || echo '... failed'

echo 'Unpacking recovery image...'
./magiskboot unpack 'boot.img' || exit 1

echo 'Applying black magic...'
./magiskboot hexpatch 'ramdisk.cpio' '484646F052FD' '484600BF00BF' || echo '... pattern 1 failed'
./magiskboot hexpatch 'ramdisk.cpio' 'FFFFFFFF01000000D1430200' 'FFFFFFFF00000000D1430200' || echo '... pattern 2 failed'
./magiskboot hexpatch 'ramdisk.cpio' '012805D00120' '012805D00020' || echo '... pattern 3 failed'
./magiskboot hexpatch 'ramdisk.cpio' '48464FF4FA61' '48464FF00001' || echo '... pattern 4 failed'
./magiskboot hexpatch 'ramdisk.cpio' '4846C0220023' '484600220023' || echo '... pattern 5 failed'
./magiskboot hexpatch 'ramdisk.cpio' '78440068007818B1' '78440068007803E0' || echo '... pattern 6 failed'

echo 'Repacking boot image...'
./magiskboot repack 'boot.img' || exit 1

echo 'Action!'
# flash the pwned recovery image
dd 'if=new-boot.img' 'of=/dev/block/by-name/recovery' || exit 1
# reboot to recovery after 5 seconds
sync
sleep 5
reboot recovery

echo 'Done!'
