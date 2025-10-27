```

cobot-rs main  ? ❯ espflash board-info
[2025-10-27T01:10:10Z INFO ] Serial port: '/dev/ttyUSB0'
[2025-10-27T01:10:10Z INFO ] Connecting...
[2025-10-27T01:10:14Z INFO ] Using flash stub
Chip type:         esp32 (revision v3.1)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi, BT, Dual Core, 240MHz, VRef calibration in efuse, Coding Scheme None
MAC address:       fc:b4:67:f1:90:c0
Security features: None
```


```
cobot-rs main  ? ❯ ls /dev/tty*
Permissions  Size User     Date Modified Name
crw-rw-rw-    5,0 root     26 Oct 21:59  󰡯 /dev/tty
..... ......
crw-rw----   4,94 root     24 Oct 14:44  󰡯 /dev/ttyS30
crw-rw----   4,95 root     24 Oct 14:44  󰡯 /dev/ttyS31
crw-rw----  188,0 root     26 Oct 22:10  󰡯 /dev/ttyUSB0
```

```
cobot-rs main  ? ❯ lsusb

Bus 001 Device 018: ID 10c4:ea60 Silicon Labs CP210x UART Bridge

```