pdflatex aa_expansion.tex
pdflatex add_remove.tex
pdflatex fat_tree.tex
pdflatex limited_expansion.tex
pdflatex mapping_flowchart.tex
pdflatex original_loop.tex
pdflatex simple_expansion.tex
pdflatex solution_representation.tex
pdflatex swap.tex
pdflatex unfolded_loop.tex

pdfcrop aa_expansion.pdf
pdfcrop add_remove.pdf
pdfcrop fat_tree.pdf
pdfcrop limited_expansion.pdf
pdfcrop mapping_flowchart.pdf
pdfcrop original_loop.pdf
pdfcrop simple_expansion.pdf
pdfcrop solution_representation.pdf
pdfcrop swap.pdf
pdfcrop unfolded_loop.pdf

rm *.log
rm *.aux
rm *.fdb_latexmk
rm *.fls
rm *.synctex.gz