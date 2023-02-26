+++
title = "How to repair a Raspberry Pi disk image"
date = "2023-02-22T20:34:30+02:00"
author = ""
authorTwitter = "" #do not include @
cover = ""
tags = []
keywords = ["", ""]
description = ""
showFullContent = true
+++

Issue: the Raspberry Pi (RPi) has crashed and the boot process ends up into a kernel panic. The reason is unclear.
I am connecting the serial output from the RPi to a FTDI to see the issue on the screen. Which kind of FTDI or USB-to-serial board you are using is not important. A cheap model is fine.
You must connect the FTDI pins to the RPi pins this way
| FTDI Pin | RPi Pin | RPi Pin Name | 
| -------- | ------- | ------------ |
| GND      | 06      | Ground       |
| TX (D0)  | 08      | TX (GPIO14)  |
| RX (D1)  | 10      | RX (GPIO15)  |

Please refer to this resources for more information how to connect the serial output from the RPi:
* https://unix.stackexchange.com/questions/208260/how-to-scroll-up-after-a-kernel-panic
* https://unix.stackexchange.com/questions/307390/what-is-the-difference-between-ttys0-ttyusb0-and-ttyama0-in-linux/367882#367882
* https://learn.adafruit.com/adafruit-ft232h-breakout/serial-uart
* https://sites.google.com/site/cartwrightraspberrypiprojects/home/steps/macbook-usb-to-raspberry-pi-using-ftdi
* https://makezine.com/article/technology/raspberry-pi/talking-to-the-raspberry-pis-serial-console-with-an-ftdi-breakout-board/

Configure the RPi boot using the cmdline.txt file. My example:
```
console=serial0,115200 console=tty1 earlycon=pl011,mmio32,0x3f201000 root=PARTUUID=6c586e13-02 rootfstype=ext4 elevator=deadline fsck.repair=yes rootwait
```

Add this to the /boot/config.txt in order to disable the mini UART and enable the real UART
```
dtoverlay=disable-bt
```

Please refer to this resources for more information how to configure the RPi for a serial output:
* https://www.abelectronics.co.uk/kb/article/1035/serial-port-setup-in-raspberry-pi-os


Boot the RPi.

Open the serial line using minicom. Tools like screen works as well
```
minicom -b 115200 -D /dev/ttyUSB0
```

You should see the boot output. I got a random kernel panic. So that I decided to rescue the data on my sdcard.
```
dd if=/dev/sde of=sde-card-copy.img_new bs=4096 conv=notrunc,noerror status=progress
```

Another option is ddrescue, especially if your disk is damaged

```
ddrescue -n -r 3 /dev/sdc sdc-card-copy.img_ddrescue2 mapfile2
```

Please refer to this resources for more information about a disk backup using dd:
* https://askubuntu.com/questions/227924/sd-card-cloning-using-the-dd-command
* https://superuser.com/questions/622541/what-does-dd-conv-sync-noerror-do
* https://datarecovery.com/rd/how-to-clone-hard-disks-with-ddrescue/
* https://www.linux.com/topic/desktop/gnu-ddrescue-best-damaged-drive-rescue/
* https://www.technibble.com/guide-using-ddrescue-recover-data/

Mounting the device

* https://unix.stackexchange.com/questions/316401/how-to-mount-a-disk-image-from-the-command-line
