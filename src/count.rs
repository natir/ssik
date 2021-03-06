/*
Copyright (c) 2020 Pierre Marijon <pmarijon@mpi-inf.mpg.de>

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

/* crate use */
use anyhow::{anyhow, Context, Result};

/* local use */
use crate::error::IO::*;
use crate::error::*;
use crate::*;

pub fn count(params: cli::SubCommandCount) -> Result<()> {
    let params = cli::check_count_param(params)?;

    let record_buffer = if let Some(len) = params.record_buffer {
        len
    } else {
        8192
    };

    log::info!("Start of count structure initialization");
    let mut counter = counter::Counter::new(params.kmer);
    log::info!("End of count structure initialization");

    for input in params.inputs.iter() {
        log::info!("Start of kmer count of the file {}", input);
        let reader = niffler::get_reader(Box::new(
            std::fs::File::open(input)
                .with_context(|| Error::IO(CantOpenFile))
                .with_context(|| anyhow!("File {}", input.clone()))?,
        ))
        .with_context(|| anyhow!("File {}", input.clone()))?
        .0;

        counter.count_fasta(reader, record_buffer);

        log::info!("End of kmer count of the file {}", &input);
    }

    dump::dump_worker(
        counter,
        params.output,
        params.csv,
        params.solid,
        params.spectrum,
        params.abundance,
    );

    Ok(())
}
