# Using the present working directory as a value for your environment variables
> 🕰️ logged on 2024.10.10 <br>
> 🔖 tagged: devops, shell, scripting, zsh, git, git-cliff

I use a [custom git-cliff configuration](https://github.com/bvdbasch/dotfiles/blob/master/roles/git/templates/cliff.toml.j2) for generating changelogs. The changelog it produces is fantastic, but unfortunately - by default - git-cliff does *not* export the generated changelog to a file.

There is an environment variable you can set called `GIT_CLIFF_OUTPUT` which will instruct git-cliff to always store the generated changelog in that location. Great, but what about multiple projects in multiple locations?

Well, I realized that when you're asking git-cliff to generate a changelog you are most likely already in the project folder, so using the current directory as the value for the output location would work. We know the `pwd` command (present working directory) produces the exact output we need, but simply putting this into an the environment variable **will not work**.

To be very clear: `GIT_CLIFF_OUTPUT="pwd/CHANGELOG.md"` will not do anything, because pwd gets treated as text and not as a command. So how do we make sure that it does get treated as a command? Simple: just wrap it in parenthesis and prepend it with a dollar sign `$(pwd)` :D

So the final solution is: `GIT_CLIFF_OUTPUT="$(pwd)/CHANGELOG.md"`, which will create the changelog in the current directory. Perfect.

## Sources
+ [This saves the absolute path of the current working directory to the variable cwd](https://stackoverflow.com/a/13275062)
