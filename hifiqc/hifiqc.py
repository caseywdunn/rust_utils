import sys
import pandas as pd
import datetime

def n50(lengths_raw: pd.Series) -> int:
    """Calculate the N50 of a list of lengths."""
    lengths = lengths_raw.copy()
    total_length = sum(lengths)
    half_length = total_length / 2
    lengths.sort_values(ascending=False, inplace=True)
    current_length = 0
    for length in lengths:
        current_length += length
        if current_length >= half_length:
            return length

def main() -> int:
    in_file = sys.argv[1]  # first argument is the input file
    
    if len(sys.argv) > 2:
        out_dir = sys.argv[2]
    else:
        out_dir = './'
    
    # If out_dir doesn't end with a slash, add one
    if out_dir[-1] != '/':
        out_dir += '/'
    
    # Get the input file  name without the directory path
    in_file_name = in_file.split('/')[-1]
    out_file_name = out_dir + in_file_name

    print("Output files: " + out_file_name)
    
    # read the input file
    df = pd.read_csv(in_file, sep=' ', header=0, index_col=0)
    
    ax = df.plot.hexbin(x='length', y='readscore', gridsize=20)
    
    # Save the plot
    fig = ax.get_figure()
    fig.savefig(out_file_name + ".readscore_v_length.png")
    
    # Open the output log file
    with open(out_file_name + ".log", 'w') as log_file:
        log_file.write("Date and time: " + str(datetime.datetime.now()) + "\n")
        log_file.write("Input file: " + in_file + "\n")
        log_file.write("Output file: " + out_file_name + "\n")
        log_file.write("Number of reads: " + str(len(df)) + "\n")
        log_file.write("Total length: " + str(sum(df['length'])) + "\n")
        log_file.write("N50: " + str(n50(df['length'])) + "\n")
        
if __name__ == '__main__':
    sys.exit(main())  # next section explains the use of sys.exit