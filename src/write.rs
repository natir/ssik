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
use crate::counter;

/* std use*/
use std::io::Write;

pub enum Mode {
    ALL_COUNTS,
    COUNTS,
    KMER_COUNTS,
    NUMPY,
}

impl From<&str> for Mode {
    fn from(mode: &str) -> Self {
        match mode {
            "all_counts" => return Mode::ALL_COUNTS,
            "counts" => return Mode::COUNTS,
            "kmer_counts" => return Mode::KMER_COUNTS,
            "numpy" => return Mode::NUMPY,
            _ => return Mode::COUNTS,
        }
    }
}

pub fn write<C: counter::Counter>(count: C, output_path: &str, k: u8, mode: Mode) -> () {
    let mut out = std::io::BufWriter::new(std::fs::File::create(output_path).unwrap());

    match mode {
        Mode::ALL_COUNTS => (write_all_counts(count, out, k)),
        Mode::COUNTS => (write_counts(count, out, k)),
        Mode::KMER_COUNTS => (write_kmer_counts(count, out, k)),
        Mode::NUMPY => (write_numpy(count, out, k)),
    }
}

fn write_all_counts<C: counter::Counter>(count: C, mut out: std::io::BufWriter<std::fs::File>, k: u8) -> () {

    // write k in first bytes
    out.write(&[k, 0])
        .expect("Error during write of count on disk");

    for i in 0..(1 << (k * 2 - 1)) {
        out.write(&[count.get(i)])
            .expect("Error durring write count on disk");
    }
}
    
fn write_counts<C: counter::Counter>(count: C, mut out: std::io::BufWriter<std::fs::File>, k: u8) -> () {
    
    // write k in first bytes
    out.write(&[k, 1])
        .expect("Error during write of count on disk");

    let mut last_write = 0;
    for i in 0..(1 << (k * 2 - 1)) {
        let val = count.get(i);
        
        if val == 0 {
            continue;
        }

        let mut dist = i - last_write;
        last_write = i;

        if dist > 255 {
            // dist overflow u8 we need write a some kmer with 0 count
            let n = dist / 255;
            for _ in 0..n {
                out.write(&[255, 0])
                    .expect("Error durring write count on disk");
            }

            dist %= 255;
        }

        // write dist to last value and count of k
        out.write(&[dist as u8, val])
            .expect("Error durring write count on disk");
    }
}


fn write_kmer_counts<C: counter::Counter>(count: C, mut out: std::io::BufWriter<std::fs::File>, k: u8) -> () {
    // write k in first bytes
    out.write(&[k, 2])
        .expect("Error during write of count on disk");

    for i in 0..(1 << (k * 2 - 1)) {
        let val = count.get(i);

        if val == 0 {
            continue;
        }
        
        out.write(&[i as u8, count.get(i)])
            .expect("Error durring write count on disk");
    }
}

fn write_numpy<C: counter::Counter>(count: C, mut out: std::io::BufWriter<std::fs::File>, k: u8) -> () {

}
