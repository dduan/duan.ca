# Installing PyQt5 with Python 3 On OS X
2014-04-23T10:36:00-06:00
tag: PyQt5, Qt5, OS X, Python 3, Python, homebrew

Today I installed PyQt5 on OS X 10.9. Turns out, it's not as straight-forward
as one would think.

Using `homebrew` will __not__ work:

        brew install PyQt5 --with-python3

This is because of [an unfortunate choice][homebrew issue] made by the
homebrew maintainer regarding Python 3.

So installing from [source][installing PyQt5] is the way to go. Following the
installing instruction, you would download and install [sip][installing sip]
first, the install [PyQt5][installing PyQt5] itself.

Except that's not enough. When you run `python configure.py`, you see this
error:

        error: Use the --qmake argument to explicitly specify a working Qt
        qmake.

It's pretty self-explanatory. `qmake`, the build tool for qt is needed here.
Install qt5 with homebrew and proceed:

        brew install qt5

Afterwards, you should be able to import `PyQt5` in a Python 3 REPL.

[homebrew issue]: <https://github.com/Homebrew/homebrew/issues/25735>
[installing PyQt5]: <http://pyqt.sourceforge.net/Docs/PyQt5/installation.html>
[installing sip]: <https://web.archive.org/web/20140410074945/http://pyqt.sourceforge.net/Docs/sip4/installation.html>
