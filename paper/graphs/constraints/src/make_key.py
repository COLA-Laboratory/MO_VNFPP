import numpy as np
import matplotlib
import matplotlib.font_manager as fm
import matplotlib.pyplot as plt
import heatmap as hm
import pandas

import sys

matplotlib.rcParams.update({
    "pgf.texsystem": "pdflatex",
    'font.family': 'serif',
    "font.serif": ["Helvetica"],
    'text.usetex': True,
    'pgf.rcfonts': False,
})

src_folder = '/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal/processed/limited_licenses'
# src_folder = 'D:/Research/NFV_PlacementModel_Journal/processed/limited_licenses'

out_folder = '/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal/paper/graphs/constraints/'
# out_folder = 'D:/Research/NFV_PlacementModel_Journal/paper/graphs/constraints/'

data = pandas.read_csv(
    src_folder + '/' + sys.argv[1] + '/' + sys.argv[2] + '/heatmap.csv', header=None)
print(data)

num_licenses = ["80%", "20%", "5%"]
percent_affected = ["10", "20", "30", "40",
                    "50", "60", "70", "80", "90", "100", ]

fig, ax = plt.subplots(figsize=(3.6,3.6))

ax.set_xlabel('VNFs affected')
ax.xaxis.set_label_position('top')

ax.set_ylabel('Licenses available')

im = hm.heatmap(data, percent_affected, num_licenses,
                ax=ax, cmap="RdYlGn", vmin=0, vmax=1, show_color_bar=True)

texts = hm.annotate_heatmap(im, valfmt="{x:.2f}")

fig.tight_layout()
fig.savefig(out_folder + 'key.pdf',
            bbox_inches='tight', orientation='portrait', transparent=True)
