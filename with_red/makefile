FILE=main

all:	
	latex $(FILE)
	bibtex $(FILE)
	latex $(FILE)
	latex $(FILE)
	dvips -o $(FILE).ps $(FILE).dvi
	ps2pdf $(FILE).ps
	open $(FILE).pdf

clean:
	rm -f *.aux *.blg *.log *.bbl *.out *.bak *.synctex.gz *.synctex.gz\(busy\) *.tex~

cleanall: clean
	rm -f *.dvi $(FILE).ps $(FILE).pdf
