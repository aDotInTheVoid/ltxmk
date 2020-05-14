# Notes on building the corpus

## a-level-notes
- Native: `latexmk`
- Dired: Yes

## dart
- Native: `make`
- Dired: No

### Strategy
```
pdflatex dartLangSpec.tex
makeindex dartLangSpec.idx
pdflatex dartLangSpec.tex
makeindex dartLangSpec.idx
pdflatex dartLangSpec.tex
pdflatex dartLangSpec.tex
```

## gsasl
- Native: `make`
- Dired: No

### Strategy
```bash
rm -f *.ps *.dvi *.aux *.toc *.idx *.ind *.ilg *.log *.out *.brf *.blg *.bbl refman.pdf
pdflatex refman
makeindex refman.idx
pdflatex refman
latex_count=8 ; \
    while egrep -s 'Rerun (LaTeX|to get cross-references right)' refman.log && [ $latex_count -gt 0 ] ;\
    do \
        echo "Rerunning latex...." ;\
        pdflatex refman ;\
        latex_count=`expr $latex_count - 1` ;\
    done
makeindex refman.idx
pdflatex refman
```


## Napkin
- Native: `mkdir asy && latexmk -pdf`
- Dired: No

## thesis-gankra
- Native: `./build.sh`
- Dired: No

### Strategy
```bash
pdflatex -shell-escape thesis.tex
cd chapters
rm *.bbl
rm *.blg
for f in *.aux
do
    bibtex "$f"
done
cd ..
pdflatex -shell-escape thesis.tex
pdflatex -shell-escape thesis.tex
```

## ua-thesis
- Native: `make all`
- Dired: yes

### Strategy
```bash
cd build
latexmk -pdf -view=none -auxdir=build -outdir=build \
    -pdflatex="pdflatex -file-line-error --shell-escape %O %S" \
    -e '$makeindex=qq/sh -c "cd "`dirname "%D"`" ; makeindex %O -o "`basename "%D"`" "`basename "%S"`""/;' \
    -cd ../cover.tex ../matter.tex
```




