# Mechanism For Bendis

## Overview

`Bendis` is a patch or terminal tool which let us use `bender` better in Heris project. 

## Mechanism

1. User use `bendis init` to initialize the project (only need to do once if there are no both `.bendis` folder and all configuration files yet). So now Bendis will create `.bendis/` folder and blank `Bender.yml` and `.bender.yml` files in this folder. If there are already `.bendis/Bender.yml` and `.bendis/.bender.yml` files with any contents (If they're blank it's ok), Bendis will deny the initialization and ask user to backup or delete them first.
2. User use `bendis update`. Bendis will run `bender -d ./.bendis update` first to make sure `.bendis/Bender.lock` is generated and up-to-date. Also, Bender will generate a `.bender/` folder in `.bendis/` folder to store dependencies (but later it will be removed by Bendis).
3. Then, Bendis will run `format_converter.py` script (Note, this python scripts is still not very good and suitable for bendis, and also, not very fast, so you need to modify, maybe modify or convert to another language), taking `.bendis/Bender.yml`, `.bendis/.bender.yml` and `.bendis/Bender.lock` files as input filepack, and copy `Bender.yml` and generate the new `.bender.yml` files in the root of the project.
4. Bendis will rerun `bender update` in the root of the project, now using the newly generated `Bender.yml` and `.bender.yml` files, generating a new `Bender.lock` file. Also, newly `.bender/` will be generated in the root of the project to store newly dependencies.
5. Bendis will remove the `.bender/` folder in `.bendis/`.

## Others Notes

1. If user use cmd otherwise rather than defined commands, Bendis will use Bender to run them. Which means, bendis is just a wrapper and patch tool for Bender.
2. Bendis also provides `bendis --version` and `bendis -v` and `bendis --help` and `bendis -h` commands to show version and help message.
3. Also, Bendis provides a `settings.sh` to source, so that users just need to add `source /path/to/bendis/settings.sh` in their `~/.bashrc` or `~/.zshrc` file to use Bender and Bendis commands directly in terminal.