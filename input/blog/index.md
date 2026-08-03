<a href="/" style="float: right;">back</a>
# How I made this blog

## The Rust part

[source code](https://gitlab.com/istipisti113/blog/-/blob/cc42d999b2e22b2b97d45b1fe77b2cda7cbe441e/src/main.rs) <br>
The Rust code does the following:

- "scan" the input directory
  - If it finds a file it parses markdown to html and puts it in the template
    - For that I use [wooorm/markdown](https://github.com/wooorm/markdown-rs)
  - If it finds a directory it creates the directory in the output

It really could not be any simpler than that.

I plan on expanding this to automatically order the entries on the main page based on some timestamp, but I leave it as a plan for now.

## Continuous Integration

GitLab CI allows us to write scripts that will be ran on each commit (.gitlab-ci.yml). 
On committing to the main branch (depends on setup), a script will be ran, where the rust parser runs,
then a bash script copies the content of the output to the public directory.

It is in two stages, the first one is the build stage, the later is the deploy.
They are in separate stages for modularity (mainly).
