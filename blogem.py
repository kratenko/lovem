import os
import re

import mkdocs_gen_files
import yaml
from mkdocs.exceptions import BuildError
from slugify import slugify


def month_name(n):
    names = ['January', 'February', 'March', 'April', 'May', 'June',
             'July', 'August', 'September', 'October', 'November', 'December']
    return names[n - 1]


def pretty_month(m):
    y, m = m.split("-")
    return month_name(int(m)) + " " + y


class Entry:
    """
    Prepared blog entry, holding meta data (but not entry's body).
    """
    file_path = None
    slug = None
    first_body_line = 0
    heading_line = 0
    insert_card_line = 0
    end_of_teaser_line = 0
    number = None
    reading_time = None
    published = None
    group = None
    entry_path = None
    entry_path_in_group = None
    title = None

    WORDS_READ_PER_MINUTE = 275
    RE_META_FENCE = re.compile(r"^(---+|\.\.\.+)$")

    def from_file(self, path):
        self.file_path = path
        # States are:
        # start, meta, head
        state = "start"
        meta = []
        words = 0
        bucks = 0
        in_first_paragraph = False
        end_of_teaser = 0
        with open(path, "r") as file:
            for n, line in enumerate(file.readlines(), start=1):
                sline = line.strip()
                if state == "start":
                    if sline == "":
                        continue
                    elif self.RE_META_FENCE.match(sline):
                        state = "meta"
                    else:
                        raise BuildError(f"Meta section missing in blog entry: {path}")
                elif state == "meta":
                    if self.RE_META_FENCE.match(sline):
                        state = "head"
                        self.first_body_line = n + 1
                    else:
                        meta.append(line)
                elif state == "head":
                    words += len(sline.split())
                    if sline.startswith("#"):
                        if sline.startswith("##"):
                            raise BuildError(f"First heading must be level 1 in blog entry: {path}")
                        else:
                            self.title = sline[1:]
                            self.heading_line = n
                            self.insert_card_line = n + 1
                            self.slug = slugify(sline[1:])
                            state = "doc"
                elif state == "doc":
                    if bucks == 0 and sline.startswith("__"):
                        bucks = 1
                    if bucks == 1 and sline.endswith("__"):
                        self.insert_card_line = n + 1
                        bucks = 2
                        end_of_teaser = 0
                        in_first_paragraph = False
                    elif end_of_teaser == 0:
                        if in_first_paragraph and sline == "":
                            end_of_teaser = n
                        if not sline == "" and not sline.startswith("#"):
                            in_first_paragraph = True
                    words += len(sline.split())
        self.end_of_teaser_line = end_of_teaser

        y = yaml.safe_load("".join(meta))
        self.number = y.get("entry")
        mins = words // self.WORDS_READ_PER_MINUTE
        if mins == 0:
            self.reading_time = f"&lt; 1 min"
        else:
            self.reading_time = f"{mins} min"
        self.published = str(y.get("published"))
        self.group = self.published[:7]
        if "slug" in y:
            self.slug = y["slug"]
        self.entry_path_in_group = self.slug + ".md"
        self.entry_path = os.path.join(self.group, self.entry_path_in_group)

    def card(self):
        """Return source for the "author/meta"-card for blog posts."""
        bibs = []
        if self.published is not None:
            bibs.append(f":octicons-calendar-24: {self.published}")
        if self.number is not None:
            bibs.append(f":octicons-book-24: Entry \\#{self.number}")
        if self.reading_time is not None:
            bibs.append(f":octicons-clock-24: {self.reading_time} read")
        # The infos in lower line; date, reading time, ...
        bibs = " · ".join(bibs)
        card = f"""

<aside class="mdx-author" markdown>
![@kratenko](https://avatars.githubusercontent.com/kratenko)

<span>__kratenko__ · :octicons-mark-github-16: [kratenko](https://github.com/kratenko)</span>
<span> {bibs}
</span>
</aside>

---

"""
        return card

    def print_teaser(self, *, file):
        with open(self.file_path, "r") as src_f:
            for n, line in enumerate(src_f.readlines(), start=1):
                if not line.startswith("    ") and line.lstrip().startswith("#"):
                    line = "#" + line.lstrip()
                if n < self.first_body_line:
                    continue
                if n >= self.end_of_teaser_line:
                    break
                print(line, end="", file=file)
                if n == self.insert_card_line:
                    print(self.card(), file=file)
            print(f"\n[:octicons-arrow-right-24: Continue reading]({self.entry_path_in_group})", file=file)

    def print_entry(self, *, file, is_sub_page=False):
        with open(self.file_path, "r") as src_f:
            for n, line in enumerate(src_f.readlines(), start=1):
                # skip meta
                if n < self.first_body_line:
                    continue
                if is_sub_page:
                    # Change headings to be on one lower level:
                    if not line.startswith("    ") and line.lstrip().startswith("#"):
                        line = "#" + line.lstrip()
                # pass-on line:
                print(line, end="", file=file)
                # insert autor/meta card:
                if n == self.insert_card_line:
                    print(self.card(), file=file)


