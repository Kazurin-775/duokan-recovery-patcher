#!/system/bin/sh

echo -n 'Kernel command line: '
cat '/proc/cmdline'
echo -n 'SELinux status: '
getenforce

echo 'Done!'
