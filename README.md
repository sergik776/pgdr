# CLI Password Generator with entropy by linux cernel (/dev/random)<br>
## PGDR - **P**assword **G**enerator by /**d**ev/**r**andom<br>

## Features
- **Dual entropy sources** — `/dev/urandom` (fast) or `/dev/random` (strict)
- **Flexible charset control** — letters, numbers, symbols
- **Lightning fast** — 215ms for 10M characters 

## Install

### By AUR
```sh
yay -S pgdr
```

### By Cargo
```sh
cargo install --git ssh://git@github.com/sergik776/pgdr.git
pgdr -U -L -S -N
```

**If you got the error:**<br>
`error: failed to clone into`<br>
`failed to authenticate when downloading repository`<br>

The Quickest Fix: Use the Git CLI<br>
As the error message suggested, you can tell Cargo to use your installed Git binary instead of its internal library. This will use your existing SSH configuration and ssh-agent automatically.

Run this command:
Bash
```sh
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << EOF
[net]
git-fetch-with-cli = true
EOF
```
**If .cargo/bin is not added to the $HOME variable:**
```sh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### By download git repo

```sh
git clone git@github.com:sergik776/pgdr.git
cd pgdr
cargo run -- -U -L -S -N
```

## Fast start
**Default params:** <br>
entropy source - /dev/urandom<br>
password length - 16<br>
```sh
pgdr -L -U -S -N     
```

## Options
| Flag | Description |
|------|-------------|
| `-l <N>` | Password length [DEFAULT: 16] |
| `-L` | Lowercase `a-z` only |
| `-U` | Uppercase `A-Z` only |
| `-N` | Numbers `0-9` |
| `-S` | Symbols `!@#$%^&*()_+-=` |
| `-r` | Use `/dev/random` (breaks on low entropy) [DEFAULT: `/dev/urandom`] |
| `-b` | Buffer size [DEFAULT: 8 192] |
| `-h` | Show help |
| `-V` | Print version |

## Security Modes
```
Default: /dev/urandom → Fast, production-ready, never breaks
-R flag: /dev/random  → Strict, breaks on low entropy, paranoid mode
```

## Security Guarantees
- **True kernel entropy** (hardware RNG + interrupts)
- **Production proven** (SSH, GPG, OpenSSL)
- **Break protection** for `/dev/random`

## License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)<br>
See [LICENSE](LICENSE) file for details.
