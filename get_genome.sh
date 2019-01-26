#!/bin/sh


mkdir reference_genome
wget ftp://ftp.ncbi.nlm.nih.gov/refseq/H_sapiens/annotation/GRCh38_latest/refseq_identifiers/GRCh38_latest_genomic.fna.gz -P reference_genome
tar -ezf reference_genome/GRCh38_latest_genomic.fna.gz
