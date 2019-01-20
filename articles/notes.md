Folder for downloaded articles, and probably some kind of literature overview.


li2018.pdf
Heng Li Minimap2: pairwise aligment for nucleotide sequences.
As written in materials section:
"Minimap2 follows a typical seed-chain-align procedure as is used by most full-genome aligners. It collects minimizers (Roberts et al., 2004) of the reference sequences and indexes them in a hash table, with the key being the hash of a minimizer and the value being a list of locations of the minimizer copies. Then for each query sequence, minimap2 takes query minimizers as seeds, finds exact matches (i.e. anchors) to the reference, and identifies sets of coli-near anchors as chains. If base-level alignment is requested, mini-map2 applies dynamic programming (DP) to extend from the endsof chains and to close regions between adjacent anchors in chains. Minimap2 uses indexing and seeding algorithms similar to mini-map (Li, 2016), and furthers the predecessor with more accurate chaining, the ability to produce base-level alignment and the support of spliced alignment."
Notes:
- across article they talk about reads that are ~100kb in length, so our optimal target probalby will be in 1Mb+ range(if we want to take this goal)
- they are basing indexing of genome on SmartDenovo (homopolymercomressed k-mers) (really don't know what does it means, need to read more)
- based on that they are writing in their article there is several stages of aligning:
	- chaining 
	- aligning
	- aligning splices sequences
	- aligning short paired-end reads
for some reason they divide process in three different alignings.
- they still have much of math that i don't understand :(((



