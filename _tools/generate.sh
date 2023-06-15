#!/bin/bash

PROJECT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}/.." )" &> /dev/null && pwd )

marp lesson01/slide.md -o lesson01/slide.pdf
marp lesson02/slide.md -o lesson02/slide.pdf
marp lesson03/slide.md -o lesson03/slide.pdf
marp lesson04/slide.md -o lesson04/slide.pdf
marp lesson05/slide.md -o lesson05/slide.pdf
marp lesson06/slide.md -o lesson06/slide.pdf

gs -dBATCH -dNOPAUSE -q -sDEVICE=pdfwrite -sOutputFile=slide_all.pdf lesson01/slide.pdf lesson02/slide.pdf lesson03/slide.pdf lesson04/slide.pdf lesson05/slide.pdf lesson06/slide.pdf