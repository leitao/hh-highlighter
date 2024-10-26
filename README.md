# hh-highlighter
Fast text highlighter

This application reads from stdin, and highlight selected words printed to
stdout.

Example:
```
# cat <foo> | hh-highlight <word1> <word2>
```

There are a few other options, such as sleeping after a word is found, for
better consumption.

This is useful when you are waiting for some word on a stream of text

## Help
```
hh: The text highlighter. Read from stdin, and highlight selected words printed to stdout

Usage: hh-highlight [OPTIONS] [WORDS]...

Arguments:
  [WORDS]...  The words you want to highlight

Options:
  -c, --context...  Add more context to the words being highlighted
  -w, --wait...     Wait a second after the line with the highlighted word is printed
  -h, --help        Print help
  -V, --version     Print version
```
