# dimploma-stas-kek
The one and only diploma project implemented by my best student everrrrr



# Ideas/Thoughts:
- we need to know some parameters of errors in nanopore, here is why:
usual aligning takes place when errors are more on "low probability" side of things, but in nanopore it's in 10% that there will be some error in reading.
usualy it's rare that illumina or other sequencing devices skip ore multiply pairs, but for nanopore it's typical, we need to find way to handle this (really i dont know how for now).
- other aligners use smith-winston aligner when, for example when we aligned one of pair-wise reads, so we can align other one; maybe we need to use it too, because in the my original idea we needed to map only short occurencies and spaces between them with something else.
- need to check how nanopore gives output and etc
- we don't need to think about long-gapped reads because who the heck will use ultra long reads to sequence short RNA (because genes is much shorter than our target - 100k+ reads).





# Links section:
- Lecture about BWA and aligneing https://www.youtube.com/watch?v=P3ORBMon8aw
- https://www.cs.cmu.edu/~ckingsf/bioinfo-lectures/bwt.pdf
- https://www.coursera.org/lecture/algorithms-on-strings/burrows-wheeler-transform-GAA6S
- https://www.hackerearth.com/practice/data-structures/advanced-data-structures/suffix-arrays/tutorial/

- Where to find fastq data from miniion <https://www.ebi.ac.uk/ena/data/warehouse/search?query=%22instrument_model=%22MinION%22%22&domain=read>




# Main nanopore aligner:
<https://github.com/lh3/minimap2>
In "Limitations" section they telling:
- Minimap2 does not work with a single query or database sequence ~2 billion bases or longer (2,147,483,647 to be exact). The total length of all sequences can well exceed this threshold.
I wonder: probably they use uint32 for numerating arrays of bases, but why?
- Minimap2 often misses small exons.
After reading article I still don't understand why it could happen.




# Short term tasks (less than one month):
- Get some book/tutorials/lectures about dynamic programming, as i saw from articles, it's must have. After it, probably I need try to code some simple programming challanges connected to dynamic programming.
- Read Roberts,M. et al. (2004) Reducing storage requirements for biological sequence comparison. Bioinformatics, 20, 3363–3369.
- read about heuristic algorithms
- wtf is homopolymercompressed k-mers
- i need to know how the fuck they generate genome efficently, becuase even for 50k nucleotides it's already needs 5seconds on one core (and 2.5 Gb RAM)

# Long term tasks:
- write aligner similar/based on BWA to understand challanges of this process.



Some things i've already done:
- watched https://www.youtube.com/watch?v=P3ORBMon8aw , and finally understood how bwa and bowtie works, but now i have even more questions/ides that i'll write to ideas section
- write Burrows–Wheeler transformer
