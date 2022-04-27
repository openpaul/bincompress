# Bincompress

In metagenomics often one needs to bin assemblies using different binners.
Storing all bins for a long time together with the assembly can 
be a large amount of data.

In reality the binning information is easily removed from the bins and
together with the assembly bins can be recreated.

Bincompress is a small rust cli that can compress folders of bins
into a single table and later restore the bins from the assembly.


## Install

its a rust project so make sure cargo is installed then run:

```
git clone https://github.com/openpaul/bincompress
cd bincompress
cargo build -r
```

The binary `bincompress` can the be found in:

`target/release/bincompress`


## Compress
```
bincompress compress binner1_bins binner2_bins
```

## Restore 
```
bincompress restore bins.csv.gz assembly.fasta 
```

## Issues

I am learning Rust so the tool will have many issues. 
The project is small enough that complete rewrites should be possible.

Feel free to suggest massive changes if you want.

### Known Issues/Todo
- No tests
- No validation of input data
- No validation of successfull restore
- No documentation
