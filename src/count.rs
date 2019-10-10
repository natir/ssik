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
use crate::counter;
use crate::write;
use crate::bucketizer;

use crate::bucketizer::Bucket;

pub fn count(input_path: &str, output_path: &str, k: u8, m: u8) -> () {
    let reader = bio::io::fasta::Reader::new(std::io::BufReader::new(std::fs::File::open(input_path).unwrap()));

    let mut counter: counter::ShortCounter = counter::ShortCounter::new(k);
    let mut bucketizer: bucketizer::Prefix<u8> = bucketizer::Prefix::new(&mut counter, k);

    let minimizer_mask = generate_mask(m * 2);
    let kmer_mask = generate_mask(k * 2);

    for record in reader.records() {
        let result = record.unwrap();
        let line = result.seq();
        
        let mut kmer = convert::seq2bit(&line[0..k as usize]);

        // found minimizer of first kmer
        //let (mut minimizer, mut mini_hash, mut mini_pos) = found_minimizer(kmer, k, m, minimizer_mask);

        for nuc in &line[1..] {
            // create space for new nuc; clean old nuc; add new nuc
            kmer = ((kmer << 2) & kmer_mask) | convert::nuc2bit(*nuc);

            /* Minimizer search
            mini_pos += 2; 
            
            if mini_pos > (k * 2 - m * 2) as i8 { // last minimizer go away we need to found the new minimizer
                let tmp = found_minimizer(kmer, k, m, minimizer_mask);
                minimizer = tmp.0; mini_hash = tmp.1; mini_pos = tmp.2;
            } else {
                if let Some(tmp) = update_minimizer(kmer, minimizer_mask, mini_hash) { // check if new nuc create a lowest minimizer
                    minimizer = tmp.0; mini_hash = tmp.1; mini_pos = tmp.2;
                }
            }
            */

            // add kmer in count structure
            bucketizer.add_kmer(convert::remove_first_bit(convert::cannonical(kmer, k)));
        }
    }

    bucketizer.clean_all_buckets();

    write::write(&counter, output_path, k);
}

fn generate_mask(size: u8) -> u64 {
    return (1 << size) - 1;
}

fn minimizer_mask(mask: u64, offset: usize) -> u64 {
    return mask << offset;
}

fn update_minimizer(kmer: u64, mask: u64, hash: i64) -> Option<(u64, i64, i8)> {
    let new_mini = kmer & mask;
    let new_hash = revhash(new_mini);
    
    if new_hash < hash {
        return Some((new_mini, new_hash, 0));
    }

    return None
}

fn found_minimizer(kmer: u64, k: u8, m: u8, mask: u64) -> (u64, i64, i8) {
    let mut minimizer = kmer & minimizer_mask(mask, 0);
    let mut minimizer_hash = revhash(minimizer);
    let mut index = 0;

    for i in (2..=(k as usize * 2 - m as usize * 2)).step_by(2) {
        let subk = (kmer & minimizer_mask(mask, i)) >> i;
        let hash = revhash(subk);
        if minimizer_hash > hash {
            minimizer = subk;
            minimizer_hash = hash;
            index = i;
        }
    }

    return (minimizer, minimizer_hash, index as i8);
}

fn revhash(mut x: u64) -> i64 {
    x = ((x >> 32) ^ x).wrapping_mul(0xD6E8FEB86659FD93);
    x = ((x >> 32) ^ x).wrapping_mul(0xD6E8FEB86659FD93);
    x = (x >> 32) ^ x;

    return x as i64;
}

#[cfg(test)]
mod test {
    use super::*;
}
