# smallauncher

![action](https://github.com/smallauncher/smallauncher/actions/workflows/release.yaml/badge.svg)

minimal cli minecraft(java) launcher

smallauncher downloads and run minecraft for you

### download minecraft
the correct version of java for the game is automatically downloaded
```sh
smallauncher -d <version>
```

### join in your minecraft account
```sh
smallauncher -a
```

### run the game
```sh
smallauncher -r <version> <username>
```

### full set of commands
```sh
smallauncher -d   --download <version>
smallauncher -c   --check    <version>
smallauncher -r   --run      <version> <user_name>
smallauncher -s   --search   <version>
smallauncher -l   --list
smallauncher -la  --list-all
smallauncher -a   --authenticate
```

### install

#### using cargo
```sh
cargo install smallauncher
```

#### manual instalation
download the [latest version](https://github.com/smallauncher/smallauncher/releases/latest) of the launcher and add binary to the path

needs help? join our [discord](https://discord.gg/yGJCn8P2yY)

Dual-licensed under [MIT](../LICENSE-MIT) or the [UNLICENSE](../UNLICENSE).