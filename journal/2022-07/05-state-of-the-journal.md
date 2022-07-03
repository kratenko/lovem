---
entry: 5
published: 2022-07-01
---
# State of the Journal

__Since I am always focused on my work on lovem, I will never get sidetracked. Unrelated: I spent 
a few days on reworking the journal on this site.__

So, no update on the core project today, sorry. I was very unhappy with my first solution, on how 
the Journal entries where created. Way too much to do by hand &ndash; that is *not* what I learned 
programming for. But *mkdocs* is python, and python I can do. So did. And now I can write my 
Journal entries (like this one) as plain Markdown files with very few metadata entries. And I 
get entries in the navigation and pages listing the whole month. I even included a *whole month 
in single page* version of the journal. I feel it is quite fancy. I will need to do a bit of work 
on the static content of the site, but one step at a time.

## What I want
I want to write my Journal entries (aka blog posts) as a nice standalone markdown file, one file 
per entry. I will need to add include a bit of metadata, at least the release date/time. And 
I want the entries to look fancy without adding the fanciness to each file. Maybe I will be changing 
the layout later, hmm? And create those teaser pages for me, thank you very much.

And I have all that, now! Just look at the [source that is used to generate this entry][source].


## How it works
I use a plugin called [`mkdocs-gen-files`][gen-files], by @oprypin, that creates additional 
mkdocs source files on the fly. It does not really put the files on disk, but they are 
parsed by mkdocs, as if they were in the `docs` directory. 

I have a directory `journal` next to my `docs` directory, where I put all my posts in 
a single markdown file each. My script walks through that directory, and processes each file. 
The content is modified a bit (to put in the card with the author's name and other metadata), and 
then put in a virtual file inside `docs`, so that the pages with the entries are created by 
mkdocs, as if I hat them inside `docs`.

The script also generates two pages for each month: one that shows that month's posts as teasers, with 
a "continue reading" link, and a second one that shows all posts from a month on a single page, so 
that you can read them without changing pages all the time.

The remaining part is adding all the pages, that the script creates, to the navigation in a way that 
makes sense. The order is a critical part, being a central aspect of a journal or a log.
For that I use another plugin by @oprypin: [`mkdocs-literate-nav`][literate-nav]. With it, you can 
control your navigation (completely or in parts) by adding markdown source files with lists of links. 
This goes together well with the gen-files plugin, because I can just create that navigation files 
with it in my script.

The plugins are a bit light on the documentation side. It took me a while to understand, that you 
cannot do multiple layers of nested navigation in those files. That is not a problem, because you 
can always just add another nesting layer by adding more of those nav files as children.
Also, what you can do in those files is very limited. I wanted to do some fancy things in the navigation
(adding a second link in a single line with alternative representation). I would guess that those 
limitations come from the ways mkdocs itself handles the navigation, so that is okay. But a word on 
that would have been nice. And the error messages popping up did not help at all, because the actual 
error happens way later in the process inside mkdocs itself and is some weird side effect problem.

## The script
If you want to take a look, see [`blogem.py`][blogem]. That will be the script in its current state. 
For the version of the script at the time of writing, see the permalink, 
[the original `blogem.py`][blogem-perma].

## TODOs
* ~~Automated reload in `mkdocs serve` when I edit entry sources.~~ <br>
  _just add parameter `-w journal` to `mkdocs serve`_
* Exclude journal overview and full month pages from search.
* Exclude `NAV.md` from generating `NAV.html`.
* Maybe add tags and/or categories for posts?
* Maybe enable comments, as in material's blog.
* Add links to source in github repo.
* Add links to entry's history in github repo.
* Support multiple posts per day (by adding time to "released").

[source]: https://github.com/kratenko/lovem/blob/journey/journal/2022-07/05-state-of-the-journal.md
[gen-files]: https://oprypin.github.io/mkdocs-gen-files/
[literate-nav]: https://oprypin.github.io/mkdocs-literate-nav/
[blogem]: https://github.com/kratenko/lovem/blob/journey/blogem.py
[blogem-perma]: https://github.com/kratenko/lovem/blob/2d6f5e4c5c95064e8ba2c0dd4468777df08b8e0c/blogem.py