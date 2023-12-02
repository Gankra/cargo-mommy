# xonsh

if you want to have mommy help you with cargo, git and rustup
you can put the following in ~/.xonshrc

```py
@events.on_pre_spec_run_cargo
@events.on_pre_spec_run_git
@events.on_pre_spec_run_rustup
def cargo_mommy(spec=None, **kwargs):
    spec.env = dict(spec.env or {})
    spec.env["CARGO_MOMMYS_ACTUAL"] = spec.cmd[0]
    spec.cmd = ["cargo-mommy", *spec.cmd[1:]]
```
