# Bincompress

In metagenomics often one needs to bin assemblies using different binners.
Storing all bins for a long time together with the assembly can 
be a large amount of data.

In reality the binning information is easily removed from the bins and
together with the assembly bins can be recreated.

Bincompress is a small rust cli that can compress folders of bins
into a single JSON file and later restore the bins from the assembly.

Restoration is correct to the last bit.


## Assumptions

The assumption is that the fasta files have consitent line-lengths within each file.
You can not use this on fastas with the sequence in a single line. Although this
is valid fasta format, it is not supported. 
This is due to the way we support checksumming and need to restore the file
correctly again.


Assembly and binner do not need to have the same line lengths, but need
to have the same contig names.


## Install

Its a rust project so make sure cargo is installed then run:

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
bincompress restore bins.json.gz assembly.fasta 
```


## Test

You can create simulated bins using the python notebook. 

Test compression with the simulated data by running:

```
cargo run -- compress test_data/simulation/bins/binner_{1,2,3,4}  \
-o test_data/simulation/bins.json.gz
```
After you should be able to restore it using:


## Issues

I am learning Rust so the tool will have many issues. 
The project is small enough that complete rewrites should be possible.

Feel free to suggest massive changes if you want.

### Known Issues/Todo
- No tests [In progress]
- No validation of input data
- No validation of successfull restore [Done, using sha256]
- No documentation
- The tool should work with JSON [Done, now using json]
- Does not retain order of contigs [It does now]
- Allow Gzipped assemblies [Works, but needs improvement as its very slow]
