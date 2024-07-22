# Install any zsh plugin with antigen directly from a github repository
> 🕰️ logged on 2024.01.24
> 🔖 tagged: shell, unix, zsh, plugins, antigen

Sometimes a shell plugin has instructions on how to install itself with antigen. For example [popstas/zsh-command-time](https://github.com/popstas/zsh-command-time) has the following instructions [in their README](https://github.com/popstas/zsh-command-time?tab=readme-ov-file#install-with-antigen):

> ```Markdown
> # Install with antigen
> `antigen bundle popstas/zsh-command-time`
> ```

The above is a fairly common way of adding a plugin to your shell. But what do you do when you stumble upon a repository of a plugin that does not have this? Like the [tom-auger/cmdtime repository](https://github.com/tom-auger/cmdtime). When you try: `antigen bundle tom-auger/cmdtime` you'll see the following error `Installing tom-auger/cmdtime... Error! Activate logging and try again.` staring right at you!

What then is the solution? A quite straightforward actually:

1. Locate the antigen plugin directory on your harddrive
2. Clone the repository directly in that directory
3. Reload your shell and voila! Now you can use the plugin

## Sources
Figured this one out myself 🥳
