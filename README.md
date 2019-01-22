# grmr

`grmr` is simple command for improving writing and grammar checking.

Scripts were copied from: http://matt.might.net/articles/shell-scripts-for-passive-voice-weasel-words-duplicates/


## Installation

```
$ cargo install grmr
```

## Usage

```
$ grmr paper.tex ~/folder/*.tex            # general check of documents
$ grmr weasel paper.tex ~/folder/*.tex     # check documents for weasel words
$ grmr passive paper.tex ~/folder/*.tex    # check documents for passive voice
$ grmr ill paper.tex ~/folder/*.tex        # check documents for illusion/duplicate words
$ grmr help                                # show help
```
