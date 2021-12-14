pdflatex bfdsu.tex
pdflatex esp_vdce.tex
pdflatex qm.tex
pdflatex std.tex
pdflatex stringer.tex
pdflatex comparison.tex
pdflatex alg_fixed.tex

pdfcrop bfdsu.pdf
pdfcrop esp_vdce.pdf
pdfcrop qm.pdf
pdfcrop std.pdf
pdfcrop stringer.pdf
pdfcrop comparison.pdf
pdfcrop alg_fixed.pdf

rm *.log
rm *.aux
rm *.fdb_latexmk
rm *.fls
rm *.synctex.gz