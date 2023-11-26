use std::cmp::{max, min};

pub struct SegmentTree{
    pub tree: Vec<usize>,
    pub lazy: Vec<usize>,
    pub size: usize
}

impl SegmentTree{
    pub fn new_segment_tree(&mut self, n: usize){
        self.tree= vec![0; n * 4];
        self.lazy= vec![n+1; n * 4];
        self.size=n;
    }

    pub fn build(&mut self, array: &Vec<usize>, v: usize, tl: usize, tr: usize){
        if tl == tr {
            self.tree[v] = array[tl]
        } else {
            let tm: usize = (tl + tr) / 2;
            self.build(array, v*2, tl, tm);
            self.build(array, v*2+1, tm+1, tr);
            self.tree[v] = max(self.tree[v*2] , self.tree[v*2 +1]);
        }
    }

    pub fn update(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize, value: usize){
        self.update_range(v, tl, tr, l, r, value);
    }
    fn update_range(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize, value: usize){
        /*
        println!(
            "Before update_range - v: {}, tl: {}, tr: {}, l: {}, r: {}, value: {}, tree: {:?}, lazy: {:?}",
            v, tl, tr, l, r, value, self.tree, self.lazy
        );*/
        if l > r {
            return;
        }
        if l == tl && tr == r {
            let new_value: usize = min(self.tree[v], value);
            if self.tree[v] != new_value {
                self.tree[v] = new_value;
                self.lazy[v] = value;
            }
            //self.tree[v as usize] += min(self.tree[v as usize], value);
            //self.lazy[v as usize] += min(self.lazy[v as usize], value);
        } else {
            self.push(v);
            let tm = (tl + tr) / 2;
            self.update_range(v*2, tl, tm, l, min(r, tm), value);
            self.update_range(v*2 + 1, tm + 1, tr, max(l, tm + 1), r, value);
            self.tree[v] = max(self.tree[v * 2], self.tree[v*2 + 1]);
        }
        /*
        println!(
            "After update_range - v: {}, tl: {}, tr: {}, l: {}, r: {}, value: {}, tree: {:?}, lazy: {:?}",
            v, tl, tr, l, r, value, self.tree, self.lazy
        );

         */
        
    }

    pub fn push(&mut self, v: usize){
        self.push_update(v)
    }
    fn push_update(&mut self, v: usize){

        self.tree[v*2] = min(self.tree[v*2], self.lazy[v]);
        self.lazy[v*2] = min(self.lazy[v*2], self.lazy[v]);
        self.tree[v*2+1] = min(self.tree[v*2+1], self.lazy[v]);
        self.lazy[v*2+1] = min(self.lazy[v*2+1], self.lazy[v]);
        self.lazy[v] = self.size+1;
    }

    pub fn query(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize ) -> usize {
        self.query_max(v, tl, tr, l, r)
    }
    fn query_max(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> usize {
        if l > r {
            return 0;
        }
        if l== tl && tr == r {
            return self.tree[v];
        }
        self.push(v);
        let tm = (tl + tr) / 2;
       return max(self.query_max(v * 2, tl, tm, l, min(r, tm)),
            self.query_max(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r),
        );

    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum PointKind {
    Begin,
    End,
}

pub struct SegmentTreeSegments{
    pub(crate) size: usize,
    pub(crate) tree: Vec<usize>
}

impl SegmentTreeSegments {
    pub fn new_segment_tree_s(&mut self, n: usize){
        self.size = n;
        self.tree = vec![0; n*4]
    }

    pub fn build(&mut self, array: &Vec<usize>, v:usize, tl:usize, tr:usize){
        if tl == tr {
            self.tree[v] = array[tl];
        } else {
            let tm = (tl + tr) / 2;
            self.build(array, v*2, tl, tm);
            self.build(array, v*2 + 1, tm + 1, tr);
            self.tree[v] = max(self.tree[v*2],self.tree[v*2 + 1]);
        }
    }

    pub fn is_there(&self, i: usize, j: usize, k: usize) -> usize {
        self.is_there_recursive(1, 0, self.size - 1, i, j, k)
    }

    fn is_there_recursive(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize, k: usize) -> usize {
        if l > r {
            return 0;
        }

        if l == tl && r == tr {
            return if self.tree[v] == k { 1 } else { 0 };
        }

        let tm = (tl + tr) / 2;
        let left_count = self.is_there_recursive(v * 2, tl, tm, l, min(r, tm), k);
        let right_count = self.is_there_recursive(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r, k);

        left_count + right_count
    }
}
pub fn create_array(intervals: &[(usize, usize)]) -> Vec<usize> {
    let mut pairs: Vec<_> = intervals
        .iter()
        .flat_map(|&(b, e)| [(b, PointKind::Begin), (e, PointKind::End)])
        .collect();

    pairs.sort_unstable();

    let mut counter = 0;
    let mut point_segments: HashMap<usize, usize> = HashMap::new();
    let mut result_array: Vec<usize> = Vec::new();

    for (point, kind) in pairs {
        match kind {
            PointKind::Begin => {
                counter += 1;
            }
            PointKind::End => {
                counter -= 1;
            }
        }
        point_segments.insert(point, counter);
    }

    for i in 0..intervals.len() {
        result_array.push(point_segments[&intervals[i].0]);
    }

    return result_array
}