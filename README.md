# dimploma-stas-kek
The one and only diploma project implemented by my best student everrrrr





# Links section:




#Main nanopore aligner:
<https://github.com/lh3/minimap2>
In "Limitations" section they telling:
- Minimap2 does not work with a single query or database sequence ~2 billion bases or longer (2,147,483,647 to be exact). The total length of all sequences can well exceed this threshold.
I wonder: probably they use uint32 for numerating arrays of bases, but why?
- Minimap2 often misses small exons.
After reading article I still don't understand why it could happen.




#Short term tasks (less than one month):
- Get some book/tutorials/lectures about dynamic programming, as i saw from articles, it's must have. After it, probably I need try to code some simple programming challanges connected to dynamic programming.
- Read Roberts,M. et al. (2004) Reducing storage requirements for biological sequence comparison. Bioinformatics, 20, 3363–3369.
- read about heuristic algorithms
- wtf is homopolymercompressed k-mers
- write Burrows–Wheeler transformer

#Long term tasks:
- write aligner similar/based on BWA to understand challanges of this process.

