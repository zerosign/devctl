The Common Framework

- argument parsing library implementation : docopt see [docopt](https://github.com/docopt/)

CLI Command Extension

The idea of this section is to explain the behavior of central command line application
to be able to be extended.

docopt compgen (autocomplete generation from docopt to bash, zsh, fish), this autocomplete
reside inside the central `devctl` cli so that when installing new plugin it will traverse its `--help` output to generate new autocomplete file in local. (effect: install, update, delete). This will be implemented internally () by `devctl` and can be extended by CLI Command Extension flow.
Split into docopt (--help) parser (into metadata) and completion generator (by shell type).

Completion generated per plugin and generated ahead of time when you install the plugin.

Proposed command & arguments list for the main cli :

```
devctl --help
devctl --version
devctl init
devctl list
devctl update
devctl install [plugin:version|--git url] [--force]
devctl info [plugin[:version]]
devctl delete [plugin]
devctl search [plugin]
devctl compgen [plugin] [--force] [--args=[key=value]]
devctl [subcommands] [args]
```

subcommands will be redirected to `devctl`-[plugin] cli to handle the command.
we might also want to have an exclusive mapping setting to subcommands (alias).

subcommands resolutions :
- can handle subcommand directly as `devctl` args ?
- search in devops config
- search in search path (as devctl-`subcommand`)
- not found

Completion Links :
- https://github.com/zsh-users/zsh-completions/blob/master/zsh-completions-howto.org
- https://iridakos.com/tutorials/2018/03/01/bash-programmable-completion-tutorial.html
- https://fishshell.com/docs/current/commands.html#complete

Devops config (config.toml):


[[plugin]]
name = “check”
alias = "devctl-check-config"
parser = "docopt" # by default we use docopt

[[plugin]]
name = “kube”
alias = "kubectl"
parser = "kubectl-parser"
# version = “1.13”
env = { KUBECONFIG = “production” }

Just like any subcommands in here parser can be another program that output the metadata that can be used on autocomplete or internal function that already being defined in devctl cli.

- ex: devctl service list
this will propagate arguments ['list'] to program `devctl-service` if not found
then print the command not found and exit -1.

- ex: env SCALE=2 devctl deploy simpsons --env production
this will propagate arguments ['simpsons', '--env', 'production'] with its environment [SCALE=2, ...]
to `devctl-deploy` cli.

- ex: devctl kube apply -f deployment.yml

CLI Central Registry

We might need to have central registry in github repo so that we can track version
changes in each plugins (this might be as simple as json manifest or search into repository release).

Release management are using github release API [Github Release API](https://developer.github.com/v3/repos/releases/#list-releases-for-a-repository/)
