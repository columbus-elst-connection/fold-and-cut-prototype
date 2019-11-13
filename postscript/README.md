# PostScript
[PostScript][postscript] is a document description langage. It

> translates documents into print – exactly as intended. Released in 1984 as
> Adobe’s founding technology, PostScript played a key role in the Desktop
> Publishing Revolution. It was the first device-independent Page Description
> Language (PDL), and also a programming language. Today, enterprises around the
> world rely on Adobe PostScript for accurately printing documents from any
> application.

Here we will use it to print out the results of our algorithms.

## Viewing results
One way of viewing the results is sending it to a printer. Often pdf viewers are
capable of displaying PostScript as well. Yet an other is to use
[`ghostscript`][ghostscript]: 

> An interpreter for the PostScript language and for PDF.

Ghostscript allows for greater resolution of problems, when they occur at all.

### Ghostscript
Ghostscript has some helpful hints that can be accessed by executing

```sh
ghostscript --help
```

For one, it shows a lot of device to render to. E.g. Unix machine could be
interested in the `x11` device, allowing them to view an image. Showing an image
amounts to

```sh
ghostscript -sDEVICE=x11 -dBATCH -q diagram.ps
```

[postscript]: https://www.adobe.com/products/postscript.html
[ghostscript]: https://www.ghostscript.com/
