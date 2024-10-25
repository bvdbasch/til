# TIL: Today I Learned
> [!NOTE]
> 7 TILs in 4 different categories and counting...

This repository contains a collection of concise write-ups on small things I've learned in my day to day journey as a developer. They cover a wide variety of languages and technologies. These are things that don't really warrant full-fledged blog posts. In fact, most of the entries in this repository either answer a simple question and are more of a how-to document than they are a journal type entry.

## The playground folder
I also keep track of my more off-the-cuff style experiments in this repository. I thought it would be nice to have a public archive of my experiments on the road to becoming a better developer. I have bundled these undertakings in the [📁 playground folder](/playground).

## Usage (aka I want this, how do I get it?)
> [!IMPORTANT]
> Be sure to switch to the [`scaffold` branch](https://github.com/bvdbasch/til/tree/scaffold) of this repository when creating your own version of this repository.

If you want to start keeping track of your own journey, feel free to fork this repository 👍

After forking the repository the workflow is roughly:

1. Write a new entry in the file entries.yml whenever you learn something (ideally you do this every day).
2. Run the Ansible playbook called `generate.yml`. You can do so by running the following command: `ansible-playbook generate.yml`

### Tweaking the output
The Ansible playbook generates all the Markdown files you see in this repository. Ansible uses the [Jinja2 template engine](https://docs.ansible.com/ansible/latest/playbook_guide/playbooks_templating.html) to this end.

This repository contains the following two template files:

1. [A template for the main README](README.md.j2) - responsible for the file you are reading right now
2. [A template for individual entries](entry.md.j2) - responsible for the files in the individual folders

Make any change you see fit to those templates and then run the Ansible playbook again to see your changes take effect. Of course [don't forget to show me the awesome changes you make](https://github.com/bvdbasch/til/discussions/categories/show-and-tell) 💪 (I want to know 😳). And if you're stuck don't be affraid to [head over to the Q&A category of the discussions](https://github.com/bvdbasch/til/discussions/categories/q-a) in this repository and feel free ask for help!

## Inspiration
I became inspired to make this repository [after stumbling upon](https://news.ycombinator.com/item?id=11068902) the [awesome TIL repo](https://github.com/jbranchaud/til) maintained by [Josh Branchaud](https://github.com/jbranchaud). I was blown away by what a small daily effort can produce over the course of several years! Mad respect for showing us the way my dude 🤵🏻‍♂️

## Categories

+ [Linux](#linux)
+ [Macos](#macos)
+ [Python](#python)
+ [Shell](#shell)

### Linux

+ [Using the present working directory as a value for your environment variables](linux/using-the-present-working-directory-as-a-value-for-your-environment-variables.md)
+ [Viewing the status of a specific service](linux/viewing-the-status-of-a-specific-service.md)

### Macos

+ [Change the default folder when saving screenshots on MacOS](macos/change-the-default-folder-when-saving-screenshots-on-macos.md)
+ [Make TextEdit.app on MacOS behave more like Windows Notepad (notepad.exe)](macos/make-textedit-app-on-macos-behave-more-like-windows-notepad--notepad-exe-.md)
+ [Pin a file or folder to the dock in MacOS](macos/pin-a-file-or-folder-to-the-dock-in-macos.md)

### Python

+ [Setting up a new Python virtual environment with Poetry and PyEnv](python/setting-up-a-new-python-virtual-environment-with-poetry-and-pyenv.md)

### Shell

+ [Install any zsh plugin with antigen directly from a github repository](shell/install-any-zsh-plugin-with-antigen-directly-from-a-github-repository.md)
