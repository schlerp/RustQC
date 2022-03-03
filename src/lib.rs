use bio::io::fastq;
use rdxsort::*; // std sort was too slow for large vecs
use std::fs::File;
use std::io::BufReader;
use flate2::bufread;

const A: u8 = 0x41;
const C: u8 = 0x43;
const G: u8 = 0x47;
const T: u8 = 0x54;
const AA: u8 = 0x61;
const CC: u8 = 0x63;
const GG: u8 = 0x67;
const TT: u8 = 0x74;

fn get_fastq_reader(path: &String) -> Box<dyn (::std::io::Read)> {
    if path.ends_with(".gz") {
        let f = fs::File::open(path).unwrap();
        Box::new(bufread::MultiGzDecoder::new(BufReader::new(f)).unwrap())
    } else {
        Box::new(fs::File::open(path).unwrap())
    }
}

pub fn load_fastq(path: String) -> fastq::Reader<BufReader<File>> {
    println!("fetching file at {}!", path);
    let file_error = &format!("File {} not found!", path);
    let f = get_fastq_reader(&path);
    fastq::Reader::new(f)
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

fn calc_mins_maxs(seq_position_scores: &[Vec<u32>]) -> (Vec<f32>, Vec<f32>) {
    let mut mins: Vec<f32> = Vec::new();
    let mut maxs: Vec<f32> = Vec::new();
    for scores in seq_position_scores.iter() {
        mins.push(*scores.iter().min().unwrap() as f32);
        maxs.push(*scores.iter().max().unwrap() as f32);
    }
    (mins, maxs)
}

fn print_qc_report(qc_report: QualityByPosition) {
    print!("RustQC Report\n");
    print!("=============\n");
    print!("Quality by Sequence Position\n");
    print!("  LEGEND:\n");
    print!("    < = min\n");
    print!("    > = max\n");
    print!("    [ = q1, ] = q3\n");
    print!("    + = median\n");
    print!(
        "NOTE: median takes highest precedence, followed by q1/q3 finally followed by min/max.\n"
    );
    print!("              |          1         2         3         4        |\n");
    print!("              |0123456789012345678901234567890123456789012345689|\n");
    let mut i = 0;
    while i < qc_report.seq_position_means.len() {
        let min_space = qc_report.seq_position_mins[i] as usize;
        let min_q1_space =
            qc_report.seq_position_q1s[i] as usize - qc_report.seq_position_mins[i] as usize;
        let q1_median_space = qc_report.seq_position_medians[i].floor() as usize
            - qc_report.seq_position_q1s[i] as usize;
        let median_q3_space = qc_report.seq_position_q3s[i].floor() as usize
            - qc_report.seq_position_medians[i].floor() as usize;
        let q3_max_space = qc_report.seq_position_maxs[i].floor() as usize
            - qc_report.seq_position_q3s[i].floor() as usize;
        let max_space = 50 - qc_report.seq_position_maxs[i].floor() as usize;
        let mut row_string = String::from(format!(
            "{}{}{}{}{}{}",
            " ".repeat(min_space),
            "-".repeat(min_q1_space),
            " ".repeat(q1_median_space),
            " ".repeat(median_q3_space),
            "-".repeat(q3_max_space),
            " ".repeat(max_space)
        ));
        row_string.replace_range(
            qc_report.seq_position_mins[i] as usize..qc_report.seq_position_mins[i] as usize + 1,
            "<",
        );
        row_string.replace_range(
            qc_report.seq_position_maxs[i] as usize..qc_report.seq_position_maxs[i] as usize + 1,
            ">",
        );
        row_string.replace_range(
            qc_report.seq_position_q1s[i] as usize..qc_report.seq_position_q1s[i] as usize + 1,
            "[",
        );
        row_string.replace_range(
            qc_report.seq_position_q3s[i] as usize..qc_report.seq_position_q3s[i] as usize + 1,
            "]",
        );
        row_string.replace_range(
            qc_report.seq_position_medians[i] as usize
                ..qc_report.seq_position_medians[i] as usize + 1,
            "|",
        );
        // row_string.replace_range(
        //     qc_report.seq_position_means[i] as usize..qc_report.seq_position_means[i] as usize + 1,
        //     "x",
        // );
        println!(
            "location {:3}: |{}| (n={})",
            i, row_string, qc_report.seq_position_ns[i]
        );
        i += 1;
    }
    print!("              |0123456789012345678901234567890123456789012345689|\n");
    print!("              |          1         2         3         4        |\n");
}

struct QCReport {
    name: String,
    path: String,
    seq_position_scores: Vec<Vec<u32>>,
    qual_by_position: QualityByPosition,
    qual_by_seq: QualityBySequence,
    base_by_position: BaseByPosition,
}

struct QualityByPosition {
    seq_position_means: Vec<f32>,
    seq_position_medians: Vec<f32>,
    seq_position_q1s: Vec<f32>,
    seq_position_q3s: Vec<f32>,
    seq_position_mins: Vec<f32>,
    seq_position_maxs: Vec<f32>,
    seq_position_ns: Vec<u32>,
}

struct QualityBySequence {
    seq_mean_quals: Vec<f32>,
    seq_min_quals: Vec<u32>,
    seq_max_quals: Vec<u32>,
}

struct BaseByPosition {
    a_bases: Vec<u32>,
    c_bases: Vec<u32>,
    g_bases: Vec<u32>,
    t_bases: Vec<u32>,
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

pub fn process_fastq(path: String) {
    let reader = load_fastq(path);
    let mut seq_position_scores: Vec<Vec<u32>> = Vec::new();

    for result in reader.records() {
        let record = result.expect("Error during fastq record parsing");
        let mut i = 0;
        for qual in record.qual().iter() {
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
    let quality_by_pos_report = create_quality_by_pos_report(seq_position_scores);
    print_qc_report(quality_by_pos_report);
}
