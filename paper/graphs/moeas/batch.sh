pdflatex comparison.tex
pdflatex ibea.tex
pdflatex moead.tex
pdflatex nsgaii.tex

pdfcrop comparison.pdf
pdfcrop ibea.pdf
pdfcrop nsgaii.pdf
pdfcrop moead.pdf

rm *.log
rm *.aux
rm *.fdb_latexmk
rm *.fls
rm *.synctex.gz