Intrors-Util
===========================================
.. .rst to .html: rst2html5 foo.rst > foo.html
..                pandoc -s -f rst -t html5 -o foo.html foo.rst

Utilities sub-package for Rust Intro examples project.

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

cargo fetch

make build [test]

build example with cargo:
cd <path> ; [sh] ./configure.sh [--prefix=$PREFIX] [--help]

cargo fetch

[PKG_CONFIG_PATH=$PREFIX/lib/pkgconfig] cargo build [test]

Usage
-----
        // PKG_CONFIG='pkg-config --with-path=$PREFIX/lib/pkgconfig'
        
        // $PKG_CONFIG --cflags --libs <ffi-lib>
        
        extern crate intrors_util;
        
        use intrors_util::util;
        
        let (arr1, arr2) = ([0, 1, 2], [10, 20, 30]);
        
        let arr_prod: Vec<(i32, i32)> = cartesian_prod(&arr1, arr2);

Author/Copyright
----------------
Copyright (c) 2016 by thebridge0491 <thebridge0491-codelab@yahoo.com>

License
-------
Licensed under the Apache-2.0 License. See LICENSE for details.
