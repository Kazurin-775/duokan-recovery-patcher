#!/system/bin/sh

# the following script doesn't have any debug output; just check the output yourself
export PATH="$PATH:/system/bin"

mount -t 'f2fs' '/dev/block/by-name/UDISK' '/data' || exit 1

dd 'if=/dev/block/by-name/boot' 'of=/data/media/0/boot.img'
chown 1023.1023 '/data/media/0/boot.img'
chmod 775 '/data/media/0/boot.img'
chcon 'u:object_r:media_rw_data_file:s0:c207,c256,c512,c768' '/data/media/0/boot.img'

stat '/data/media/0/boot.img'

umount '/data'
