# Showing Swatch Internet time in your terminal (.beats)
> 🕰️ logged on 2025.03.17 <br>
> 🔖 tagged: devops, shell, scripting, zsh, swatch, internet-time, .beats

I am sucker for [Swatch Internet Time](https://www.wnyc.org/story/15-internet-time/) and wanted to add it to my zsh prompt.

Here is the command line magic that made it happen: `echo '@'$(TZ=GMT-1 date +'(%-S + %-M * 60 + %-H * 3600) / 86.4'|bc)`

## Sources
+ [Bash alias to output the current Swatch Internet Time](https://www.commandlinefu.com/commands/view/25323/bash-alias-to-output-the-current-swatch-internet-time)
