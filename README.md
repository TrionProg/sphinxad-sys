# sphinxad-sys
Sphinxad low-level wrapper, that allows to read data from microphone

Dependencies
------------

In order to use the this crate, you must have the `libpocketsphinx`, `libsphinxad`, `libsphinxbase` libraries installed, you should use you packet manager or follow this [tutorial](http://cmusphinx.sourceforge.net/wiki/tutorialpocketsphinx)

Usage
-----

Look at file "read microphone and write to wav file.rs" from examples directory, it contains everything you need.

Documentation
-------------

* [sphinxad-sys documentation](https://docs.rs/sphinxad-sys/0.1.1/sphinxad_sys/)

Since `sphinxad-sys` does nothing more than export symbols from the native `libsphinxad` library, the best source for help is the information already available for the *cmusphinx* project:

* [sphinxad official documentation](http://cmusphinx.sourceforge.net/doc/sphinxbase/ad_8h.html)

Advice
------

You can use this library with horrible C-like style with [pocketsphinx](https://github.com/kriomant/pocketsphinx-sys) by this [examples](https://github.com/cmusphinx/pocketsphinx/blob/master/src/programs/continuous.c#L233), or by hight-level library with [pocketsphinx-rs](https://github.com/kriomant/pocketsphinx-rs)

License
-------

MIT
