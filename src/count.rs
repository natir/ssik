/*
Copyright (c) 2019 Pierre Marijon <pierre.marijon@inria.fr>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

/* project use */
use crate::convert;
use crate::write;
use crate::counter;

pub fn count(input_path: &str, output_path: &str, k: u8, abundance_min: u8) -> () {
    let reader = bio::io::fasta::Reader::new(std::io::BufReader::new(
        std::fs::File::open(input_path).unwrap(),
    ));

    // init counter
    // let mut kmer2count: Vec<u8> = vec![0; 1 << (k * 2 - 1)];

    let mut counter = counter::Counter::new(k);
    
    // count
    for result in reader.records() {
        let record = result.unwrap();

        for subseq in record.seq().windows(k as usize) {
            let hash = hash(subseq, k);
            counter.add_bit(hash, k);
            //add_in_counter(&mut kmer2count, hash);
        }
    }

    for i in 0..4096 {
        counter.clean_buckets(i);
    }
    
    
    write::write(counter.counts, output_path, k, abundance_min);
}

fn add_in_counter(kmer2count: &mut Vec<u8>, hash: usize) -> () {
    kmer2count[hash] = kmer2count[hash].saturating_add(1);
}

fn hash(kmer: &[u8], k: u8) -> u64 {
    return convert::cannonical(convert::seq2bit(kmer), k) >> 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_() {
        // TAGGC -> 100011110
        assert_eq!(hash(b"TAGGC", 5), 0b100011110);

        // GCCTA -> 110101100
        assert_eq!(hash(b"GCCTA", 5), 0b100011110);
    }
}
