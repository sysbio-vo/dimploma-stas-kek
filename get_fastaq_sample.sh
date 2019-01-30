#!/bin/sh




mkdir samples 
wget ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR595/008/SRR5951588/SRR5951588.fastq.gz -P samples
tar -xzf samples/SRR5951588.fastq.gz

