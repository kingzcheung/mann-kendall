use std::ops::{Add, Sub, Mul};
use distrs::Normal;
use libc::size_t;
use std::slice;

#[derive(Debug,PartialEq, PartialOrd)]
struct VecExt(Vec<i32>);

impl VecExt {
    pub fn sum(&self) ->i32 {
        self.0.iter().sum()
    }
}


impl Mul<i32> for VecExt {
    type Output = VecExt;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut v = Vec::new();
        
        for i in self.0 {
            v.push(i * rhs)
        }

        VecExt(v)
    }
}

impl Mul<VecExt> for VecExt {
    type Output = VecExt;

    /// VecExt * VecExt, 实现a与b 数组相同索引相乘,必须要保持a与b的大小一致
    /// example:
    /// [3,2] * [2,1] = [6,2]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut v = Vec::new();
        for i in 0..self.0.len() {
            v.push(self.0[i] * rhs.0[i])
        }

        VecExt(v)
    }
}

/// 实现减法
/// example:
/// > VecExt(vec!([1f32,2f32])) - 1.0
impl Sub<i32> for VecExt {
    type Output = VecExt;

    fn sub(self, rhs: i32) -> Self::Output {
        let mut v = Vec::new();
        for i in self.0 {
            v.push(i - rhs)
        }

        VecExt(v)
    }
}

impl Sub<VecExt> for usize {
    type Output = VecExt;

    fn sub(self, rhs: VecExt) -> Self::Output {
        let mut v = Vec::new();
        for i in rhs.0 {
            v.push(self as i32 - i)
        }

        VecExt(v)
    }
}

/// 实现加法
/// example:
/// > VecExt(vec!([1f32,2f32])) + 1.0
impl Add<i32> for VecExt {
    type Output = VecExt;

    fn add(self, rhs: i32) -> Self::Output {
        let mut v = Vec::new();
        for i in self.0 {
            v.push(i + rhs)
        }

        VecExt(v)
    }
}

fn sign(x: f32) -> i32 {
    if x > 0f32 {
        1
    } else if x == 0f32 {
        0
    } else {
        -1
    }
}

/// unique_and_counts 返回vec 的唯一，还会返回每个值出现的次数
/// 默认为升序
fn unique_and_counts(mut x: Vec<f32>) -> (Vec<f32>, Vec<i32>) {
    let mut uniquex: Vec<f32> = Vec::new();
    let mut counts: Vec<i32> = Vec::new();

    // 以下代码应该可以合并在一个迭代实现,这里先分开

    // 排序
    x.sort_by(|a,b| a.partial_cmp(b).unwrap());


    for k in x.iter() {
        if uniquex.contains(k) {
            // 如果存在重复,增加计数
            continue;
        } else {
            uniquex.push(*k);
        }
    }

    // 计数
    for a in uniquex.iter() {
        let mut idx = 0;
        for b in x.iter() {
            if a == b {
                idx += 1
            }
        }
        counts.push(idx);
        // idx = 0;
    }


    (uniquex, counts)
}

#[repr(C)]
#[derive(Debug)]
pub struct Trend {
    pub norm_stat: f64,
    pub is_present: bool
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn mann_kendall_a(ptr: *mut f32, len: size_t  ,alpha:f64) ->Trend{
    let len = len as usize;
    let v = slice::from_raw_parts_mut(ptr, len);
    // let v = Vec::from_raw_parts(ptr, len, len);
    let v = v.to_vec();
    let (z,h) = mann_kendall_test(v, alpha);

    Trend { norm_stat: z, is_present: h}
    
}

/// # Safety
/// 曼－肯德尔趋势检验
/// 
#[no_mangle]
pub unsafe extern "C" fn mann_kendall(ptr: *mut f32, len: size_t  ,alpha:f64) ->f64{
    let len = len as usize;
    let v = slice::from_raw_parts_mut(ptr, len);
    // let v = Vec::from_raw_parts(ptr, len, len);
    let v = v.to_vec();
    let (r,_) = mann_kendall_test(v, alpha);

    r
    
}



/// 返回元组 (z,h)
/// 
/// z: 归一化检验统计,检验统计量
/// h: True（如果趋势存在）或 False（如果趋势不存在）
pub fn mann_kendall_test(x: Vec<f32>,alpha:f64) ->(f64,bool) {
    
    let n = x.len();
    let mut s = 0;

    for k in 0..n {
        for j in k + 1..n {
            s += sign(x[j] - x[k])
        }
    }
    

    let (unique_x,tp) = unique_and_counts(x);
    let g = unique_x.len();
    
    let var_s = if n == g {
        (n*(n-1)*(2*n+5))/18
    }else {
        // 0 as usize
        // (n*(n-1)*(2*n+5) - np.sum(tp*(tp-1)*(2*tp+5)))/18

        (
            (n*(n-1)*(2*n+5) - ( VecExt(tp.clone()) * (VecExt(tp.clone()) - 1))  * (VecExt(tp) * 2 + 5) ).sum() / 18
        ) as usize
    };

    let z = match s.cmp(&0) {
        std::cmp::Ordering::Less => (s+1) as f64 / ((var_s as f64).sqrt()),
        std::cmp::Ordering::Equal => 0.0,
        std::cmp::Ordering::Greater => (s - 1) as f64 / ( (var_s as f64).sqrt() ),
    };
    
    //  h = abs(z) > norm.ppf(1-alpha/2)
    let h = z.abs() > ppf(1f64 - alpha / 2f64);
    println!(">>> {}",  Normal::pdf(0.975, 0f64, 0f64));
    (z,h)
}

/// Percent point function 分位点函数/标准偏差乘数
fn ppf(x: f64) ->f64{
    Normal::pdf(x, 0f64, 1f64)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{unique_and_counts, VecExt, mann_kendall_test};

    #[test]
    fn test_unique_and_counts() {
        let data: Vec<f32> = vec![5., 1., 1., 2., 3., 4.];
        let uni_result: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let counts_result: Vec<i32> = vec![2, 1, 1, 1, 1];
        let (uni, counts) = unique_and_counts(data);
        assert_eq!(uni,uni_result);
        assert_eq!(counts,counts_result);
        println!("{:#?}", VecExt(vec![1,2]) * VecExt(vec![2,2]))

    }


    #[test]
    fn test_vec_ext_add(){
        let first = VecExt(vec![1,2]);
        let second = 2;
        let result = VecExt(vec![3,4]);
        let r = first + second;

        assert_eq!(result, r)
    }

    #[test]
    fn test_vec_ext_mul(){
        let first = VecExt(vec![1,2]);
        let second = VecExt(vec![3,2]);
        let result = VecExt(vec![3,4]);

        assert_eq!(result, first * second)
    }

    #[test]
    fn test_vec_ext_sum(){
        let data = VecExt(vec![1,3,5]);
        assert_eq!(9,data.sum())
    }

    #[test]
    fn test_mann_kendall_test(){
        let x = vec![1.,2.,3.,4.,5.,6.];
        let x = vec![6.,5.,4.,3.,2.,1.];
        let x = vec![1.,1.,1.,1.,1.,1.,1.,1.,1.];
        let res = mann_kendall_test(x, 0.05);
        assert!((mann_kendall_test(vec![1.,1.,1.,1.,1.,1.,1.,1.,1.], 0.05).0 == 0f64));
        assert!((mann_kendall_test(vec![1.,2.,3.,4.,5.,6.], 0.05).0 > 0f64));
        assert!((mann_kendall_test(vec![6.,5.,4.,3.,2.,1.], 0.05).0 < 0f64));
        println!("{}",res.0);
        println!("{}",res.1);
    }
}
