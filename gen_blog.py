import logging
import os
import re
import pathlib
from datetime import datetime

import markdown
import mkdocs_gen_files
import yaml
from mkdocs import utils
from mkdocs.exceptions import BuildError


class Entry:
    file_path = None
    entry_path = None
    title = None
    published = None
    head_at = 0
    start_at = 0
    stop_at = 0
    dir = None
    number = None
    reading_time = None


def parse_file_header(path):
    with open(path, "r") as file:
        state = "start"
        meta = []
        headline = None
        words = 0
        bucks = 0
        head_at = 0

        for n, line in enumerate(file.readlines(), start=1):
            sline = line.strip()
            if sline == "":
                continue
            is_lim = re.match(r"^(---+|\.\.\.+)$", sline)
            if state == "start":
                if is_lim:
                    state = "meta"
                else:
                    raise BuildError(f"Meta section missing in blog entry: {path}")
            elif state == "meta":
                if is_lim:
                    state = "head"
                    head_at = n + 1
                else:
                    meta.append(line)
            elif state == "head":
                words += len(sline.split())
                if sline.startswith("#"):
                    if sline.startswith("##"):
                        raise BuildError(f"First heading must be level 1 in blog entry: {path}")
                    else:
                        headline = sline
                        start_at = n
                        state = "doc"
            elif state == "doc":
                if bucks == 0 and sline.startswith("__"):
                    bucks = 1
                if bucks == 1 and sline.endswith("__"):
                    start_at = n
                words += len(sline.split())
    if headline is None:
        raise BuildError(f"No level one heading in blog entry: {path}")

    y = yaml.safe_load("".join(meta))
    pub = str(y['published'])

    e = Entry()
    e.file_path = path
    e.published = pub
    e.title = headline
    e.start_at = start_at
    e.head_at = head_at
    e.dir = pub[:7]
    e.entry_path = os.path.join(e.dir, os.path.basename(path))
    e.number = y.get('entry')
    mins = words // 275
    e.reading_time = f"{mins} min"
    return e

    return pub, path, headline, start_at

    raise BuildError("I want to fail")
    exit(1)
    return
    data = pathlib.Path(path).read_text(encoding='utf-8')

    md = markdown.Markdown(extensions=['meta'])
    md.convert(data)
    meta = md.Meta
    print(data)
    p = meta['published'][0]
    exit()

#    print(md.Meta)
    #print(dir(md))
    #print(md.toc_tokens)
    if len(md.toc_tokens):
        h1 = md.toc_tokens[0]
        if h1['level'] == 1:
            slug = h1['id']
            name = h1['name']
            # print(f"id: '{h1['id']}', '{h1['name']}")
    pdate = md.Meta.get('published')
#    print(pdate)
    if pdate:
        d = datetime.strptime(pdate[0], "%Y-%m-%d")
#        print(d)
        return pdate[0], path

#    with open(path, "r") as f:
#        for line in f.readlines():



nav = mkdocs_gen_files.Nav()

entries = {}

months = {}

for root, dirs, files in os.walk("blog"):
    for fn in files:
        file_path = os.path.join(root, fn)
        g = parse_file_header(file_path)
        print(g)
        entries[g.published] = g
        continue
        nav[("blog", fn)] = file_path

print(sorted(entries.items()))
for d, e in sorted(entries.items(), reverse=True):
    # print(d, e)
    with open(e.file_path, "r") as f_in:
        print(f"{e.file_path} -> {e.entry_path}")
        teaser = []
        with mkdocs_gen_files.open(e.entry_path, "w") as f:
            in_teaser = False
            first_block = False
            for n, line in enumerate(f_in.readlines(), start=1):
                sline = line.strip()
#                print(f"SLINE: in_t={in_teaser}, fb={first_block} | {sline}")
                if n == e.head_at:
                    in_teaser = True
                if in_teaser:
                    tline = line.lstrip()
                    if tline.startswith("#"):
                        tline = "#" + tline
                    teaser.append(tline)
                    if n > e.start_at:
                        if first_block:
                            if sline == "":
                                first_block = False
                                in_teaser = False
                        else:
                            if not sline.startswith('#') and not sline == "":
                                # start first block
                                first_block = True
                print(line, file=f, end="")
                if n == e.start_at:
                    bibs = []
                    if e.published is not None:
                        bibs.append(f":octicons-calendar-24: {e.published}")
                    if e.number is not None:
                        bibs.append(f":octicons-book-24: Entry \\#{e.number}")
                    if e.reading_time is not None:
                        bibs.append(f":octicons-clock-24: {e.reading_time} read")
                    bibs = " · ".join(bibs)
                    addition = f"""

<aside class="mdx-author" markdown>
![@kratenko](https://avatars.githubusercontent.com/kratenko)

<span>__kratenko__ · :octicons-mark-github-16: [kratenko](https://github.com/kratenko)</span>
<span> {bibs}
</span>
</aside>

---

"""
                    print(addition, file=f)
                    teaser.append(addition)
            e.teaser = "".join(teaser)
            month = e.published[:7]
            if month not in months:
                months[month] = []
            months[month].append(e)
            print("\nSTART-BLOCK: {}\nEND-BLOCK\n".format("".join(teaser)))
            print(e.start_at, e.head_at)

print(months)
for m, es in months.items():
    with mkdocs_gen_files.open(f"{m}.md", "w") as f:
        for e in es:
            print(e.teaser, file=f)
            print(f"[:octicons-arrow-right-24: Continue reading]({e.entry_path})", file=f)