# Securely wipe a disk in Linux
> 🕰️ logged on 2024.12.02 <br>
> 🔖 tagged: linux, unix, shell, data-protection

New laptop. Bring back old. Always do zhe wipe! Forgot how to do it because it has been so long.

Many different utils, but shred be my fav.

My process:
- rm files from terminal first
- then wipe entire disk with shred.
- I use this command: `shred <disk> -
- sudo shred -v -n 1 -z /dev/sda
verbose so we see the progress, 1 pass is enough, do a last pass with all 0 to make it look like a blank disk

## Sources
+ [3 Best Ways to Securely Wipe Disk in Linux](https://linuxiac.com/best-ways-to-securely-erase-disk-in-linux/)
+ [How to Securely Erase a Disk and File using the Linux shred Command](https://www.freecodecamp.org/news/securely-erasing-a-disk-and-file-using-linux-command-shred/)
