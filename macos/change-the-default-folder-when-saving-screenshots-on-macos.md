# Change the default folder when saving screenshots on MacOS
> 🕰️ logged on 2024.07.02 <br>
> 🔖 tagged: apple, macos, screenshots, plist

By default MacOS saves all screenshots to your desktop. This may be fine for some, but for me it is far from ideal. I'd rather have th e screenshots saved to a dedicated screenshots folder. The commands below achieve that.

```zsh
$ defaults write com.apple.screencapture location <new_folder_location>
$ killall SystemUIServer
```

The first command `defaults write ...` sets the new default location

The second command `killall` kills the lingering processes, making sure that the new location takes effect the next time you take a screenshot.

## Bonus
I keep forgetting that the screenshot utility has some nifty keyboard shortcuts, so I've jotted the ones I'm interested in here:

1. When you take a screenshot with `cmd + shift + 4` you can press `space`` to toggle between region and full window for your current screenshot
2. Use `cmd + shift + 3` to take a screenshot of the whole flippin' screen

## Sources
+ [MacOS screenshot utility keyboard shortcuts](https://support.apple.com/guide/mac-help/take-a-screenshot-mh26782/mac)
+ [How to change the default screenshot save location](http://osxdaily.com/2011/01/26/change-the-screenshot-save-file-location-in-mac-os-x/)
