Intrors-Foreignc
===========================================
.. .rst to .html: rst2html5 foo.rst > foo.html
..                pandoc -s -f rst -t html5 -o foo.html foo.rst

FFI sub-package for Rust Intro examples project.

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
        
        extern crate intrors_foreignc;
        
        use intrors_foreignc::classic;
        
        let n = 5;
        
        let res = classic::fact_i(n);

Author/Copyright
----------------
Copyright (c) 2016 by thebridge0491 <thebridge0491-codelab@yahoo.com>

License
-------
Licensed under the Apache-2.0 License. See LICENSE for details.
