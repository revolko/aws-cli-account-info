# aws-cli-account-info
Rust CLI tool for Command Prompt integration.

There is no magic. The tool simply reads `AWS_ACCOUNT_ID` environment variable. Thus, you need to set the variable yourself.
If compatible with your AWS account, you can use my other tool [adfs-rs](https://github.com/revolko/adfs-rs).

## Installation
```bash
cargo install aws-cli-account-info
```

## ZSH example
```zsh
PROMPT='%(!.%{$fg[red]%}.%{$fg[green]%})%~$(git_prompt_info)%{$reset_color%} %{$fg_bold[yellow]%}$(aws-cli-account-info)%{$reset_color%}'

ZSH_THEME_GIT_PROMPT_PREFIX=" %{$fg_bold[blue]%}("
ZSH_THEME_GIT_PROMPT_SUFFIX="%{$fg_bold[blue]%})"
ZSH_THEME_GIT_PROMPT_DIRTY=" %{$fg[red]%}✗"
ZSH_THEME_GIT_PROMPT_CLEAN=" %{$fg[green]%}✔"
```

## Alternative names
It is possible to define custom alternative names for accounts ids.

Create a file at `$XDG_CONFIG_HOME/aws_cli_account_info/accounts.yaml`. Note, if not set, the default value for
`$XDG_CONFIG_HOME` is `$HOME/.config`.

The `accounts.yaml` file expects the following structure:
```yaml
accounts:
  123456789: my-prod
```

If `account id` matches the key in `accounts` map, it will print the alternative name instead.
