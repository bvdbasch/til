# Enable using Touch ID for sudo commands
> 🕰️ logged on 2023.05.17 <br>
> 🔖 tagged: apple, macos, sudo, terminal, touch-id

> [!IMPORTANT]
> This used to be done by editing the file `/etc/pam.d/sudo` but this file gets reset to default after changes such as security updates. Use the file `/etc/pam.d/sudo_local` instead!

> [!TIP]
> Check out the  Ansible template file [here](https://github.com/bvdbasch/dotfiles/blob/master/roles/macos/templates/sudo_local.j2) and the Ansible task [here](https://github.com/bvdbasch/dotfiles/blob/20df2f90e8e706cdb5a3de7f0980b99d91647809/roles/macos/tasks/main.yml#L14) to automate this with Ansible! 🤙

I was recently handed a new company Macbook with device management policies. A strong password policy is enforced on the machine (as it should). However, this is quite the hassle with sudo (spare my fingers plzzzz) so I needed a method to authorize sudo commands with Touch ID. Luckily this was really straightforward by following the steps underneath.

1. Create a file called `sudo_local.j2` in /etc/pam.d
2. Ensure the file has the following line: `auth       sufficient     pam_tid.so`
3. Restart your terminal
4. Done

## Sources
+ [🇳🇱 Touch ID gebruiken bij een sudo commando in Terminal en iTerm 2](https://www.appletips.nl/terminal-touch-id-sudo)
