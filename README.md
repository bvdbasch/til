# TIL: Today I Learned
## How to use the Ansible playbook
> [!TIP]
> You can install Ansible without much hassle via pipx. To do so run the following command in your terminal `pipx pipx install --include-deps ansible`

This branch (`scaffold`) only contains the files you need to populate your own repository with your write-ups. Your workflow should be the similar to the one below:

1. Write a new entry in the `entries.yml` whenever you learn something (ideally you do this every day).
2. Run the Ansible playbook called `generate.yml`. You can do so by running the following command: `ansible-playbook generate.yml`
3. Stage, commit, and push your changes
4. That's it. You're done. Time to go learn new stuff 😉

### Tweaking the output
The Ansible playbook generates all the Markdown files you see in this repository. Ansible uses the [Jinja2 template engine](https://docs.ansible.com/ansible/latest/playbook_guide/playbooks_templating.html) to this end.

This repository contains the following two template files:

1. [A template for the main README](README.md.j2) - responsible for the file you are reading right now
2. [A template for individual entries](entry.md.j2) - responsible for the files in the individual folders

Make any change you see fit to those templates and then run the Ansible playbook again to see your changes take effect. Of course [don't forget to show me the awesome changes you make](https://github.com/bvdbasch/til/discussions/categories/show-and-tell) 💪 (I want to know 😳). And if you're stuck don't be affraid to [head over to the Q&A category of the discussions](https://github.com/bvdbasch/til/discussions/categories/q-a) in this repository and feel free ask for help!
