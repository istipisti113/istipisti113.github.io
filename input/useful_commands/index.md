# Linux commands that I found useful

I found these commands useful. Will keep updating it if I find something worthy. 
Please add yours if You have some (email on front page)! 
- grep -r "target" /path/to/dir [example](https://linuxvox.com/blog/find-word-in-file-linux/) [docs](https://www.man7.org/linux/man-pages/man1/grep.1.html)
  - will search for the word in all the files in the directory
- tree [docs](https://linux.die.net/man/1/tree)
  - will print out the files and directories in a path in a tree graph
- pv [manpage](https://www.man7.org/linux/man-pages/man1/pv.1.html)
  - will print a progress bar of operations
  - pv -l -L 50000 /path/to/txt/file | ssh hostname 'cat - > target.txt'
    - will print a progress bar of how a file is sent over ssh
