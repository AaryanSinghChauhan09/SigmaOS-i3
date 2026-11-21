# SigmaOS-i3

SigmaOS integration component for **i3 window manager**.

## Commands

```bash
sigmaos-i3 install   # install package + deploy defaults
sigmaos-i3 deploy    # copy config templates to $HOME
sigmaos-i3 status    # binary/package/config report
sigmaos-i3 reset     # remove deployed config
sigmaos-i3 open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
