# RustQC

It's a quality checking tool for fastq files like FastQC except its implemented in rust and outputs its QC's as plain text.

```bash
rustqc --path /path/to/file.fastq
```

Currently very alpha but showing some promising speed and I personally really like ASCII charts. Currently only implementing a single QC check which is the quality by position report. See example output below for a 400MB fastq file processed on my laptop in ~8 seconds.

```
RustQC Report
=============

File: /home/schlerp/Downloads/MSHR8970_MIXED_1_sequence.fastq.gz

Quality by Sequence Position
----------------------------
  A box plot representation of each position in the reads.
  Median takes highest precedence, followed by q1/q3 finally followed by min/max.

                            <-----[     |     ]----->
                           min   q1   median  q3   max

         |          1         2         3         4         |
         |01234567890123456789012345678901234567890123456789|
pos   0: |  <-----------------------------|---->            | (n=4956295, mean=31.47)
pos   1: |  <-----------------------------|---->            | (n=4956295, mean=31.47)
pos   2: |            <-------------------[    |            | (n=4956295, mean=35.25)
pos   3: |            <------------------------|            | (n=4956295, mean=36.01)
pos   4: |  <----------------------------------|            | (n=4956295, mean=36.20)
pos   5: |  <--------------------------------------|        | (n=4956295, mean=39.64)
pos   6: |            <----------------------------|        | (n=4956295, mean=39.78)
pos   7: |  <--------------------------------------|        | (n=4956295, mean=39.74)
pos   8: |            <----------------------------|        | (n=4956295, mean=39.87)
pos   9: |            <----------------------------|        | (n=4956295, mean=39.85)
pos  10: |            <----------------------------|        | (n=4956295, mean=39.87)
...
pos 130: |            <------------------------[   |        | (n=4956295, mean=36.11)
pos 131: |  <----------------------------------[   |        | (n=4956295, mean=36.04)
pos 132: |            <------------------------[   |        | (n=4956295, mean=35.94)
pos 133: |            <------------------------[   |        | (n=4956295, mean=35.82)
pos 134: |            <-------------------[        |        | (n=4956295, mean=35.70)
pos 135: |  <-----------------------------[        |        | (n=4956295, mean=35.63)
pos 136: |            <-------------------[        |        | (n=4956295, mean=35.48)
pos 137: |            <-------------------[        |        | (n=4956295, mean=35.35)
pos 138: |  <-----------------------------[        |        | (n=4956295, mean=35.25)
pos 139: |            <-------------------[        |        | (n=4956295, mean=35.21)
pos 140: |  <-----------------------------[        |        | (n=4956295, mean=35.05)
pos 141: |  <-----------------------------[        |        | (n=4956295, mean=34.93)
pos 142: |  <-----------------------------[        |        | (n=4956295, mean=34.78)
pos 143: |  <-----------------------------[        |        | (n=4956295, mean=34.64)
pos 144: |  <-----------------------------[        |        | (n=4956295, mean=34.49)
pos 145: |  <-----------------------------[        |        | (n=4956295, mean=34.35)
pos 146: |        <-----------------------[        |        | (n=4956295, mean=34.23)
pos 147: |  <-----------------------------[        |        | (n=4956295, mean=34.08)
pos 148: |  <-----------------------------[        |        | (n=4956295, mean=33.98)
pos 149: |        <-----------------------[    |   ]        | (n=4956295, mean=33.83)
         |01234567890123456789012345678901234567890123456789|
         |          1         2         3         4         |
```
