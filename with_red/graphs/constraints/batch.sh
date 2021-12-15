pdflatex anti_affinity.tex
pdfcrop anti_affinity.pdf

rm *.log
rm *.aux
rm *.fdb_latexmk
rm *.fls
rm *.synctex.gz

cd src
# python plot_limited.py ca IBEA
# python plot_limited.py ca NSGAII
python plot_limited.py ca NSGAII

# python plot_limited.py std IBEA
# python plot_limited.py std NSGAII
python plot_limited.py std NSGAII

python make_key.py std NSGAII

cd ..

pdfcrop --margins "0 -209 0 0" key.pdf

# pdfcrop --margins "0 -35 -61 -64" ca_IBEA_LIM.pdf
pdfcrop --margins "0 0 0 -1" ca_NSGAII_LIM.pdf
# pdfcrop --margins "0 -64 -61 -64" ca_NSGAII_LIM.pdf

# pdfcrop --margins "-31 -35 -61 -64" std_IBEA_LIM.pdf
pdfcrop --margins "-50 0 0 -1" std_NSGAII_LIM.pdf
# pdfcrop --margins "-31 -64 -61 -64" std_NSGAII_LIM.pdf