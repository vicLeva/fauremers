import sys

def is_valid_dna(seq):
    return all(base in b"ATCG" for base in seq)

def extract_queries(input_file, output_file, query_len, step):
    count = 0
    with open(input_file, 'rb') as f_in, open(output_file, 'w') as f_out:
        seq = bytearray()
        header = None

        for line in f_in:
            if line.startswith(b'>'):
                if seq:
                    count += process_sequence(seq, query_len, step, f_out)
                    seq = bytearray()
                header = line.strip()
            else:
                seq.extend(line.strip())

        if seq:
            count += process_sequence(seq, query_len, step, f_out)

    print(f"{count} queries written.")

def process_sequence(seq, k, step, f_out):
    written = 0
    for i in range(0, len(seq) - k + 1, step):
        sub = seq[i:i + k]
        if is_valid_dna(sub):
            f_out.write(f">query_{written}_{i}\n{mutate(sub, 0.005)}\n") #1/200 mutation rate
            written += 1
    return written


def mutate(seq, mutation_rate):
    import random

    random.seed(42)  # For reproducibility

    bases = b"ATCG"
    mutated_seq = bytearray(seq)
    for i in range(len(mutated_seq)):
        if random.random() < mutation_rate:
            mutated_seq[i] = random.choice(bases)
    return mutated_seq.decode()

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: python extract_queries.py <input.fasta> <step> <query_length>")
        sys.exit(1)

    input_file = sys.argv[1]
    step = int(sys.argv[2])
    query_len = int(sys.argv[3])
    output_file = "queries.fasta"

    extract_queries(input_file, output_file, query_len, step)
