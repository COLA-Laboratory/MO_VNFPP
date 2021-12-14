pdflatex generic.tex
pdflatex tailored.tex
pdflatex uniform.tex
pdflatex initialisation.tex

pdfcrop generic.pdf
pdfcrop tailored.pdf
pdfcrop uniform.pdf
pdfcrop initialisation.pdf

rm *.log
rm *.aux
rm *.fdb_latexmk
rm *.fls
rm *.synctex.gz