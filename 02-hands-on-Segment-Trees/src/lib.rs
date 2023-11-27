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
    pub size: usize,
    pub tree: Vec<usize>
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum EventType {
    Begin,
    End,
}
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
struct Event {
    num: usize,
    event_type: EventType,
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

    pub fn is_there(&mut self, v: usize, tl: usize, tr: usize, i: usize, j :usize, k: usize) -> usize {
        if i > j {
            return 0;
        }

        if i == tl && j == tr {
            if self.tree[v]  == k {
                return 1;
            } else if tl != tr {
                let tm :usize = (tl + tr) / 2;
                return max(self.is_there(v*2, tl, tm, i, min(j, tm), k), self.is_there(v*2+1, tm+1, tr, max(i,tm+1), j, k));
            } else {
                return 0;
            }
        }
        let tm : usize =(tl +tr)/2;
        return max(self.is_there(v*2, tl, tm, i, min(j, tm), k), self.is_there(v*2+1, tm+1, tr, max(i,tm+1), j, k));
    }
}

pub fn sweep_line(intervals: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut events: Vec<Event> = Vec::new();
    let mut array: Vec<usize> = Vec::new();

    for &(begin, end) in intervals {
        events.push(Event { num: begin, event_type: EventType::Begin });
        events.push(Event { num: end, event_type: EventType::End });
    }

    events.sort_unstable();

    let max_x = events.iter().map(|event| event.num).max().unwrap();
    array = vec![0; (max_x + 1)];

    for i in 0..=max_x {
        if i != 0 {
            array[i] += array[(i - 1)];
        }

        for event in &events {
            match event.event_type {
                EventType::Begin => {
                    if event.num == i {
                        array[i] += 1;
                    }
                }
                EventType::End => {
                    if event.num == i && array[i] > 0 {
                        array[i] -= 1;
                    }
                }
            }
        }
    }

    return array;
}