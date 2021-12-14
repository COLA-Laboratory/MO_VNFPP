pdflatex length_used.tex
pdflatex mm1.tex
pdflatex mm1k.tex
pdflatex proposed.tex
pdflatex resources_energy.tex
pdflatex constant_energy.tex
pdflatex models.tex

pdfcrop length_used.pdf
pdfcrop mm1.pdf
pdfcrop mm1k.pdf
pdfcrop proposed.pdf
pdfcrop resources_energy.pdf
pdfcrop constant_energy.pdf
pdfcrop models.pdf

rm *.log
rm *.aux
rm *.fdb_latexmk
rm *.fls
rm *.synctex.gz