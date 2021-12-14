pdflatex p_feasible.tex
pdfcrop p_feasible.pdf

pdflatex model_sim.tex
pdfcrop model_sim.pdf

rm *.log
rm *.aux
rm *.fdb_latexmk
rm *.fls
rm *.synctex.gz