# Experience 1 

Indexing 5MB ecoli assembled genome with kmers and fauremers.  
Varying
  + **k** (kmer & fauremer)
  + **order** (fauremer only)
  + **compression ratio** (fauremer only)

Then querying **10,000** sequences vs these indexes. Sequences are **1,000bp** long and come from original ecoli data with a mutation rate of **0.005** (1error/200bp).
To do so : kmers or fauremers are collected from the sequences and searched inside the corresponding index. The ratio of found elements is reported.

## Results

### k=10
![boxplot_k10](figs/boxplot_k10.png)
![histogram_k10](figs/histogram_k10.png)

### k=20

![boxplot_k20](figs/boxplot_k20.png)
![histogram_k20](figs/histogram_k20.png)

### k=30

![boxplot_k30](figs/boxplot_k30.png)
![histogram_k30](figs/histogram_k30.png)

### k=40

![boxplot_k40](figs/boxplot_k40.png)
![histogram_k40](figs/histogram_k40.png)

### k=50

![boxplot_k50](figs/boxplot_k50.png)
![histogram_k50](figs/histogram_k50.png)
