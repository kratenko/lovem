# Source Code
## Getting the source code
The best way to work with the source code, is cloning the complete repo to your computer. If you do 
not know how to do that, GitHub has documentation on [cloning a repositry][clone].

The way to do this in bash:

~~~shell
git clone git@github.com:kratenko/lovem.git
~~~

Or, if you have problems using git over ssh, use https:

~~~shell
git clone https://github.com/kratenko/lovem.git
~~~

This will create a directory named `lovem` inside your current directory, that holds all the 
source code and its complete history as a git repository.

## Tags
Lovem is a developing project that I write about while creating it. My journal entries (blog posts, if you prefere)
often talk about a very distinct state of the source code. I am describing, what I do, while I do it. 
It is a very likely possibility, that at the time you are reading my journal entries, the code will look nothing 
like it did, when I posted the entry. I will dump a lot of my ideas. Sometimes I write code that I know I will 
be changing, in order to illustrate my thoughts and, ultimately, to let you participate in my journey. 

Luckily, it is easy for us, to travel back in time, using the magical powers of git! To make it easy, I will 
create a tag (and with it a pre-release) for the entries that refer to source code. They should be named 
something like `v1.2.3-journey`, and you can find them in header card of the entries (where author and 
publication date, etc. are shown). At the bottom of the pages, holding entries with a tag, there will 
be some additional links that take you directly to the source code of that tag.

The easiest way to view source code for my posts, is having the 
[repository cloned locally](#getting-the-source-code), and then 
checking out the tag. So, if you want to check out tag `v1.2.3-journey`, while inside your lovem 
directory, simply type:

~~~shell
git checkout v1.2.3-journey
~~~

And you will have the code for that journal entry ready to be inspected with you favourite 
IDE or editor. And you can fire up cargo to build the code and run the examples. You can then 
mess around with the source and try out stuff. This really helps to understand what we are doing!
And the best thing: you can mess around as much as you like. It is git! You can always switch 
back to the current state of the code by typing 

~~~shell
git checkout master
~~~

You can even commit your tinkering to your own copy of the repo - be it inside your own branches, 
or however you prefer.

I will not be linking to the source code explicitly in my entries (only in the first ones, before 
I introduced this). So be sure to use the link at the top or bottom, to find the source &ndash; or 
better yet, just check out the tag in your local repo clone. The git command will be listed in the 
bottom of the entry.

[clone]: https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository