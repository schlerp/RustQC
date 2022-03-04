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

pub fn load_fastq(path: &str) -> Box<dyn FastxReader> {
    //println!("fetching file at {}!", path);
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
            medians.push((scores[mid] + scores[mid + 1]) as f32 / 2.0);
        } else {
            medians.push(scores[mid] as f32)
        }
        // handle q1s
        if q1 % 2 == 0 {
            q1s.push((scores[q1] + scores[q1 + 1]) as f32 / 2.0);
        } else {
            q1s.push(scores[q1] as f32)
        }
        // handle q3s
        if q3 % 2 == 0 {
            q3s.push((scores[q3] + scores[q3 + 1]) as f32 / 2.0);
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
    let (means, ns) = calc_means_ns(seq_position_scores.as_slice());
    let (mins, maxs) = calc_mins_maxs(seq_position_scores.as_slice());
    let (medians, q1s, q3s) = calc_median_q1_q3(seq_position_scores.as_slice());
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

fn create_base_by_position_report(seq_position_bases: Vec<Vec<u32>>) -> BaseByPosition {
    let seq_position_bases_t = transpose2(seq_position_bases);
    BaseByPosition {
        a_bases: seq_position_bases_t[0].clone(),
        c_bases: seq_position_bases_t[1].clone(),
        g_bases: seq_position_bases_t[2].clone(),
        t_bases: seq_position_bases_t[3].clone(),
        other_bases: seq_position_bases_t[4].clone(),
    }
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn process_fastq(path: &str) -> (QualityByPosition, BaseByPosition) {
    let mut reader = load_fastq(path);
    let mut seq_position_scores: Vec<Vec<u32>> = Vec::new();
    let mut seq_position_bases: Vec<Vec<u32>> = Vec::new();

    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        let qual_iter = seqrec.qual().unwrap().iter();
        let seq_iter = seqrec.raw_seq().iter();
        for (i, qual) in qual_iter.enumerate() {
            if i >= seq_position_scores.len() {
                seq_position_scores.push(Vec::new());
            }
            seq_position_scores[i].push(*qual as u32 - 33);
        }

        for (i, base) in seq_iter.enumerate() {
            if i >= seq_position_bases.len() {
                seq_position_bases.push(vec![0, 0, 0, 0, 0]);
            }
            if [A, AA].contains(base) {
                seq_position_bases[i][0] += 1;
            } else if [C, CC].contains(base) {
                seq_position_bases[i][1] += 1;
            } else if [G, GG].contains(base) {
                seq_position_bases[i][2] += 1;
            } else if [T, TT].contains(base) {
                seq_position_bases[i][3] += 1;
            } else {
                seq_position_bases[i][4] += 1;
            }
        }
    }
    // sort here to avoid clone later
    for scores in seq_position_scores.iter_mut() {
        scores.rdxsort();
    }
    (
        create_quality_by_pos_report(seq_position_scores),
        create_base_by_position_report(seq_position_bases),
    )
}
