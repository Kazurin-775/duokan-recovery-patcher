#!/system/bin/sh

echo 'Preparing...'
exec 2>&1
export PATH="$PATH:/system/bin"
cd '/tmp'
chmod 755 'magiskboot'

echo 'Dumping recovery image...'
dd 'if=/dev/block/by-name/recovery' 'of=boot.img' || exit 1

echo 'Unpacking recovery image...'
./magiskboot unpack 'boot.img' || exit 1

echo 'Done!'
