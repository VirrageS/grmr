# grmr

[![Crates.io](https://img.shields.io/crates/v/grmr.svg)](https://crates.io/crates/grmr)
[![Build Status](https://travis-ci.org/VirrageS/grmr.svg?branch=master)](https://travis-ci.org/VirrageS/grmr)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/VirrageS/grmr/blob/master/LICENSE)

`grmr` is simple command for improving writing and grammar checking.

Scripts were inspired by: http://matt.might.net/articles/shell-scripts-for-passive-voice-weasel-words-duplicates/


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
$ grmr -h                                  # show help
```
