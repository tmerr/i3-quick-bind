Warning: vaporware

# i3-quick-bind
Temporarily bind a shell command to a hotkey

Planned usage:
- Bind any hotkeys you want to use to no-ops in your i3 config, i.e. `bindsym Mod1+e nop`
- Now open a terminal and enter `i3-quick-bind e cargo build` to run cargo build whenever alt + e is pressed. The output gets dumped to the same terminal. If there are multiple modifiers with the same key symbol you can disambiguate with the same syntax as in the config: `i3-quick-bind Mod1+e cargo build`.

Requires i3 version 4.11+.
