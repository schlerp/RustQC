pub struct QCReport {
    name: String,
    path: String,
    seq_position_scores: Vec<Vec<u32>>,
    qual_by_position: QualityByPosition,
    qual_by_seq: QualityBySequence,
    base_by_position: BaseByPosition,
}

pub struct QualityByPosition {
    pub seq_position_means: Vec<f32>,
    pub seq_position_medians: Vec<f32>,
    pub seq_position_q1s: Vec<f32>,
    pub seq_position_q3s: Vec<f32>,
    pub seq_position_mins: Vec<u32>,
    pub seq_position_maxs: Vec<u32>,
    pub seq_position_ns: Vec<u32>,
}

pub struct QualityBySequence {
    seq_mean_quals: Vec<f32>,
    seq_min_quals: Vec<u32>,
    seq_max_quals: Vec<u32>,
}

pub struct BaseByPosition {
    a_bases: Vec<u32>,
    c_bases: Vec<u32>,
    g_bases: Vec<u32>,
    t_bases: Vec<u32>,
}
