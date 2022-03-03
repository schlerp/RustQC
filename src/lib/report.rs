use super::structs::*;

pub fn print_report_header(path: &String) {
    print!("RustQC Report\n");
    print!("=============\n\n");
    println!("File: {}", path);
    print!("\n");
}

pub fn print_qual_by_position_report(qual_by_position: &QualityByPosition) {
    print!("Quality by Sequence Position\n");
    print!("----------------------------\n\n");
    print!("Median takes highest precedence, followed by q1/q3 finally followed by min/max.\n\n");
    print!("                            <-----[     |     ]----->\n");
    print!("                           min   q1   median  q3   max\n\n");
    print!("         |          1         2         3         4         |\n");
    print!("         |01234567890123456789012345678901234567890123456789|\n");
    let mut i = 0;
    while i < qual_by_position.seq_position_means.len() {
        let min_space = qual_by_position.seq_position_mins[i] as usize;
        let min_q1_space = qual_by_position.seq_position_q1s[i] as usize
            - qual_by_position.seq_position_mins[i] as usize;
        let q1_median_space = qual_by_position.seq_position_medians[i].floor() as usize
            - qual_by_position.seq_position_q1s[i] as usize;
        let median_q3_space = qual_by_position.seq_position_q3s[i].floor() as usize
            - qual_by_position.seq_position_medians[i].floor() as usize;
        let q3_max_space = qual_by_position.seq_position_maxs[i] as usize
            - qual_by_position.seq_position_q3s[i].floor() as usize;
        let max_space = 50 - qual_by_position.seq_position_maxs[i] as usize;
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
            qual_by_position.seq_position_mins[i] as usize
                ..qual_by_position.seq_position_mins[i] as usize + 1,
            "<",
        );
        row_string.replace_range(
            qual_by_position.seq_position_maxs[i] as usize
                ..qual_by_position.seq_position_maxs[i] as usize + 1,
            ">",
        );
        row_string.replace_range(
            qual_by_position.seq_position_q1s[i] as usize
                ..qual_by_position.seq_position_q1s[i] as usize + 1,
            "[",
        );
        row_string.replace_range(
            qual_by_position.seq_position_q3s[i] as usize
                ..qual_by_position.seq_position_q3s[i] as usize + 1,
            "]",
        );
        row_string.replace_range(
            qual_by_position.seq_position_medians[i] as usize
                ..qual_by_position.seq_position_medians[i] as usize + 1,
            "|",
        );
        println!(
            // "location {:3}: |{}| (n={},min={},q1={},median={},mean={},q3={},max={})",
            "pos {:3}: |{}| (n={}, mean={:.2})",
            i,
            row_string,
            qual_by_position.seq_position_ns[i],
            // qc_report.seq_position_mins[i],
            // qc_report.seq_position_q1s[i],
            // qc_report.seq_position_medians[i],
            qual_by_position.seq_position_means[i],
            // qc_report.seq_position_q3s[i],
            // qc_report.seq_position_maxs[i],
        );
        i += 1;
    }
    print!("         |01234567890123456789012345678901234567890123456789|\n");
    print!("         |          1         2         3         4         |\n");
    print!("\n\n");
}

pub fn print_base_overall_report(base_by_position: &BaseByPosition) {
    print!("Base Proportion Overall\n");
    print!("-----------------------\n\n");
    let sum_a: f32 = base_by_position.a_bases.iter().sum::<u32>() as f32;
    let sum_c: f32 = base_by_position.c_bases.iter().sum::<u32>() as f32;
    let sum_g: f32 = base_by_position.g_bases.iter().sum::<u32>() as f32;
    let sum_t: f32 = base_by_position.t_bases.iter().sum::<u32>() as f32;
    let sum_other: f32 = base_by_position.other_bases.iter().sum::<u32>() as f32;
    let total: f32 = sum_a + sum_c + sum_g + sum_t;

    println!(
        "Overall:\tA: {:.4}%\tC: {:.4}%\tG: {:.4}%\tT: {:.4}%\tOther: {:.4}%",
        (sum_a / total) * 100.0,
        (sum_c / total) * 100.0,
        (sum_g / total) * 100.0,
        (sum_t / total) * 100.0,
        (sum_other / total) * 100.0,
    );
    print!("\n\n");
}

pub fn print_base_by_position_report(base_by_position: &BaseByPosition) {
    print!("Base Proportion by Sequence Position\n");
    print!("------------------------------------\n\n");
    let mut i = 0;
    while i < base_by_position.a_bases.len() {
        let sum = base_by_position.a_bases[i]
            + base_by_position.c_bases[i]
            + base_by_position.g_bases[i]
            + base_by_position.t_bases[i];
        println!(
            "pos: {:4}\tA: {:.4}%\tC: {:.4}%\tG: {:.4}%\tT: {:.4}%\tOther: {:.4}%",
            i,
            (base_by_position.a_bases[i] as f32 / sum as f32) * 100.0,
            (base_by_position.c_bases[i] as f32 / sum as f32) * 100.0,
            (base_by_position.g_bases[i] as f32 / sum as f32) * 100.0,
            (base_by_position.t_bases[i] as f32 / sum as f32) * 100.0,
            (base_by_position.other_bases[i] as f32 / sum as f32) * 100.0
        );
        i += 1;
    }
    print!("\n\n");
}
