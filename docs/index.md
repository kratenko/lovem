# The Journey towards Lovem
This is a journal for my journey towards building 
a **the low overhead virtual embedded machine**.

What that is and why I started building it will come clear from the entries in this journal.
For some reason I feel the need to add the history of lovem's creation (so far) to it. 
So I wrote down the history of it before I recently came back to it in three articles, 
which I then shamelessly split into multiple shorter posts, so I have more to publish.
This is new to me, let's see what will happen!

My work on lovem so far has been focused on self education. Why not share my insights?
I hope it will be useful to someone &ndash; maybe even for my future self?

<center>&mdash; [The journey starts here](2022-06/ALL.md) &mdash;</center>

## Lovem
Lovem is meant to become a virtual machine for use in constrained embedded devices. 
It was started by me, @kratenko, for reasons I am writing about in this journal

You can find lovem on github: <br>
https://github.com/kratenko/lovem

## Me
If for some reason you want to contact me, you can find me on 
twitter [@garstenko][garstenko] or of course on github as :octicons-mark-github-16: @kratenko.

[garstenko]: https://twitter.com/garstenko

## This site
So I quickly googled `mkdocs blog` to find some sensible way to document my journey online. 
https://squidfunk.github.io/mkdocs-material/blog/ was not quite what I was looking for. 
It is the blog of @squidfunk the guy who writes [Material for MkDocs][material], where they 
blog about their work on Material for MkDocs. They (unsurprisingly) do this with the help of 
Material for MkDocs. It looks like the blog entries are generated by some preprocessing 
script, which I might be trying to also build at some point, because I like the previews 
on the "Blog" page. 

So, big shout out to @squidfunk for their work!

I managed to hack together a little script I called `blogem`, that solves blogging for me. 
It is not ready to be used for other projects, but if you are curious, you are welcome to 
take a look at it. It uses [`mkdocs-gen-files`][gen-files] and [`mkdocs-literate-nav`][literate-nav]
by @oprypin. Or maybe it abuses those plugins, I'm not sure, but it works for now. 
See Journal entry [*State of the journal*][journal-post] for more that part of the story.
Pretty sure I will change every thing again at some point. But currently, I quite like it.

[material]: https://squidfunk.github.io/mkdocs-material/
[gen-files]: https://oprypin.github.io/mkdocs-gen-files/
[literate-nav]: https://oprypin.github.io/mkdocs-literate-nav/
[journal-post]: 2022-07/state-of-the-journal.md