def load_entries(path):
    """
    Read each blog entry source file, to extract their meta-data. Check plausibility of entries.
    This will cancel the build process on multiple kinds of errors/inconsistencies inside blog.
    """
    entries = {}
    for root, _, files in os.walk(path):
        for fn in files:
            file_path = os.path.join(root, fn)
            e = Entry()
            e.from_file(file_path)
            if e.published in entries:
                raise BuildError(f"Duplicate published date/time '{e.published}': {file_path}")
            entries[e.published] = e
    groups = {}
    for pub, e in sorted(entries.items()):
        # put into groups:
        if e.group not in groups:
            groups[e.group] = []
        groups[e.group].append(e)
    return groups


def create_entry_files(entries):
    groups = {}
    for pub, e in sorted(entries.items(), reverse=True):
        if e.group not in groups:
            groups[e.group] = []
        groups[e.group].append(e)
        with open(e.file_path, "r") as src_f:
            with mkdocs_gen_files.open(e.entry_path, "w") as dst_f:
                for n, line in enumerate(src_f.readlines(), start=1):
                    print(line, end="", file=dst_f)
                    if n == e.insert_card_line:
                        print(e.card(), file=dst_f)
    return groups


def blogem():
    """
    Generate mkdocs sources for blog from blog sources.

    Called by `mkdocs-gen-files plugin`. This will virtually generate additional markdown files inside `docs`
    directory, will add blog posts to the site. You will not see the generated files, only what is rendered
    of them in the `site` directory.

    This will generate the blog entries themselves, the monthly pages (one with teasers and "continue" links,
    one with all posts in single page), and multiple `NAV.md` files, that define the nav-entries for
    the blog.
    """
    # All prepared entries, by published date/time (date/time must be unique for whole blog).
    groups = load_entries("blog")

    # build nav for months in descending order:
    for group, es in sorted(groups.items(), reverse=True):
        # Build the nav entries for the month:
        with mkdocs_gen_files.open(os.path.join(group, "NAV.md"), "w") as dst_f:
            for e in reversed(es):
                print(f"- [{e.title}]({e.entry_path_in_group})", file=dst_f)
                with mkdocs_gen_files.open(e.entry_path, "w") as entry_f:
                    e.print_entry(file=entry_f)
        # Build month's overview page, with teasers and "continue reading" links:
        with mkdocs_gen_files.open(os.path.join(group, "index.md"), "w") as dst_f:
            print("# Journal entries from " + pretty_month(group), file=dst_f)
            print("[Read all in single page](ALL.md)", file=dst_f)
            for e in reversed(es):
                e.print_teaser(file=dst_f)

    # Build top level nav for blog (months in descending order).
    # Must live in a directory, that does not need to do anything als.
    with mkdocs_gen_files.open("journal/NAV.md", "w") as nav_f:
        for group, es in sorted(groups.items(), reverse=True):
            print(f"- [{pretty_month(group)}](../{group}/)", file=nav_f)
            with mkdocs_gen_files.open(group + "/ALL.md", "w") as dst_f:
                print(f"# Complete month of {pretty_month(group)}", file=dst_f)
                for e in es:
                    e.print_entry(file=dst_f, is_sub_page=True)

    # Build nav for months' ALL pages (in ascending order):
    with mkdocs_gen_files.open("months/NAV.md", "w") as months_f:
        for group, es in sorted(groups.items()):
            print(f"- [{pretty_month(group)} complete](../{group}/ALL.md)", file=months_f)

    with mkdocs_gen_files.open("journal/index.md", "w") as f:
        nav = """
# Journal

## Latest entry

## Complete month in single page

If you want to read the whole story, this might be easier to follow.

"""
        print(nav, file=f)
        for group, es in sorted(groups.items()):
            print(f"- [{pretty_month(group)} complete](../{group}/ALL.md)", file=f)


blogem()
