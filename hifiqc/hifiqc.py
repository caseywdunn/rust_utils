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
    
    # Generate a histogram of read lengths
    hist_len = df['length'].plot.hist(bins=100)
    fig_hist_len = hist_len.get_figure()
    fig_hist_len.axes[0].set_xlabel("length")
    fig_hist_len.savefig(out_file_name + ".hist_length.png")
    fig_hist_len.clf()

    # Generate a histogram of read scores
    hist_readscore = df['readscore'].plot.hist(bins=100)
    fig_hist_readscore = hist_readscore.get_figure()
    fig_hist_readscore.axes[0].set_xlabel("readscore")
    fig_hist_readscore.savefig(out_file_name + ".hist_readscore.png")
    fig_hist_readscore.clf()

    # generate a histogram of np
    hist_np = df['np'].plot.hist(bins=100)
    fig_hist_np = hist_np.get_figure()
    fig_hist_np.axes[0].set_xlabel("np")
    fig_hist_np.savefig(out_file_name + ".hist_np.png")
    fig_hist_np.clf()

    # Generate a histogram of quality_mean
    hist_qm = df['quality_mean'].plot.hist(bins=100)
    fig_hist_qm = hist_qm.get_figure()
    fig_hist_qm.axes[0].set_xlabel("quality_mean")
    fig_hist_qm.savefig(out_file_name + ".hist_quality_mean.png")
    fig_hist_qm.clf()

    # hexbin plots
    ax = df.plot.hexbin(x='length', y='np', gridsize=20, bins='log')
    fig = ax.get_figure()
    fig.savefig(out_file_name + ".np_v_length.png")
    fig.clf()

    ax = df.plot.hexbin(x='length', y='readscore', gridsize=20, bins='log')
    fig = ax.get_figure()
    fig.savefig(out_file_name + ".readscore_v_length.png")
    fig.clf()

    ax = df.plot.hexbin(x='np', y='readscore', gridsize=20, bins='log')
    fig = ax.get_figure()
    fig.savefig(out_file_name + ".readscore_v_np.png")
    fig.clf()

    ax = df.plot.hexbin(x='quality_mean', y='readscore', gridsize=20, bins='log')
    fig = ax.get_figure()
    fig.savefig(out_file_name + ".readscore_v_qualitymean.png")
    fig.clf()

    ax = df.plot.hexbin(x='length', y='quality_mean', gridsize=20, bins='log')
    fig = ax.get_figure()
    fig.savefig(out_file_name + ".qualitymean_v_length.png")
    fig.clf()   
    
    # Open the output log file
    with open(out_file_name + ".log", 'w') as log_file:
        log_file.write("Date and time: " + str(datetime.datetime.now()))
        log_file.write("Input file: " + in_file)
        log_file.write("Output file: " + out_file_name)
        log_file.write("Number of reads: " + str(len(df)))
        log_file.write("Total length: " + str(sum(df['length'])))
        log_file.write("N50: " + str(n50(df['length'])))
    
    return 0
        
if __name__ == '__main__':
    sys.exit(main())  # next section explains the use of sys.exit