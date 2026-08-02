# How I made this blog

## The Rust part
The Rust code does the following:

- "scan" the input directory
  - if it finds a file it parses markdown to html and puts it in the template
  - if it finds a directory it creates the directory in the output

It really could not be any simpler than that.

## Continuous Integration

GitLab CI allows us to write scripts that will be ran on each commit.
On committing to the main branch, a script will be ran, where the rust parser runs,
then a bash script copies the content of the output to the public directory. It could output could be
configured to be the _public_ one, but whatever.
