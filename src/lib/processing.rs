extern crate needletail;
extern crate rdxsort;

use super::structs::*;
use needletail::{parse_fastx_file, FastxReader};
use rdxsort::*; // std sort was too slow for large vecs

const A: u8 = 0x41;
const C: u8 = 0x43;
const G: u8 = 0x47;
const T: u8 = 0x54;
const AA: u8 = 0x61;
const CC: u8 = 0x63;
const GG: u8 = 0x67;
const TT: u8 = 0x74;

pub fn load_fastq(path: &String) -> Box<dyn FastxReader> {
    println!("fetching file at {}!", path);
    let file_error = &format!("File {} not found!", path);
    parse_fastx_file(&path).expect(file_error)
}

fn calc_means_ns(seq_position_scores: &[Vec<u32>]) -> (Vec<f32>, Vec<u32>) {
    // calculate average
    let mut means: Vec<f32> = Vec::new();
    let mut ns: Vec<u32> = Vec::new();
    for scores in seq_position_scores.iter() {
        let mut sum: u64 = 0;
        for x in scores {
            sum += *x as u64;
        }
        let n = scores.len();
        means.push(sum as f32 / n as f32);
        ns.push(n as u32);
    }
    (means, ns)
}

fn calc_median_q1_q3(seq_position_scores: &[Vec<u32>]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut medians: Vec<f32> = Vec::new();
    let mut q1s: Vec<f32> = Vec::new();
    let mut q3s: Vec<f32> = Vec::new();
    for scores in seq_position_scores.iter() {
        let mid = scores.len() / 2;
        let q1 = scores.len() / 4;
        let q3 = q1 * 3;

        // handle medians
        if mid % 2 == 0 {
            medians.push((scores[mid] + scores[mid + 1]) as f32 / 2 as f32);
        } else {
            medians.push(scores[mid] as f32)
        }
        // handle q1s
        if q1 % 2 == 0 {
            q1s.push((scores[q1] + scores[q1 + 1]) as f32 / 2 as f32);
        } else {
            q1s.push(scores[q1] as f32)
        }
        // handle q3s
        if q3 % 2 == 0 {
            q3s.push((scores[q3] + scores[q3 + 1]) as f32 / 2 as f32);
        } else {
            q3s.push(scores[q3] as f32)
        }
    }
    (medians, q1s, q3s)
}

fn calc_mins_maxs(seq_position_scores: &[Vec<u32>]) -> (Vec<u32>, Vec<u32>) {
    let mut mins: Vec<u32> = Vec::new();
    let mut maxs: Vec<u32> = Vec::new();
    for scores in seq_position_scores.iter() {
        mins.push(*scores.iter().min().unwrap() as u32);
        maxs.push(*scores.iter().max().unwrap() as u32);
    }
    (mins, maxs)
}

fn create_quality_by_pos_report(seq_position_scores: Vec<Vec<u32>>) -> QualityByPosition {
    let (means, ns) = calc_means_ns(&seq_position_scores.as_slice());
    let (mins, maxs) = calc_mins_maxs(&seq_position_scores.as_slice());
    let (medians, q1s, q3s) = calc_median_q1_q3(&seq_position_scores.as_slice());
    QualityByPosition {
        seq_position_means: means,
        seq_position_medians: medians,
        seq_position_q1s: q1s,
        seq_position_q3s: q3s,
        seq_position_mins: mins,
        seq_position_maxs: maxs,
        seq_position_ns: ns,
    }
}

pub fn process_fastq(path: &String) -> QualityByPosition {
    let mut reader = load_fastq(path);
    let mut seq_position_scores: Vec<Vec<u32>> = Vec::new();

    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        let mut i = 0;
        let mut qual_iter = seqrec.qual().unwrap().iter();
        while let Some(qual) = qual_iter.next() {
            if i >= seq_position_scores.len() {
                seq_position_scores.push(Vec::new());
            }
            seq_position_scores[i].push(*qual as u32 - 33);
            i += 1;
        }
    }
    // sort here to avoid clone later
    for scores in seq_position_scores.iter_mut() {
        scores.rdxsort();
    }
    create_quality_by_pos_report(seq_position_scores)
}