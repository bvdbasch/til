# Setting up a new Python virtual environment with Poetry and PyEnv
> 🕰️ logged on 2024.01.11 <br>
> 🔖 tagged: python, poetry, dependency-management, pyenv, venv

Today I had to set up a small python project from scratch rather than continuing from a pre-existing project. Since I've only had to deal with projects already containing a `pyproject.toml` this was a new experience for me. Here's how I did it

## Create the environment, add the dependencies and jump into the shell
> [!NOTE]
> Prerequisites: [PyEnv](https://github.com/pyenv/pyenv) and [Poetry](https://github.com/python-poetry/poetry)

```sh
$ cd <target_dir>
$ poetry init  # follow on the on screen instructions
$ poetry shell # spawn environment shell
```

## Sources
Figured this one out myself 🥳
