Intrors-Intro
===========================================
.. .rst to .html: rst2html5 foo.rst > foo.html
..                pandoc -s -f rst -t html5 -o foo.html foo.rst

Main app sub-package for Rust Intro examples project.

Installation
------------
source code tarball download:
    
        # [aria2c --check-certificate=false | wget --no-check-certificate | curl -kOL]
        
        FETCHCMD='aria2c --check-certificate=false'
        
        $FETCHCMD https://bitbucket.org/thebridge0491/intrors/[get | archive]/master.zip

version control repository clone:
        
        git clone https://bitbucket.org/thebridge0491/intrors.git

build example with make:
cd <path> ; [sh] ./configure.sh [--prefix=$PREFIX] [--help]

make build [test]

build example with cargo:
cd <path> ; [sh] ./configure.sh [--prefix=$PREFIX] [--help]

[env PKG_CONFIG_PATH=$PREFIX/lib/pkgconfig] cargo build [test]

Usage
-----
        [env RSRC_PATH=<path>/resources] [$PREFIX/bin/]intrors_intro_main [-h]

Author/Copyright
----------------
Copyright (c) 2016 by thebridge0491 <thebridge0491-codelab@yahoo.com>

License
-------
Licensed under the Apache-2.0 License. See LICENSE for details.
