# [LibRaw][libraw]

Rust Bindings and Safe(er) api over the C Api of [libraw][libraw] + some extra stuff.

You might need to use  
`export MACOSX_DEPLOYMENT_TARGET=10.8`  
for older versions of macos.  

Raw files for testing were sourced from https://rawsamples.ch/index.php

Setup and install [vcpkg](https://vcpkg.io/en/getting-started). Make sure you have set `VCPKG_ROOT`.  
Rawspeed requires "export MACOSX_DEPLOYMENT_TARGET=10.13" on MacOS.

[libraw]: https://libraw.org
