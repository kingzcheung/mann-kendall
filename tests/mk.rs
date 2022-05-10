
#[cfg(test)]
mod tests {
    use std::vec;

    use mann_kendall::mann_kendall::{VecExt, unique_and_counts};
    use mann_kendall::mann_kendall::test as mann_kendall_test;
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
