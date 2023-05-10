# prompl

This is a small binary I use to generate my shell prompt. Why do I need a
script for this you ask? Because my prompt changes colour and character
randomly every time it is refreshed, otherwise I end up getting bored of it and
changing it manually every few weeks. Also its got some homemade powerline-esk
stuff going on with the path display and git branch.

This isn't really intended to be used as-is by others, its public so I can
clone it without having to login and because it might serve as a
reference/inspiration for others.

Oh yeah and this is designed soley for zsh, for use like:

```zsh
export PROMPT='$(prompl)'
```

The `''` are important because otherwise it will only run `prompl` once per zsh
session rather than per prompt refresh.
