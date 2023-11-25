use std::cmp::{max, min};

pub struct SegmentTree{
    pub tree: Vec<i32>,
    pub lazy: Vec<i32>
}

impl SegmentTree{
    pub fn build_segment_tree(&mut self, array: Vec<i32>, len_array: i32, v: i32, tl: i32, tr: i32){
        self.tree= vec![0; (len_array * 4) as usize];
        self.lazy= vec![0; (len_array * 4) as usize];
        self.build(&array, v, tl, tr);
    }

    fn build(&mut self, array: &Vec<i32>, v: i32, tl: i32, tr: i32){
        if tl == tr {
            self.tree[v as usize] = array[tl as usize]
        } else {
            let tm: i32 = (tl + tr) / 2;
            self.build(array, v*2, tl, tm);
            self.build(array, v*2+1, tm+1, tr);
            self.tree[v as usize] = max(self.tree[(v*2) as usize] , self.tree[(v*2 +1) as usize]);
        }
    }

    pub fn update(&mut self, v: i32, tl: i32, tr: i32, l: i32, r: i32, value: i32){
        self.update_range(v, tl, tr, l, r, value);
    }
    fn update_range(&mut self, v: i32, tl: i32, tr: i32, l: i32, r: i32, value:i32){
        if l > r {
            return;
        }
        if l == tl && tr == r {
            self.tree[v as usize] += min(self.tree[v as usize], value);
            self.lazy[v as usize] += min(self.lazy[v as usize], value);
        } else {
            self.push(v);
            let tm = (tl + tr) / 2;
            self.update( v*2, tl, tm, l, min(r, tm), value);
            self.update(v*2 +1, tm+1, tr, max(l, tm+1), r, value);
            self.tree[v as usize] = max(self.tree[(v*2) as usize], self.tree[(v*2+1) as usize]);
        }
    }

    pub fn push(&mut self, v: i32){
        self.push_update(v)
    }
    fn push_update(&mut self, v: i32){
        self.tree[(v*2) as usize] = min(self.tree[(v*2) as usize],self.lazy[v as usize]);
        self.lazy[(v*2) as usize] = min( self.lazy[(v*2) as usize],self.lazy[v as usize]);
        self.tree[(v*2+1) as usize] = min( self.tree[(v*2+1) as usize],self.lazy[v as usize] );
        self.lazy[(v*2+1) as usize] = min(self.lazy[(v*2+1) as usize], self.lazy[v as usize]);
        self.lazy[v as usize] = 0;
    }

    pub fn query(&mut self, v: i32, tl: i32, tr: i32, l: i32, r: i32 ) -> i32 {
        self.query_max(v, tl, tr, l, r)
    }
    fn query_max(&mut self, v: i32, tl: i32, tr: i32, l: i32, r: i32) -> i32 {
        if l > r {
            return 0;
        }
        if l== tl && tr == r {
            return self.tree[v as usize];
        }
        //self.push(v);
        let tm = (tl + tr) / 2;
        return max(self.query_max(self.tree[(v*2) as usize], tl, tm, l, min(r, tm)),
                   self.query_max(self.tree[(v*2+1) as usize], tm+1, tr, max(l, tm+1), r))

    }
}