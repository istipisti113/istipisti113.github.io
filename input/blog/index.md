# How I made this blog

## The Rust part

[source code](https://github.com/istipisti113/istipisti113.github.io) <br>
The Rust code does the following:

- "scan" the input directory
  - If it finds a file it parses markdown to html and puts it in the template
    - For that I use [wooorm/markdown](https://github.com/wooorm/markdown-rs)
  - If it finds a directory it creates the directory in the output

The rust program handles the http sites as <span class="codeblock">Strings</span>, I use <span class="codeblock">.replace()</span> on them to put the parts together like Lego.
It is most certainly not the _de facto_ correct way, but it works. It really could not be any simpler than that.
I edit the markdown input files with Obsidian.

From the contents of the pages put into the html template (or "bracket" if you will) to the href in the button everything is just <span class="codeblock">.replace()</span>. I have yet to shoot myself in the foot with it, so until the day comes, I will not change it. Html is _just_ text after all, what could go wrong...

I plan on expanding this to automatically order the entries on the main page based on some timestamp, but I leave it as a plan for now.

## Continuous Deployment

The site used to be hosted on GitLab, but I had trouble hosting it without logging in.


GitHub allows us to write scripts that will be ran on each commit (.github/workflows/*). 
On committing to the main branch (depends on setup), a script will be ran, where the rust parser runs, then a bash script copies the content of the output to the public directory.

It is in two stages, the first one is the build stage, the later is the deploy.
They are in separate stages for modularity (mainly).
