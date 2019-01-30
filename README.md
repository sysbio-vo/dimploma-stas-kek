# dimploma-stas-kek
The one and only diploma project implemented by my best student everrrrr




# Helpfull defentions:
![SA](/included/sa_defenition.png?raw=true "SA")

![bwt](/included/bwt_karkkainen.png?raw=true "bwt")








# Ideas/Thoughts:
- we need to know some parameters of errors in nanopore, here is why:
usual aligning takes place when errors are more on "low probability" side of things, but in nanopore it's in 10% that there will be some error in reading.
usualy it's rare that illumina or other sequencing devices skip ore multiply pairs, but for nanopore it's typical, we need to find way to handle this (really i dont know how for now).
	- hey, there is pic around end of this documents with error rates. One of interesting findings - "Some properties of nucleic acid sequences are known to raise the error rates for all or most technologies, such as extremes in GC content, long homopolymer stretches, the presence of human promoter sequences and the well-known decay of the base signal along each read."
	- accoring to David Laehnemann, 2015 <https://academic.oup.com/bib/article/17/1/154/1742474> : " Averaging over three replicates of the phage M13, they report 5.1% substitution errors, 7.8% deletion errors and 4.9% insertion errors and also apply their error estimation to the Escherichia coli data of Quick et al., where they find 5.3% substitution errors, 9.1% deletion errors and 6.0% insertion errors.
	Altogether, these studies roughly agree that substitution and insertion errors occur at a similar rate, while deletion errors are about two times as common. The joint error rate of about 20–25% still clearly exceeds that of PacBio, the other current single-molecule sequencing technique. Also in contrast to PacBio, errors seem to be biased: with substitution errors, A to T and T to A errors are much less likely than all other substitution errors [30] and homopolymer runs seem to increase insertion and deletion error rates [30, 33]."

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
- read about heuristic algorithms
- wtf is homopolymercompressed k-mers
- need to find how efficently find uniqe short substrings in long strings (for ex: 20bp unique sequences in 100k bp)
- download minimap and test it by myself
- read minimap source-code
- re-read minimap1 and minimap2 articles


# Long term tasks:
- write aligner similar/based on BWA to understand challanges of this process.



# Some things i've already done:
- watched https://www.youtube.com/watch?v=P3ORBMon8aw , and finally understood how bwa and bowtie works, but now i have even more questions/ides that i'll write to ideas section
- write Burrows–Wheeler transformer
- i need to know how they generate genome efficently, becuase even for 50k nucleotides it's already needs 5seconds on one core (and 2.5 Gb RAM)
(Okay, so in bowtie2 sources they write they use : "Fast BWT in Small Space by Blockwise Suffix Sorting Juha K¨arkk¨ainen")(it's avalible in articles/bwt folder)
- Read Roberts,M. et al. (2004) Reducing storage requirements for biological sequence comparison. Bioinformatics, 20, 3363–3369.





# Pics
from <https://academic.oup.com/bib/article/17/1/154/1742474>
![erorrates](/included/error_rates.png?raw=true "errorrates")
