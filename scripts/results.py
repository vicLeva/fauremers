#type;k;order;c_ratio;size;values

import matplotlib.pyplot as plt
import numpy as np


def trash_to_csv():
    def read_line(line):
        parts = line.strip().split(";")
        
        config = parts[0]
        size = int(parts[1])
        values = list(map(float, parts[2].split(",")[:-1]))
        return config, size, values
    

    output = "results.csv"
    with open(output, "w") as out_fh:
        for k in range(10, 60, 10):
            with open(f"results_{k}.txt", "r") as fh:
                #1st line, kmers
                k, size, values = read_line(fh.readline())
                print(f"K: {k}, Size: {size}, Values: {len(values)}")
                out_fh.write(f"kmer;{k};;;{size};{','.join(map(str, values))}\n")

                #other lines, fauremers
                for line in fh:
                    config, size, values = read_line(line)
                    print(f"Config: {config}, Size: {size}, Values: {len(values)}")
                    out_fh.write(f"fauremer;{config.split(',')[2]};{config.split(',')[0]};{config.split(',')[1]};{size};{','.join(map(str, values))}\n")


def read_line(line):
    #type;k;order;c_ratio;size;values
    parts = line.strip().split(";")

    if parts[0] == "kmer":
        return parts[0], (int(parts[1]), 
                          int(parts[4]), 
                          list(map(float, parts[5].split(",")))) #[:-1] removed for now
    else:
        return parts[0], (int(parts[1]), 
                          int(parts[2]), 
                          float(parts[3]), 
                          int(parts[4]), list(map(float, parts[5].split(",")))) #[:-1] removed for now
    
def read_data(filename, kmers, fauremers):
    with open(filename, "r") as fh:
        for line in fh:
            type_, data = read_line(line)
            if type_ == "kmer":
                kmers.append(data)
            else:
                fauremers.append(data)


import matplotlib.pyplot as plt


def box_plot(samples, k):
    orders = list(range(5, 55, 5))           # [5, 10, ..., 50]
    c_ratios = [round(0.05 * i, 2) for i in range(1, 11)]  # [0.05, 0.1, ..., 0.5]

    # Generate label list with "k-mer" first, then (order, c_ratio) pairs
    labels = ["k-mer"] + [f"({o},{c})" for o in orders for c in c_ratios][1:]

    plt.figure(figsize=(30, 6))
    box = plt.boxplot(samples, showfliers=False, patch_artist=True)

    # Highlight first box
    box['boxes'][0].set(facecolor='lightcoral')
    box['medians'][0].set(color='black')

    plt.xticks(ticks=np.arange(1, len(labels) + 1), labels=labels, rotation=90)
    plt.title(f'Boxplots for {len(samples)} samples (k={k})')
    plt.xlabel('Sample (order, c_ratio)')
    plt.ylabel('ratio of found kmers or fauremers')
    plt.tight_layout()
    
    plt.savefig(f"figs/boxplot_k{k}.png", dpi=500)
    #plt.show()


def histogram_plot(samples, k):
    # Compute one value per sample (e.g. mean)
    values = [np.mean(sample) for sample in samples]

    orders = list(range(5, 55, 5))           # [5, 10, ..., 50]
    c_ratios = [round(0.05 * i, 2) for i in range(1, 11)]  # [0.05, 0.1, ..., 0.5]

    labels = ["k-mer"] + [f"({o},{c})" for o in orders for c in c_ratios][1:]

    plt.figure(figsize=(30, 6))
    bars = plt.bar(range(len(values)), values, color='skyblue')

    # Highlight first bar
    bars[0].set_color('lightcoral')

    plt.xticks(ticks=np.arange(len(labels)), labels=labels, rotation=90)
    plt.title(f'index sizes per sample (k={k})')
    plt.xlabel('Sample (order, c_ratio)')
    plt.ylabel('index size')
    plt.tight_layout()
    
    plt.savefig(f"figs/histogram_k{k}.png", dpi=500)
    #plt.show()




if __name__ == "__main__":
    #type;k;order;c_ratio;size;values

    kmers       = list()  #[k, size, values]
    fauremers   = list()  #[k, order, c_ratio, size, values]
    read_data("results.csv", kmers, fauremers)
    print(f"Read {len(kmers)} kmers and {len(fauremers)} fauremers.")

    for exp in range(5):
        print(f"Experiment {exp+1}")
        
        box_plot(
            [kmers[exp][2]] + [x[4] for x in fauremers[exp*100:(exp+1)*100]], #kmer and fauremers for k
            (exp+1)*10
        )  

        histogram_plot(
            [kmers[exp][1]] + [x[3] for x in fauremers[exp*100:(exp+1)*100]], #kmer and fauremers for k
            (exp+1)*10
        )

    


    

