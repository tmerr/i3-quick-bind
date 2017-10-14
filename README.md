# i3-quick-bind [![Crate](https://img.shields.io/crates/v/i3-quick-bind.svg)](https://crates.io/crates/i3-quick-bind)

Temporarily bind a shell command to a hotkey

Usage:
- Edit your i3 config to bind any keys you want to use to no-ops. For example for `alt + a` you would add the line `bindsym Mod1+a nop`.
- Launch i3-quick-bind from the terminal, like `i3-quick-bind Mod1+a 'ls -la | grep *.png'`. This will start the i3-quick-bind process which waits for messages from i3. Whenever the key combo is pressed it should spawn an `sh` child process that in this case interprets `ls -la | grep *.png`, which writes the output in the same terminal that `i3-quick-bind` was invoked from.
