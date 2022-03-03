use super::structs::*;

pub fn print_report_header(path: &String) {
    print!("RustQC Report\n");
    print!("=============\n\n");
    println!("File: {}", path);
    print!("\n");
}

pub fn print_qual_by_position_report(qc_report: QualityByPosition) {
    print!("Quality by Sequence Position\n");
    print!("----------------------------\n");
    print!("  A box plot representation of each position in the reads.\n");
    print!("  Median takes highest precedence, followed by q1/q3 finally followed by min/max.\n\n");
    print!("                            <-----[     |     ]----->\n");
    print!("                           min   q1   median  q3   max\n\n");
    print!("         |          1         2         3         4         |\n");
    print!("         |01234567890123456789012345678901234567890123456789|\n");
    let mut i = 0;
    while i < qc_report.seq_position_means.len() {
        let min_space = qc_report.seq_position_mins[i] as usize;
        let min_q1_space =
            qc_report.seq_position_q1s[i] as usize - qc_report.seq_position_mins[i] as usize;
        let q1_median_space = qc_report.seq_position_medians[i].floor() as usize
            - qc_report.seq_position_q1s[i] as usize;
        let median_q3_space = qc_report.seq_position_q3s[i].floor() as usize
            - qc_report.seq_position_medians[i].floor() as usize;
        let q3_max_space = qc_report.seq_position_maxs[i] as usize
            - qc_report.seq_position_q3s[i].floor() as usize;
        let max_space = 50 - qc_report.seq_position_maxs[i] as usize;
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
        println!(
            // "location {:3}: |{}| (n={},min={},q1={},median={},mean={},q3={},max={})",
            "pos {:3}: |{}| (n={}, mean={:.2})",
            i,
            row_string,
            qc_report.seq_position_ns[i],
            // qc_report.seq_position_mins[i],
            // qc_report.seq_position_q1s[i],
            // qc_report.seq_position_medians[i],
            qc_report.seq_position_means[i],
            // qc_report.seq_position_q3s[i],
            // qc_report.seq_position_maxs[i],
        );
        i += 1;
    }
    print!("         |01234567890123456789012345678901234567890123456789|\n");
    print!("         |          1         2         3         4         |\n");
}
