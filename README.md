# Mojiharau (文字祓う)

Mokibake'd zipfiles fixer.

The name is a pun around the Japanese term [文字化け (mojibake)][1], which
roughly translates to "Transformed characters". Since the "Transform (化け)"
part is also used for fox/tanuki spirits I thought I'd try to be all smart and
use "Cleanse/Purify/Exorcise (祓う)" to indicate that the software tries to undo
the transformation.

What a weeb, really.

## How does this work?

Mojibake'd zipfiles are that way due to the fact that the compression utility
used to create them used [JIS encoding][2] but didn't explicitly mark it in the
zip file. To fix the issue we have to explicitly mark the files as JIS encoded.

## Thanks

This project was inspired by the extremely useful tool ["Mojibake-fixer" by
ianharmon][3]. You don't even know how much I suffered to fix those damn files
before finding this tool.

[1]: https://en.wikipedia.org/wiki/Mojibake
[2]: https://en.wikipedia.org/wiki/Shift_JIS
[3]: https://github.com/ianharmon/mojibake-fixer
